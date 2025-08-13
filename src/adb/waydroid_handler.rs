use std::{
    fmt::Display,
    io::{BufRead, Write},
    path::Path,
    process::Stdio,
};

use adb_client::DeviceShort;

use crate::{
    adb::adb_handler::{AdbHandler, gen_temp_path},
    ext_source::ExternalSaveSource,
};

#[derive(Debug, Default)]
struct WaydroidHandler {
    adb: AdbHandler,
}

pub async fn is_waydroid_installed() -> bool {
    if cfg!(feature = "wasm") {
        false
    } else {
        std::thread::spawn(|| {
            std::process::Command::new("waydroid")
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok_and(|s| s.success())
        })
        .join()
        .unwrap_or(false)
    }
}

#[derive(Debug)]
pub enum WaydroidError {
    Io(std::io::Error),
    ErrorStatus(String),
    Adb(adb_client::RustADBError),
    PermissionDenied,
}

impl Display for WaydroidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WaydroidError::Io(error) => error.to_string(),
                WaydroidError::ErrorStatus(e) => e.to_string(),
                WaydroidError::Adb(rust_adberror) => rust_adberror.to_string(),
                WaydroidError::PermissionDenied => "permission denied".to_string(),
            }
        )
    }
}

fn escape_command(cmd: Vec<String>) -> String {
    cmd.iter()
        .map(|arg| {
            let escaped = arg.replace("'", "'\\''");
            format!("'{}'", escaped)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

impl WaydroidHandler {
    pub async fn run_command(&self, cmds: &[&str]) -> Result<Vec<u8>, WaydroidError> {
        self.run_commands(&[cmds]).await
    }
    pub async fn run_commands(&self, cmds: &[&[&str]]) -> Result<Vec<u8>, WaydroidError> {
        let cmds: Vec<Vec<String>> = cmds
            .into_iter()
            .map(|cmd| cmd.into_iter().map(|v| v.to_string()).collect())
            .collect();
        std::thread::spawn(move || {
            let mut command = std::process::Command::new("pkexec")
                .arg("waydroid")
                .arg("shell")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(WaydroidError::Io)?;

            if let Some(mut stdin) = command.stdin.take() {
                for cmd in cmds {
                    stdin
                        .write_all(escape_command(cmd).as_bytes())
                        .map_err(WaydroidError::Io)?;
                    stdin.write_all(";".as_bytes()).map_err(WaydroidError::Io)?;
                }
            }

            let output = command.wait_with_output().map_err(WaydroidError::Io)?;

            if output.status.success() {
                Ok(output.stdout)
            } else {
                Err(WaydroidError::ErrorStatus(
                    String::from_utf8_lossy(&output.stderr).to_string(),
                ))
            }
        })
        .join()
        .map_err(|_| WaydroidError::Io(std::io::Error::other("failed to join")))?
    }

    pub async fn push_file(&mut self, input: Vec<u8>, path: &Path) -> Result<(), WaydroidError> {
        let temp_path = gen_temp_path(path);

        self.adb
            .adb_push_file(&mut std::io::Cursor::new(input), &temp_path)
            .map_err(WaydroidError::Adb)?;

        // reduces 1 pkexec call
        self.run_commands(&[
            &["mv", &temp_path.to_string_lossy(), &path.to_string_lossy()],
            &["chmod", "664", &path.to_string_lossy()],
        ])
        .await?;

        Ok(())
    }

    pub async fn pull_file(&mut self, path: &Path) -> Result<Vec<u8>, WaydroidError> {
        let temp_path = gen_temp_path(path);
        self.run_command(&["cp", &path.to_string_lossy(), &temp_path.to_string_lossy()])
            .await?;

        let mut output = std::io::Cursor::new(Vec::new());

        self.adb
            .adb_pull_file(&temp_path, &mut output)
            .map_err(WaydroidError::Adb)?;

        Ok(output.into_inner())
    }

    pub async fn close_program(&mut self, pkg: &str) -> Result<(), WaydroidError> {
        self.run_command(&["am", "force-stop", pkg]).await?;

        Ok(())
    }

    pub async fn run_program(&mut self, pkg: &str) -> Result<(), WaydroidError> {
        self.run_command(&["monkey", "--pct-syskeys", "0", "-p", pkg, "1"])
            .await?;

        Ok(())
    }

    pub async fn rerun_program(&mut self, pkg: &str) -> Result<(), WaydroidError> {
        // reduces 1 pkexec call
        self.run_commands(&[
            &["am", "force-stop", pkg],
            &["monkey", "--pct-syskeys", "0", "-p", pkg, "1"],
        ])
        .await?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct WaydroidGameHandler {
    handler: WaydroidHandler,
}

impl WaydroidGameHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_selected_device(&mut self, device: DeviceShort) {
        self.handler.adb.set_device(device);
    }
}

impl ExternalSaveSource for WaydroidGameHandler {
    type Error = WaydroidError;
    async fn write_path(&mut self, data: Vec<u8>, path: &Path) -> Result<(), Self::Error> {
        self.handler.push_file(data, path).await
    }

    async fn read_path(&mut self, path: &Path) -> Result<Vec<u8>, Self::Error> {
        self.handler.pull_file(path).await
    }

    async fn close_game(&mut self, pkg: &str) -> Result<(), Self::Error> {
        self.handler.close_program(pkg).await
    }
    async fn run_game(&mut self, pkg: &str) -> Result<(), Self::Error> {
        self.handler.run_program(pkg).await
    }
    async fn rerun_game(&mut self, pkg: &str) -> Result<(), Self::Error> {
        self.handler.rerun_program(pkg).await
    }
    async fn get_all_game_packages(&mut self) -> Result<Vec<String>, Self::Error> {
        let res = self
            .handler
            .run_command(&[
                "find",
                "/data/data/",
                "-name",
                "SAVE_DATA",
                "-mindepth",
                "3",
                "-maxdepth",
                "3",
            ])
            .await?;

        let mut packages = Vec::new();

        for line in res.lines() {
            let line = line.map_err(WaydroidError::Io)?;
            let mut parts = line.split("/");

            let package = parts.nth(3);
            if let Some(package) = package {
                if package.trim_start_matches(": ") == "Permission denied" {
                    return Err(WaydroidError::PermissionDenied);
                }
                packages.push(package.to_string());
            }
        }

        Ok(packages)
    }
}

use std::{
    fmt::Display,
    io::Write,
    path::{Path, PathBuf},
};

// use adb_client::{ADBDeviceExt, ADBServer, DeviceShort};

use crate::ext_source::ExternalSaveSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdbDevice {
    pub id: String,
    pub state: String,
}

#[derive(Debug)]
pub struct AdbHandler {
    pub adb_path: PathBuf,
    pub selected_device: Option<AdbDevice>,
}

pub fn find_adb_path() -> Option<PathBuf> {
    let possible_paths: Vec<PathBuf> = vec![
        Path::new("/opt")
            .join("android-sdk")
            .join("platform-tools")
            .join("adb"),
    ];
    for path in possible_paths {
        if std::fs::exists(&path).ok()? {
            return Some(path);
        }
    }
    None
}

pub async fn is_adb_installed(adb_path: Option<PathBuf>) -> bool {
    std::thread::spawn(|| {
        std::process::Command::new(adb_path.unwrap_or(PathBuf::from("adb")))
            .arg("version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
    })
    .join()
    .unwrap_or(false)
}

#[derive(Debug)]
pub enum AdbError {
    JoinThread,
    AdbRun(std::io::Error),
    NotSuccess(Vec<String>, String, String),
    DecodeString(std::string::FromUtf8Error),
    NoDeviceSelected,
    CreateTempFile(std::io::Error),
    ReadData(std::io::Error),
    WriteData(std::io::Error),
    PermissionDenied,
    CantFindAdb,
}

impl Display for AdbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AdbError::JoinThread => "failed to join thread".to_string(),
                AdbError::AdbRun(error) => format!("failed to run adb command: {error}"),
                AdbError::NotSuccess(args, stderr, stdout) => format!(
                    "failed to run command: {}, stdout: {stdout}, stderr: {stderr}",
                    args.join(" ")
                ),
                AdbError::DecodeString(e) => format!("failed to decode utf8 string: {e}"),
                AdbError::NoDeviceSelected => "no devices selected".to_string(),
                AdbError::CreateTempFile(error) => format!("failed to create temp file: {error}"),
                AdbError::ReadData(error) => format!("failed to read data: {error}"),
                AdbError::WriteData(error) => format!("failed to write data: {error}"),
                AdbError::PermissionDenied => format!("permission denied, do you have root?"),
                AdbError::CantFindAdb => format!("failed to find adb binary"),
            }
        )
    }
}

pub fn escape_command(cmd: &[&str]) -> String {
    cmd.iter()
        .map(|arg| {
            let escaped = arg.replace("'", "'\\''");
            format!("'{}'", escaped)
        })
        .collect::<Vec<String>>()
        .join(" ")
}

impl AdbHandler {
    pub fn new(adb_path: PathBuf) -> Self {
        Self {
            adb_path,
            selected_device: None,
        }
    }

    pub async fn run_adb_command(&self, cmd: &[&str]) -> Result<String, AdbError> {
        let adb_path = self.adb_path.clone();
        let cmd: Vec<String> = cmd.iter().map(|v| v.to_string()).collect();
        let selected_device = self.selected_device().cloned();
        std::thread::spawn(move || {
            let mut command = std::process::Command::new(adb_path);
            if let Some(dev) = selected_device {
                command.arg("-s").arg(dev.id);
            }
            let command = command.args(&cmd);
            let output = command.output().map_err(|e| AdbError::AdbRun(e))?;

            if !output.status.success() {
                return Err(AdbError::NotSuccess(
                    command
                        .get_args()
                        .map(|v| v.to_string_lossy().to_string())
                        .collect(),
                    String::from_utf8_lossy(&output.stderr).into_owned(),
                    String::from_utf8_lossy(&output.stdout).into_owned(),
                ));
            }

            let output2: String =
                String::from_utf8(output.stdout).map_err(|e| AdbError::DecodeString(e))?;

            Ok(output2)
        })
        .join()
        .map_err(|_| AdbError::JoinThread)?
    }

    pub async fn adb_devices(&self) -> Result<Vec<AdbDevice>, AdbError> {
        let output = self.run_adb_command(&["devices"]).await?;

        let mut devices = Vec::new();

        for line in output.lines().skip(1) {
            let device = line.split_once("\t");

            if let Some((device_id, status)) = device {
                devices.push(AdbDevice {
                    id: device_id.to_string(),
                    state: status.to_string(),
                })
            }
        }

        Ok(devices)
    }

    pub fn with_device(mut self, device: AdbDevice) -> Self {
        self.selected_device = Some(device);

        self
    }
    pub fn selected_device_mut(&mut self) -> Option<&mut AdbDevice> {
        self.selected_device.as_mut()
    }

    pub fn set_selected_device(&mut self, device: AdbDevice) {
        self.selected_device = Some(device);
    }

    pub fn selected_device(&self) -> Option<&AdbDevice> {
        self.selected_device.as_ref()
    }
    pub fn selected_device_or_err(&self) -> Result<&AdbDevice, AdbError> {
        self.selected_device
            .as_ref()
            .ok_or(AdbError::NoDeviceSelected)
    }

    pub async fn run_shell_command(&mut self, cmd: &[&str]) -> Result<String, AdbError> {
        let escaped_args = escape_command(cmd);

        self.run_adb_command(&["shell", &escaped_args]).await
    }

    pub async fn adb_push_file<R: std::io::Read>(
        &mut self,
        input: &mut R,
        path: &Path,
    ) -> Result<(), AdbError> {
        let mut temp_path =
            tempfile::NamedTempFile::new().map_err(|e| AdbError::CreateTempFile(e))?;
        let mut data = Vec::new();
        input
            .read_to_end(&mut data)
            .map_err(|e| AdbError::ReadData(e))?;

        temp_path
            .write_all(&data)
            .map_err(|e| AdbError::WriteData(e))?;
        self.run_adb_command(&[
            "push",
            &temp_path.into_temp_path().to_string_lossy(),
            &path.to_string_lossy(),
        ])
        .await?;

        Ok(())
    }

    pub async fn push_file<R: std::io::Read>(
        &mut self,
        input: &mut R,
        path: &Path,
    ) -> Result<(), AdbError> {
        let temp_path = gen_temp_path(path);
        self.adb_push_file(input, &temp_path).await?;

        if let Err(e) = self
            .run_shell_command(&["mv", &temp_path.to_string_lossy(), &path.to_string_lossy()])
            .await
        {
            self.run_shell_command(&["rm", &temp_path.to_string_lossy()])
                .await?;
            return Err(e);
        };
        // might not be needed
        self.run_shell_command(&["chmod", "664", &path.to_string_lossy()])
            .await?;

        Ok(())
    }

    pub async fn adb_pull_file<W: std::io::Write>(
        &mut self,
        path: &Path,
        output: &mut W,
    ) -> Result<(), AdbError> {
        let temp_path = tempfile::NamedTempFile::new().map_err(|e| AdbError::CreateTempFile(e))?;

        self.run_adb_command(&[
            "pull",
            &path.to_string_lossy(),
            &temp_path.path().to_string_lossy(),
        ])
        .await?;

        let data = std::fs::read(temp_path.path()).map_err(AdbError::ReadData)?;

        output.write_all(&data).map_err(AdbError::WriteData)?;

        Ok(())
    }

    pub async fn pull_file<W: std::io::Write>(
        &mut self,
        path: &Path,
        output: &mut W,
    ) -> Result<(), AdbError> {
        let temp_path = gen_temp_path(path);
        self.run_shell_command(&["cp", &path.to_string_lossy(), &temp_path.to_string_lossy()])
            .await?;

        let res = self.adb_pull_file(&temp_path, output).await;
        self.run_shell_command(&["rm", &temp_path.to_string_lossy()])
            .await?;

        res
    }

    pub async fn close_program(&mut self, pkg: &str) -> Result<(), AdbError> {
        self.run_shell_command(&["am", "force-stop", pkg]).await?;

        Ok(())
    }

    pub async fn run_program(&mut self, pkg: &str) -> Result<(), AdbError> {
        self.run_shell_command(&["monkey", "--pct-syskeys", "0", "-p", pkg, "1"])
            .await?;

        Ok(())
    }
}

pub fn gen_temp_path(path: &Path) -> PathBuf {
    PathBuf::from("/data").join("local").join("tmp").join(
        path.file_name()
            .map(|v| v.to_string_lossy().to_string())
            .unwrap_or("temp.txt".to_string()),
    )
}

#[derive(Debug)]
pub struct AdbGameHandler {
    handler: AdbHandler,
}

impl AdbGameHandler {
    pub fn new(adb_path: Option<PathBuf>) -> Option<Self> {
        Some(Self {
            handler: AdbHandler::new(adb_path.unwrap_or(find_adb_path()?)),
        })
    }

    pub async fn adb_devices(&self) -> Result<Vec<AdbDevice>, AdbError> {
        self.handler.adb_devices().await
    }

    pub fn selected_device_mut(&mut self) -> Option<&mut AdbDevice> {
        self.handler.selected_device_mut()
    }
    pub fn set_selected_device(&mut self, device: AdbDevice) {
        self.handler.set_selected_device(device);
    }
}

impl ExternalSaveSource for AdbGameHandler {
    type Error = AdbError;
    async fn read_path(&mut self, path: &Path) -> Result<Vec<u8>, Self::Error> {
        let mut writer = std::io::Cursor::new(Vec::new());
        self.handler.pull_file(path, &mut writer).await?;
        Ok(writer.into_inner())
    }
    async fn write_path(&mut self, data: Vec<u8>, path: &Path) -> Result<(), Self::Error> {
        self.handler
            .push_file(&mut std::io::Cursor::new(data), path)
            .await
    }

    async fn close_game(&mut self, pkg: &str) -> Result<(), Self::Error> {
        self.handler.close_program(pkg).await
    }

    async fn run_game(&mut self, pkg: &str) -> Result<(), Self::Error> {
        self.handler.run_program(pkg).await
    }
    async fn all_game_packages(&mut self) -> Result<Vec<String>, Self::Error> {
        let res = self
            .handler
            .run_shell_command(&[
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
            let mut parts = line.split("/");

            let package = parts.nth(3);
            if let Some(package) = package {
                if package.trim_start_matches(": ") == "Permission denied" {
                    return Err(AdbError::PermissionDenied);
                }
                packages.push(package.to_string());
            }
        }

        Ok(packages)
    }
}

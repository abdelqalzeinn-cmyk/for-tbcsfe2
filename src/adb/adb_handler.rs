use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

use adb_client::{ADBDeviceExt, DeviceShort};

use crate::ext_source::ExternalSaveSource;

#[derive(Debug, Default)]
pub struct AdbHandler {
    pub server: adb_client::ADBServer,
    pub selected_device: Option<DeviceShort>,
}

impl AdbHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_devices(&mut self) -> Result<Vec<DeviceShort>, adb_client::RustADBError> {
        self.server.devices()
    }

    pub fn with_device(mut self, device: DeviceShort) -> Self {
        self.selected_device = Some(device);

        self
    }
    pub fn set_device(&mut self, device: DeviceShort) {
        self.selected_device = Some(device);
    }

    pub fn get_selected_device(&self) -> Option<&DeviceShort> {
        self.selected_device.as_ref()
    }

    pub fn get_selected_device_or_default(
        &mut self,
    ) -> Result<adb_client::ADBServerDevice, adb_client::RustADBError> {
        if let Some(sel) = self.get_selected_device() {
            Ok(adb_client::ADBServerDevice::new(
                sel.identifier.clone(),
                None,
            ))
        } else {
            self.server.get_device()
        }
    }

    pub async fn run_command(&mut self, cmd: &[&str]) -> Result<Vec<u8>, adb_client::RustADBError> {
        let mut device = self.get_selected_device_or_default()?;
        let cmd: Vec<String> = cmd.into_iter().map(|v| v.to_string()).collect();
        std::thread::spawn(move || {
            let mut writer = std::io::Cursor::new(Vec::new());

            let string_slices: &[&str] = &cmd.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
            device.shell_command(string_slices, &mut writer)?;

            Ok(writer.into_inner())
        })
        .join()
        .map_err(|_| adb_client::RustADBError::IOError(std::io::Error::other("failed to join")))?
    }

    pub fn adb_push_file<R: std::io::Read>(
        &mut self,
        input: &mut R,
        path: &Path,
    ) -> Result<(), adb_client::RustADBError> {
        self.get_selected_device_or_default()?
            .push(input, path.to_string_lossy())
    }

    pub async fn push_file<R: std::io::Read>(
        &mut self,
        input: &mut R,
        path: &Path,
    ) -> Result<(), adb_client::RustADBError> {
        let temp_path = gen_temp_path(path);
        self.adb_push_file(input, &temp_path)?;

        if let Err(e) = self
            .run_command(&["mv", &temp_path.to_string_lossy(), &path.to_string_lossy()])
            .await
        {
            self.run_command(&["rm", &temp_path.to_string_lossy()])
                .await?;
            return Err(e);
        };
        // might not be needed
        self.run_command(&["chmod", "664", &path.to_string_lossy()])
            .await?;

        Ok(())
    }

    pub fn adb_pull_file<W: std::io::Write>(
        &mut self,
        path: &Path,
        output: &mut W,
    ) -> Result<(), adb_client::RustADBError> {
        self.get_selected_device_or_default()?
            .pull(&path.to_string_lossy(), output)
    }

    pub async fn pull_file<W: std::io::Write>(
        &mut self,
        path: &Path,
        output: &mut W,
    ) -> Result<(), adb_client::RustADBError> {
        let temp_path = gen_temp_path(path);
        self.run_command(&["cp", &path.to_string_lossy(), &temp_path.to_string_lossy()])
            .await?;

        let res = self.adb_pull_file(&temp_path, output);
        self.run_command(&["rm", &temp_path.to_string_lossy()])
            .await?;

        res
    }

    pub async fn close_program(&mut self, pkg: &str) -> Result<(), adb_client::RustADBError> {
        self.run_command(&["am", "force-stop", pkg]).await?;

        Ok(())
    }

    pub async fn run_program(&mut self, pkg: &str) -> Result<(), adb_client::RustADBError> {
        self.run_command(&["monkey", "--pct-syskeys", "0", "-p", pkg, "1"])
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

#[derive(Debug, Default)]
pub struct AdbGameHandler {
    handler: AdbHandler,
}

impl AdbGameHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_devices(&mut self) -> Result<Vec<DeviceShort>, adb_client::RustADBError> {
        self.handler.get_devices()
    }

    pub fn set_selected_device(&mut self, device: DeviceShort) {
        self.handler.set_device(device);
    }
}

impl ExternalSaveSource for AdbGameHandler {
    type Error = adb_client::RustADBError;
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
            let line = line.map_err(adb_client::RustADBError::IOError)?;
            let mut parts = line.split("/");

            let package = parts.nth(3);
            if let Some(package) = package {
                if package.trim_start_matches(": ") == "Permission denied" {
                    return Err(adb_client::RustADBError::IOError(std::io::Error::other(
                        "permission denied",
                    )));
                }
                packages.push(package.to_string());
            }
        }

        Ok(packages)
    }
}

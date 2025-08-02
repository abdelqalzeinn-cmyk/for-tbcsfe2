use std::{
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct AssetManager {
    base_path: PathBuf,
}

impl AssetManager {
    pub fn new(base_path: Option<&Path>) -> Result<Self, std::io::Error> {
        let path = if let Some(base_path) = base_path {
            base_path.to_path_buf()
        } else {
            std::env::current_exe()
                .map_err(|e| {
                    std::io::Error::other(format!("failed to get path of executable, {e}"))
                })?
                .join("assets")
        };

        if !std::fs::exists(&path)? {
            return Err(std::io::Error::other(format!(
                "failed to find assets path: {}",
                path.to_string_lossy()
            )));
        }

        Ok(Self { base_path: path })
    }

    pub fn new_from_exe_path() -> Result<Self, std::io::Error> {
        Self::new(None)
    }

    pub fn new_from_base_path(base_path: &Path) -> Result<Self, std::io::Error> {
        Self::new(Some(base_path))
    }

    fn get_asset_path(&self, path: &Path) -> PathBuf {
        self.base_path.join(path)
    }

    pub fn open_asset(&self, path: &Path) -> Result<std::fs::File, std::io::Error> {
        let path = self.get_asset_path(path);
        std::fs::File::open(&path).map_err(|e| {
            std::io::Error::other(format!("error for path: {}, {e}", path.to_string_lossy()))
        })
    }

    pub fn read_asset(&self, path: &Path) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = Vec::new();
        self.open_asset(path)?.read_to_end(&mut buf)?;

        Ok(buf)
    }

    pub fn read_asset_str(&self, path: &Path) -> Result<String, std::io::Error> {
        String::from_utf8(self.read_asset(path)?).map_err(std::io::Error::other)
    }
}

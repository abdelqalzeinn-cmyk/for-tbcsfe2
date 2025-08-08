use std::{
    fmt::Display,
    io::{Cursor, Read, Write},
    path::Path,
};

use zip::{
    HasZipMetadata,
    result::ZipError,
    write::{FileOptions, SimpleFileOptions},
};

use crate::{
    country_code::CountryCode, network::password::ManagedItem, save::SaveFile, stream::StreamError,
};

#[derive(Debug)]
pub enum AccountInfoError {
    Io(std::io::Error),
    SerdeJson(serde_json::Error),
}

impl Display for AccountInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccountInfoError::Io(e) => format!("failed to read to string: {e}"),
                AccountInfoError::SerdeJson(error) => format!("json error: {error}"),
            }
        )
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GameAccountInfo {
    pub password: Option<String>,
    #[serde(rename = "token")]
    pub auth_token: Option<String>,
}
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EditorAccountInfo {
    pub account_info: GameAccountInfo,
    pub managed_items: Vec<ManagedItem>,
}

impl EditorAccountInfo {
    pub fn new(account_info: GameAccountInfo, managed_items: Vec<ManagedItem>) -> Self {
        Self {
            account_info,
            managed_items,
        }
    }

    pub fn read_from_string(data: &str) -> Result<Self, AccountInfoError> {
        serde_json::from_str(data).map_err(|e| AccountInfoError::SerdeJson(e))
    }

    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, AccountInfoError> {
        let mut str = "".to_string();
        reader
            .read_to_string(&mut str)
            .map_err(AccountInfoError::Io)?;

        Self::read_from_string(&str)
    }

    pub fn to_string(&self) -> Result<String, AccountInfoError> {
        serde_json::to_string(self).map_err(|e| AccountInfoError::SerdeJson(e))
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), AccountInfoError> {
        writer
            .write_all(self.to_string()?.as_bytes())
            .map_err(AccountInfoError::Io)
    }
}

impl GameAccountInfo {
    pub fn new(password: Option<String>, auth_token: Option<String>) -> Self {
        Self {
            password,
            auth_token,
        }
    }

    pub fn read_from_string(data: &str) -> Result<Self, AccountInfoError> {
        serde_json::from_str(data).map_err(|e| AccountInfoError::SerdeJson(e))
    }

    pub fn from_data(data: &[u8]) -> Result<Self, AccountInfoError> {
        Self::from_reader(&mut std::io::Cursor::new(data))
    }
    pub fn to_data(&self) -> Result<Vec<u8>, AccountInfoError> {
        Ok(self.to_string()?.into_bytes())
    }

    pub fn from_reader<R: std::io::Read>(reader: &mut R) -> Result<Self, AccountInfoError> {
        let mut str = "".to_string();
        reader
            .read_to_string(&mut str)
            .map_err(AccountInfoError::Io)?;

        Self::read_from_string(&str)
    }

    pub fn to_string(&self) -> Result<String, AccountInfoError> {
        serde_json::to_string(self).map_err(|e| AccountInfoError::SerdeJson(e))
    }

    pub fn write<W: std::io::Write>(&self, writer: &mut W) -> Result<(), AccountInfoError> {
        writer
            .write_all(self.to_string()?.as_bytes())
            .map_err(AccountInfoError::Io)
    }
}

#[derive(Debug)]
pub enum SaveFileZipError {
    ZipError(ZipError),
    Io(std::io::Error),
    AccountInfo(AccountInfoError),
    Save(StreamError),
}

impl std::error::Error for SaveFileZipError {}

impl Display for SaveFileZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SaveFileZipError::ZipError(zip_error) => format!("zip error: {zip_error}"),
                SaveFileZipError::Io(error) => format!("io error: {error}"),
                SaveFileZipError::AccountInfo(account_info_error) =>
                    format!("account info error: {account_info_error}"),
                SaveFileZipError::Save(stream_error) => format!("stream error: {stream_error}"),
            }
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct SaveFileAccount {
    pub save_file: SaveFile,
    pub account_info: EditorAccountInfo,
}

impl SaveFileAccount {
    pub fn load_from_path(path: &Path, cc: Option<CountryCode>) -> Result<Self, SaveFileZipError> {
        if path.extension().is_some_and(|e| e == "zip") {
            Self::load_from_zip_file(path, cc)
        } else {
            Ok(Self {
                save_file: SaveFile::load_from_path(path, cc).map_err(SaveFileZipError::Save)?,
                account_info: EditorAccountInfo::default(),
            })
        }
    }
    pub fn load_from_zip_file(
        path: &Path,
        cc: Option<CountryCode>,
    ) -> Result<Self, SaveFileZipError> {
        Self::load_from_zip_reader(
            &mut std::fs::File::open(path).map_err(SaveFileZipError::Io)?,
            cc,
        )
    }
    pub fn load_from_zip_data(
        data: &[u8],
        cc: Option<CountryCode>,
    ) -> Result<Self, SaveFileZipError> {
        Self::load_from_zip_reader(&mut std::io::Cursor::new(data), cc)
    }

    pub fn load_from_zip_reader<R: std::io::Seek + std::io::Read>(
        reader: &mut R,
        cc: Option<CountryCode>,
    ) -> Result<Self, SaveFileZipError> {
        let mut zip_reader = zip::ZipArchive::new(reader).map_err(SaveFileZipError::ZipError)?;

        let account_info = {
            let mut account_info_z = zip_reader
                .by_name("save_account.json")
                .map_err(SaveFileZipError::ZipError)?;

            let account_info = EditorAccountInfo::from_reader(&mut account_info_z)
                .map_err(SaveFileZipError::AccountInfo)?;

            account_info
        };

        let mut save_data_z = zip_reader
            .by_name("SAVE_DATA")
            .map_err(SaveFileZipError::ZipError)?;

        let mut save_data =
            Vec::with_capacity(save_data_z.get_metadata().uncompressed_size as usize);

        save_data_z
            .read_to_end(&mut save_data)
            .map_err(SaveFileZipError::Io)?;

        let save_file = if let Some(cc) = cc {
            SaveFile::load_cc(&save_data, cc).map_err(SaveFileZipError::Save)?
        } else {
            SaveFile::load_detect_cc(&save_data).map_err(SaveFileZipError::Save)?
        };

        Ok(Self {
            save_file,
            account_info: account_info,
        })
    }

    pub fn write_to_zip_data(&self) -> Result<Vec<u8>, SaveFileZipError> {
        let savedata = self
            .save_file
            .write_with_hash()
            .map_err(SaveFileZipError::Save)?;

        let inner = Cursor::new(Vec::new());

        let mut writer = zip::ZipWriter::new(inner);

        let opts: SimpleFileOptions = FileOptions::default();

        writer
            .start_file("SAVE_DATA", opts)
            .map_err(SaveFileZipError::ZipError)?;
        writer.write_all(&savedata).map_err(SaveFileZipError::Io)?;

        writer
            .start_file("save_account.json", opts)
            .map_err(SaveFileZipError::ZipError)?;

        let data = self
            .account_info
            .to_string()
            .map_err(SaveFileZipError::AccountInfo)?;

        writer
            .write_all(data.as_bytes())
            .map_err(SaveFileZipError::Io)?;

        Ok(writer
            .finish()
            .map_err(SaveFileZipError::ZipError)?
            .into_inner())
    }

    pub fn write_to_path(&self, path: &Path) -> Result<(), SaveFileZipError> {
        let path = path.with_extension("zip");
        let data = self.write_to_zip_data()?;

        std::fs::write(path, &data).map_err(SaveFileZipError::Io)
    }
}

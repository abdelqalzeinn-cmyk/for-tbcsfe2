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

    pub fn add_managed_item(&mut self, item: ManagedItem) {
        self.managed_items.push(item);
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

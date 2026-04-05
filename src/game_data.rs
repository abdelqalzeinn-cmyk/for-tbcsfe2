use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::{country_code::CountryCode, game_version::GameVersion};

#[derive(Debug, Clone, Deserialize)]
pub struct GameRepoMetadata {
    pub base_url: String,
    pub versions: HashMap<CountryCode, HashMap<GameVersion, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum GameRepoError {
    #[error("failed to fetch metadata: {0}, at url: {1}")]
    GetRequest(reqwest::Error, String),
    #[error("failed to parse metadata: {0}, at url: {1}")]
    GetJson(reqwest::Error, String),
    #[error("failed to get data: {0}, at url: {1}")]
    GetData(reqwest::Error, String),
    #[error("no versions found for country code: {0}")]
    NoCountryCode(CountryCode),
    #[error("failed to extract tar.xz archive to {1} due to: {0}")]
    ExtractArchive(std::io::Error, PathBuf),
    #[error("failed to read file at {1} due to {0}")]
    ReadFile(std::io::Error, PathBuf),
    #[error("failed to create file at: {1} due to {0}")]
    CreateFile(std::io::Error, PathBuf),
    #[error("failed to find file: {1} in pack: {0}")]
    FileNotFound(String, String),
}

type Archive<'a> = tar::Archive<xz2::read::XzDecoder<std::io::Cursor<&'a [u8]>>>;

impl GameRepoMetadata {
    pub async fn fetch_meta(url: &str) -> Result<Self, GameRepoError> {
        reqwest::get(url)
            .await
            .map_err(|e| GameRepoError::GetRequest(e, url.to_string()))?
            .json()
            .await
            .map_err(|e| GameRepoError::GetJson(e, url.to_string()))
    }

    pub fn find_closest_meta_gv(
        &self,
        country_code: CountryCode,
        game_version: GameVersion,
    ) -> Result<(GameVersion, &str), GameRepoError> {
        let versions = self
            .versions
            .get(&country_code)
            .ok_or_else(|| GameRepoError::NoCountryCode(country_code))?;

        if let Some(val) = versions.get(&game_version) {
            return Ok((game_version, val));
        }

        let versions_ls = versions.iter().map(|v| *v.0).collect();
        let version = Self::find_closest_gv(versions_ls, game_version)
            .ok_or_else(|| GameRepoError::NoCountryCode(country_code))?;

        Ok((
            version,
            versions
                .get(&version)
                .ok_or_else(|| GameRepoError::NoCountryCode(country_code))?,
        ))
    }
    pub fn find_closest_gv(
        mut versions: Vec<GameVersion>,
        game_version: GameVersion,
    ) -> Option<GameVersion> {
        versions.sort();

        let mut index = versions.partition_point(|v| *v < game_version);

        if index == versions.len() {
            index = versions.len() - 1;
        }

        versions.get(index).copied()
    }

    pub async fn fetch_data(&self, path: &str) -> Result<Vec<u8>, GameRepoError> {
        let mut url = self.base_url.to_string();
        url.push_str(path);
        reqwest::get(&url)
            .await
            .map_err(|e| GameRepoError::GetRequest(e, url.to_string()))?
            .bytes()
            .await
            .map_err(|e| GameRepoError::GetData(e, url.to_string()))
            .map(|v| v.to_vec())
    }

    pub fn open_archive<'a>(data: &'a [u8]) -> Archive<'a> {
        let reader = xz2::read::XzDecoder::new(std::io::Cursor::new(data));
        let archive = tar::Archive::new(reader);

        archive
    }

    pub fn extract_archive(archive: &mut Archive, outdir: &Path) -> Result<(), GameRepoError> {
        archive
            .unpack(outdir)
            .map_err(|e| GameRepoError::ExtractArchive(e, outdir.to_path_buf()))?;

        std::fs::File::create(outdir.join("downloaded"))
            .map_err(|e| GameRepoError::CreateFile(e, outdir.join("downloaded")))?;

        Ok(())
    }

    pub fn get_version_path(
        game_data_dir: &Path,
        country_code: CountryCode,
        game_version: GameVersion,
    ) -> PathBuf {
        game_data_dir
            .join(country_code.to_string())
            .join(game_version.to_string())
    }

    pub async fn download_data(
        &self,
        country_code: CountryCode,
        game_version: GameVersion,
        game_data_dir: &Path,
    ) -> Result<GameVersion, GameRepoError> {
        let (gv, path) = self.find_closest_meta_gv(country_code, game_version)?;
        Self::extract_archive(
            &mut Self::open_archive(&self.fetch_data(path).await?),
            &Self::get_version_path(game_data_dir, country_code, game_version),
        )?;

        Ok(gv)
    }
}

#[derive(Debug, Clone)]
pub struct GameData {
    pub game_data_dir: PathBuf,
}

impl GameData {
    pub fn new(
        game_data_dir: &Path,
        country_code: CountryCode,
        game_version: GameVersion,
    ) -> GameData {
        Self {
            game_data_dir: GameRepoMetadata::get_version_path(
                game_data_dir,
                country_code,
                game_version,
            ),
        }
    }

    pub fn is_downloaded(&self) -> bool {
        let path = self.game_data_dir.join("downloaded");

        path.exists()
    }
    pub fn get_file_err(&self, pack_name: &str, file_name: &str) -> Result<Vec<u8>, GameRepoError> {
        self.get_file(pack_name, file_name)?.ok_or_else(|| {
            GameRepoError::FileNotFound(pack_name.to_string(), file_name.to_string())
        })
    }
    pub fn get_file_lang_err(
        &self,
        pack_name: &str,
        lang: &str,
        file_name: &str,
    ) -> Result<Vec<u8>, GameRepoError> {
        self.get_file_lang(pack_name, lang, file_name)?
            .ok_or_else(|| {
                GameRepoError::FileNotFound(pack_name.to_string(), file_name.to_string())
            })
    }

    pub fn get_file(
        &self,
        pack_name: &str,
        file_name: &str,
    ) -> Result<Option<Vec<u8>>, GameRepoError> {
        let path = self.game_data_dir.join(pack_name).join(file_name);

        if !path.exists() {
            return Ok(None);
        }

        std::fs::read(&path)
            .map(|v| Some(v))
            .map_err(|e| GameRepoError::ReadFile(e, path.to_path_buf()))
    }
    pub fn get_file_lang(
        &self,
        base_pack_name: &str,
        lang: &str,
        file_name: &str,
    ) -> Result<Option<Vec<u8>>, GameRepoError> {
        let packname = match lang {
            "en" | "ja" | "jp" | "kr" | "ko" | "tw" => base_pack_name.to_string(),
            _ => format!("{base_pack_name}_{lang}"),
        };
        let path = self.game_data_dir.join(packname).join(file_name);

        if !path.exists() {
            return Ok(None);
        }

        std::fs::read(&path)
            .map(|v| Some(v))
            .map_err(|e| GameRepoError::ReadFile(e, path.to_path_buf()))
    }
}

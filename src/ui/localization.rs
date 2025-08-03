use std::path::{Path, PathBuf};

use fluent::{FluentArgs, FluentBundle, FluentMessage, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::ui::asset::AssetManager;

pub struct LocaleManager {
    bundle: FluentBundle<FluentResource>,
    pub metadata: LocaleMetadata,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocaleMetadata {
    pub authors: Vec<String>,
    pub files: Vec<PathBuf>,
}

impl LocaleManager {
    fn read_metadata(
        locale: &LanguageIdentifier,
        asset_manager: &AssetManager,
    ) -> Result<LocaleMetadata, std::io::Error> {
        let locale_dir = Path::new("locales").join(&locale.to_string());
        let metadata_data = asset_manager.read_asset_str(&locale_dir.join("metadata.toml"))?;

        toml::from_str(&metadata_data).map_err(|e| std::io::Error::other(e))
    }
    fn read_res(
        locale: &LanguageIdentifier,
        asset_manager: &AssetManager,
        paths: &Vec<PathBuf>,
    ) -> Result<Vec<FluentResource>, std::io::Error> {
        let mut reses = Vec::with_capacity(paths.len());
        for file_path in paths {
            let str_data = asset_manager.read_asset_str(
                &Path::new("locales")
                    .join(&locale.to_string())
                    .join(file_path),
            )?;

            let res = FluentResource::try_new(str_data).map_err(|e| {
                std::io::Error::other(format!(
                    "{}: {}",
                    e.0.source(),
                    e.1.first().map(|v| v.to_string()).unwrap_or_default()
                ))
            })?;

            reses.push(res)
        }

        Ok(reses)
    }

    pub fn new_wasm(locale: LanguageIdentifier) -> Self {
        // FIXME: actually get the locales
        let mut bundle = FluentBundle::new(vec![locale]);

        let resource = FluentResource::try_new(
            r#"title = Battle Cats Save File Editor
load-save = Load Save
save-save = Save Save
catfood = Catfood
xp = XP
main-story = Main Story
save-path = Save Path
transfer-code = Transfer Code
confirmation-code = Confirmation Code
country-code = Country Code
load = Load
select-path = Select Path
apply = Apply

chapter_1 = Empire of Cats 1
chapter_2 = Empire of Cats 2
chapter_3 = Empire of Cats 3
chapter_4 = Into the Future 1
chapter_5 = Into the Future 2
chapter_6 = Into the Future 3
chapter_7 = Cats of the Cosmos 1
chapter_8 = Cats of the Cosmos 2
chapter_9 = Cats of the Cosmos 3

clear-all-selected-chapters = Clear All Selected Chapters
clear-count = Clear Count
toggle-select-all = Toggle Select All
"#
            .to_string(),
        )
        .unwrap();

        bundle.add_resource(resource).unwrap();

        Self {
            bundle,
            metadata: LocaleMetadata {
                authors: Vec::new(),
                files: Vec::new(),
            },
        }
    }

    pub fn new(
        locale: LanguageIdentifier,
        asset_manager: &AssetManager,
    ) -> Result<Self, std::io::Error> {
        let metadata = Self::read_metadata(&locale, asset_manager)?;
        let reses = Self::read_res(&locale, asset_manager, &metadata.files)?;
        let mut bundle = FluentBundle::new(vec![locale]);

        for res in reses {
            bundle.add_resource(res).map_err(|e| {
                std::io::Error::other(e.first().map(|v| v.to_string()).unwrap_or_default())
            })?;
        }

        Ok(Self { bundle, metadata })
    }

    fn get_msg<'a>(&'a self, id: &str) -> Option<FluentMessage<'a>> {
        self.bundle.get_message(id)
    }

    pub fn localize_optional_args_fallible(
        &self,
        id: &str,
        args: Option<&FluentArgs>,
    ) -> Result<String, std::io::Error> {
        let mut errors = Vec::new();
        if let Some(msg) = self.get_msg(id) {
            let pattern = msg.value().ok_or(std::io::Error::other(format!(
                "failed to get value for key: {id}"
            )))?;
            Ok(self
                .bundle
                .format_pattern(pattern, args, &mut errors)
                .into_owned())
        } else {
            Ok(id.to_string())
        }
    }

    pub fn localize_with_args_fallible(
        &self,
        id: &str,
        args: &FluentArgs,
    ) -> Result<String, std::io::Error> {
        self.localize_optional_args_fallible(id, Some(args))
    }

    pub fn localize_fallible(&self, id: &str) -> Result<String, std::io::Error> {
        self.localize_optional_args_fallible(id, None)
    }

    pub fn localize(&self, id: &str) -> String {
        self.localize_fallible(id).unwrap_or(id.to_string())
    }

    pub fn localize_with_args(&self, id: &str, args: &FluentArgs) -> String {
        self.localize_with_args_fallible(id, args)
            .unwrap_or(id.to_string())
    }

    pub fn localize_optional_args(&self, id: &str, args: Option<&FluentArgs>) -> String {
        self.localize_optional_args_fallible(id, args)
            .unwrap_or(id.to_string())
    }
}

pub trait Localizable: ToString {
    fn localize(&self, manager: &LocaleManager) -> String {
        manager.localize(&self.to_string())
    }

    fn localize_with_args(&self, manager: &LocaleManager, args: &FluentArgs) -> String {
        manager.localize_with_args(&self.to_string(), args)
    }
    fn localize_optional_args(&self, manager: &LocaleManager, args: Option<&FluentArgs>) -> String {
        manager.localize_optional_args(&self.to_string(), args)
    }
}

impl<T: ToString> Localizable for T {}

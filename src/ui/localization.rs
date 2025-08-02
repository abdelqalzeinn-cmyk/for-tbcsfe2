use std::path::Path;

use fluent::{FluentArgs, FluentBundle, FluentMessage, FluentResource};
use unic_langid::LanguageIdentifier;

use crate::ui::asset::AssetManager;

pub struct LocaleManager {
    bundle: FluentBundle<FluentResource>,
}

impl LocaleManager {
    fn read_res(
        locale: &LanguageIdentifier,
        asset_manager: &AssetManager,
    ) -> Result<FluentResource, std::io::Error> {
        let str_data = asset_manager.read_asset_str(
            &Path::new("locales")
                .join(&locale.to_string())
                .join("main.ftl"),
        )?;

        let res = FluentResource::try_new(str_data).map_err(|e| {
            std::io::Error::other(format!(
                "{}: {}",
                e.0.source(),
                e.1.first().map(|v| v.to_string()).unwrap_or_default()
            ))
        })?;

        Ok(res)
    }

    pub fn new(
        locale: LanguageIdentifier,
        asset_manager: &AssetManager,
    ) -> Result<Self, std::io::Error> {
        let res = Self::read_res(&locale, asset_manager)?;
        let mut bundle = FluentBundle::new(vec![locale]);

        bundle.add_resource(res).map_err(|e| {
            std::io::Error::other(e.first().map(|v| v.to_string()).unwrap_or_default())
        })?;

        Ok(Self { bundle })
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

pub mod adb;
pub mod app;
pub mod catfood;
pub mod config;
pub mod editview;
pub mod helper;
pub mod loadsave;
pub mod mainstory;
pub mod savesave;

pub fn get_project_dir() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("org", "fieryhenry", "bcsfe")
}

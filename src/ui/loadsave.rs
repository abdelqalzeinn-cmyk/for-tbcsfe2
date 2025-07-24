use std::path::{Path, PathBuf};

use iced::{
    Element, Length, Task, Theme,
    alignment::Vertical,
    widget::{container::bordered_box, text::LineHeight},
};

use crate::{save::SaveFile, ui::app::Message};

#[derive(Debug, Clone)]
pub enum LoadSaveMsg {
    SelectPath,
    LoadPath,
    OnSaveInput(String),
    SelectedPath(Option<PathBuf>),
}

impl LoadedSaveFile {
    pub fn view(&self, _theme: &Theme) -> Element<'_, Message> {
        let text = iced::widget::text("Save File: ");
        let item: Element<Message> = match &self.source {
            SaveSource::Path(path) => iced::widget::text(path.to_string_lossy()).into(),
        };

        iced::widget::row([text.into(), item]).into()
    }
}

#[derive(Debug, Clone)]
pub enum SaveSource {
    Path(PathBuf),
}

impl Default for SaveSource {
    fn default() -> Self {
        Self::Path(PathBuf::default())
    }
}

#[derive(Debug, Clone)]
pub struct LoadedSaveFile {
    pub source: SaveSource,
    pub save_file: SaveFile,
}

pub async fn select_save() -> Option<PathBuf> {
    Some(
        rfd::AsyncFileDialog::new()
            .pick_file()
            .await?
            .path()
            .to_path_buf(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LoadSave {
    pub save_path: String,
}

impl LoadSave {
    pub fn update(&mut self, message: LoadSaveMsg) -> Task<Message> {
        match message {
            LoadSaveMsg::SelectPath => {
                return Task::perform(select_save(), |p| {
                    Message::LoadSave(LoadSaveMsg::SelectedPath(p))
                });
            }
            LoadSaveMsg::LoadPath => match self.load_save_from_path() {
                Ok(s) => return Task::done(Message::LoadedSave(Box::new(s))),
                Err(e) => return Task::done(Message::Error(e.to_string())),
            },
            LoadSaveMsg::OnSaveInput(p) => self.save_path = p,
            LoadSaveMsg::SelectedPath(path_buf) => {
                if let Some(path) = path_buf {
                    self.save_path = path.to_string_lossy().to_string()
                }
            }
        };
        Task::none()
    }

    fn load_save_from_path(&self) -> Result<LoadedSaveFile, Box<dyn std::error::Error>> {
        let save_file = SaveFile::load_from_path_detect_cc(Path::new(self.save_path.as_str()))?;
        let path = std::fs::canonicalize(Path::new(&self.save_path))?;
        Ok(LoadedSaveFile {
            source: SaveSource::Path(path),
            save_file,
        })
    }
    pub fn view(&self) -> Element<'_, Message> {
        let save_path_layout: Element<Message> = iced::widget::container(
            iced::widget::row([
                iced::widget::text("Save Path:")
                    .align_y(Vertical::Center)
                    .height(Length::Fixed(30.0))
                    .into(),
                iced::widget::button("Select Path")
                    .on_press(Message::LoadSave(LoadSaveMsg::SelectPath))
                    .into(),
                iced::widget::container(
                    iced::widget::text_input("Save Path", &self.save_path)
                        .size(15)
                        .line_height(LineHeight::Relative(1.5))
                        .on_input(|p| Message::LoadSave(LoadSaveMsg::OnSaveInput(p))),
                )
                .align_y(Vertical::Center)
                .into(),
                iced::widget::button("Load")
                    .on_press_maybe(match std::fs::exists(&self.save_path) {
                        Ok(exists) => match exists {
                            true => Some(Message::LoadSave(LoadSaveMsg::LoadPath)),
                            false => None,
                        },
                        Err(_) => None,
                    })
                    .into(),
            ])
            .spacing(10),
        )
        .padding(10)
        .style(bordered_box)
        .into();
        iced::widget::container(iced::widget::column([save_path_layout])).into()
    }
}

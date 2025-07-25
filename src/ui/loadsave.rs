use std::path::{Path, PathBuf};

use iced::{
    Element, Length, Task, Theme,
    alignment::Vertical,
    widget::{container::bordered_box, text::LineHeight},
};

use crate::{
    country_code::CountryCode, network::transfers::from_codes, save::SaveFile, ui::app::Message,
};

#[derive(Debug, Clone)]
pub enum LoadSaveMsg {
    SelectPath,
    FromCodes,
    LoadPath,
    OnSaveInput(String),
    SelectedPath(Option<PathBuf>),
    LoadedCodes(crate::network::transfers::FromCodesResponse),
    OnTransferInput(String),
    OnConfirmationInput(String),
    SelectCC(CountryCode),
}

impl LoadedSaveFile {
    pub fn view(&self, _theme: &Theme) -> Element<'_, Message> {
        let text = iced::widget::text("Save File: ");
        let item: Element<Message> = match &self.source {
            SaveSource::Path(path) => iced::widget::text(path.to_string_lossy()).into(),
            SaveSource::TransferCodes => iced::widget::text("transfer codes!!!").into(),
        };

        iced::widget::row([text.into(), item]).into()
    }
}

#[derive(Debug, Clone)]
pub enum SaveSource {
    Path(PathBuf),
    TransferCodes,
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
    pub transfer_code: String,
    pub confirmation_code: String,
    pub selected_cc: Option<CountryCode>,
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
            LoadSaveMsg::FromCodes => {
                return Task::done(Message::Notif("downloading save...".to_string())).chain(
                    Task::perform(
                        from_codes(
                            self.transfer_code.clone(),
                            self.confirmation_code.clone(),
                            self.selected_cc.unwrap_or_default(),
                            crate::game_version::GameVersion(140500),
                            None,
                            None,
                        ),
                        |r| match r {
                            Ok(v) => Message::LoadSave(LoadSaveMsg::LoadedCodes(v)),
                            Err(e) => Message::Error(e.to_string()),
                        },
                    ),
                );
            }
            LoadSaveMsg::LoadedCodes(r) => {
                let save_file = SaveFile::load_detect_cc(&r.save_data);
                match save_file {
                    Ok(s) => {
                        return Task::done(Message::LoadedSave(Box::new(LoadedSaveFile {
                            source: SaveSource::TransferCodes,
                            save_file: s,
                        })));
                    }
                    Err(e) => return Task::done(Message::Error(e.to_string())),
                }
            }
            LoadSaveMsg::OnTransferInput(t) => self.transfer_code = t,
            LoadSaveMsg::OnConfirmationInput(c) => self.confirmation_code = c,
            LoadSaveMsg::SelectCC(country_code) => self.selected_cc = Some(country_code),
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
        let save_path_layout = self.view_save_path_layout();
        let transfer_code_layout = self.view_transfer_code_layout();
        iced::widget::container(
            iced::widget::column([save_path_layout, transfer_code_layout]).spacing(10),
        )
        .into()
    }

    fn view_transfer_code_layout(&self) -> Element<'_, Message> {
        let transfer_code_layout = iced::widget::row([
            iced::widget::text("Transfer Code:")
                .align_y(Vertical::Center)
                .height(Length::Fixed(30.0))
                .into(),
            iced::widget::text_input("Transfer Code", &self.transfer_code)
                .size(15)
                .line_height(LineHeight::Relative(1.5))
                .on_input(|t| Message::LoadSave(LoadSaveMsg::OnTransferInput(t)))
                .into(),
            iced::widget::text("Confirmation Code:")
                .align_y(Vertical::Center)
                .height(Length::Fixed(30.0))
                .into(),
            iced::widget::text_input("Confirmation Code", &self.confirmation_code)
                .size(15)
                .line_height(LineHeight::Relative(1.5))
                .on_input(|c| Message::LoadSave(LoadSaveMsg::OnConfirmationInput(c)))
                .into(),
            iced::widget::text("Country Code:")
                .align_y(Vertical::Center)
                .height(Length::Fixed(30.0))
                .into(),
            iced::widget::pick_list(CountryCode::ALL, self.selected_cc, |c| {
                Message::LoadSave(LoadSaveMsg::SelectCC(c))
            })
            .into(),
            iced::widget::button("Load")
                .on_press_maybe(
                    if self.transfer_code.is_empty()
                        || self.confirmation_code.is_empty()
                        || self.selected_cc.is_none()
                    {
                        None
                    } else {
                        Some(Message::LoadSave(LoadSaveMsg::FromCodes))
                    },
                )
                .into(),
        ])
        .spacing(10);

        iced::widget::container(transfer_code_layout)
            .padding(10)
            .style(bordered_box)
            .into()
    }

    fn view_save_path_layout(&self) -> Element<'_, Message> {
        let select_save_layout = iced::widget::row([
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
        .spacing(10);
        let save_path_layout: Element<Message> = iced::widget::container(select_save_layout)
            .padding(10)
            .style(bordered_box)
            .into();
        save_path_layout
    }
}

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use iced::{
    Element, Length, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::text::LineHeight,
};

use crate::{
    country_code::CountryCode,
    localization::{LocaleManager, Localizable},
    network::{
        account_info::{EditorAccountInfo, GameAccountInfo, SaveFileAccount},
        password::TransferCodes,
        transfer::from_codes,
    },
    save::SaveFile,
    ui::{
        adb::{AdbMessage, AdbView},
        app::Message,
        helper::labeled_box,
    },
};

#[derive(Debug, Clone)]
pub enum LoadSaveMsg {
    SelectPath,
    FromCodes,
    LoadPath,
    LoadData,
    OnSaveInput(String),
    SelectedData(Option<Vec<u8>>),
    LoadedCodes(crate::network::transfer::FromCodesResponse),
    OnTransferInput(String),
    OnConfirmationInput(String),
    SelectCC(CountryCode),
    Adb(AdbMessage),
    PulledAccountInfo(SaveFile, GameAccountInfo, String),
}

impl LoadedSaveFile {
    pub fn view(&self, theme: &Theme, locale_manager: &LocaleManager) -> Element<'_, Message> {
        let text = "loaded-save-file".localize(locale_manager);
        let first_text = iced::widget::text(text);

        let inquiry_code = iced::widget::text(
            self.save_file
                .save_file
                .save
                .get_inquiry_code_with_default("no-inquiry-code".localize(locale_manager)),
        );

        let game_version = iced::widget::text(self.save_file.save_file.gvcc.gv.to_string());

        let country_code = iced::widget::text(
            LocalizedCC::from_cc(self.save_file.save_file.gvcc.cc, locale_manager).1,
        );

        let info_row = iced::widget::row([
            first_text.into(),
            inquiry_code.color(theme.palette().primary).into(),
            iced::widget::text("loaded-save-file-cc-splitter".localize(locale_manager)).into(),
            country_code.color(theme.palette().primary).into(),
            iced::widget::text("loaded-save-file-gv-splitter".localize(locale_manager)).into(),
            game_version.color(theme.palette().primary).into(),
        ])
        .spacing(5)
        .into();

        let mut rows = vec![info_row];

        if let Some(ref codes) = self.codes {
            let codes_row = iced::widget::row([
                iced::widget::row([
                    iced::widget::text("transfer-code-colon".localize(locale_manager)).into(),
                    iced::widget::text(&codes.transfer_code)
                        .color(theme.palette().primary)
                        .into(),
                ])
                .spacing(5)
                .into(),
                iced::widget::text("transfer-code-splitter".localize(locale_manager)).into(),
                iced::widget::row([
                    iced::widget::text("confirmation-code-colon".localize(locale_manager)).into(),
                    iced::widget::text(&codes.confirmation_code)
                        .color(theme.palette().primary)
                        .into(),
                ])
                .spacing(5)
                .into(),
            ])
            .spacing(10)
            .into();

            rows.push(codes_row)
        }

        iced::widget::column(rows).spacing(10).into()
    }
}

#[derive(Debug, Clone)]
pub enum SaveSource {
    Path(PathBuf),
    Data,
    TransferCodes,
    Adb(String),
}

impl Default for SaveSource {
    fn default() -> Self {
        Self::Path(PathBuf::default())
    }
}

#[derive(Debug, Clone)]
pub struct LoadedSaveFile {
    pub source: SaveSource,
    pub save_file: SaveFileAccount,
    pub codes: Option<TransferCodes>,
}

pub async fn select_save() -> Option<Vec<u8>> {
    Some(rfd::AsyncFileDialog::new().pick_file().await?.read().await)
}

#[derive(Debug, Clone)]
pub struct LoadSave {
    pub save_path: String,
    pub save_data: Option<Vec<u8>>,
    pub transfer_code: String,
    pub confirmation_code: String,
    pub selected_cc: Option<CountryCode>,
    pub adb: AdbView,
}

impl LoadSave {
    pub async fn new() -> Self {
        Self {
            save_path: String::default(),
            save_data: None,
            transfer_code: String::default(),
            confirmation_code: String::default(),
            selected_cc: Some(CountryCode::En),
            adb: AdbView::new(super::adb::AdbDirection::LoadSave),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LocalizedCC(CountryCode, String);

impl PartialEq for LocalizedCC {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Display for LocalizedCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl LocalizedCC {
    pub(crate) fn from_cc(cc: CountryCode, locale_manager: &LocaleManager) -> Self {
        Self(cc, format!("cc-{cc}").localize(locale_manager))
    }
    fn all(locale_manager: &LocaleManager) -> [LocalizedCC; 4] {
        [
            Self::from_cc(CountryCode::En, locale_manager),
            Self::from_cc(CountryCode::Jp, locale_manager),
            Self::from_cc(CountryCode::Kr, locale_manager),
            Self::from_cc(CountryCode::Tw, locale_manager),
        ]
    }
}

impl LoadSave {
    pub fn update(
        &mut self,
        message: LoadSaveMsg,
        locale_manager: &LocaleManager,
    ) -> Task<Message> {
        match message {
            LoadSaveMsg::SelectPath => {
                return Task::perform(select_save(), |d| {
                    Message::LoadSave(LoadSaveMsg::SelectedData(d))
                });
            }
            LoadSaveMsg::LoadPath => match self.load_save_from_path() {
                Ok(s) => return Task::done(Message::LoadedSave(Box::new(s))),
                Err(e) => return Task::done(Message::Error(e.to_string())),
            },
            LoadSaveMsg::LoadData => match self.load_save_from_data() {
                Ok(s) => return Task::done(Message::LoadedSave(Box::new(s))),
                Err(e) => return Task::done(Message::Error(e.to_string())),
            },
            LoadSaveMsg::OnSaveInput(p) => self.save_path = p,
            LoadSaveMsg::SelectedData(data) => {
                if let Some(data) = data {
                    self.save_data = Some(data);

                    return Task::done(Message::LoadSave(LoadSaveMsg::LoadData));
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
                    Ok(mut s) => {
                        if let Some(prt) = r.password_refresh_token {
                            s.save.set_password_refresh_token(prt);
                        }

                        let save_file = SaveFileAccount {
                            save_file: s,
                            account_info: EditorAccountInfo::new(r.account_info, Vec::new()),
                        };
                        return Task::done(Message::LoadedSave(Box::new(LoadedSaveFile {
                            source: SaveSource::TransferCodes,
                            save_file,
                            codes: Some(TransferCodes {
                                transfer_code: self.transfer_code.clone(),
                                confirmation_code: self.confirmation_code.clone(),
                            }),
                        })));
                    }
                    Err(e) => return Task::done(Message::Error(e.to_string())),
                }
            }
            LoadSaveMsg::OnTransferInput(t) => self.transfer_code = t,
            LoadSaveMsg::OnConfirmationInput(c) => self.confirmation_code = c,
            LoadSaveMsg::SelectCC(country_code) => self.selected_cc = Some(country_code),
            LoadSaveMsg::Adb(adb_message) if self.adb.adb_installed.is_none_or(|i| i) => {
                if let AdbMessage::Error(e) = adb_message {
                    return Task::done(Message::Error(e));
                }
                if let AdbMessage::PulledAccountInfo(s, info) = adb_message {
                    return Task::done(Message::LoadSave(LoadSaveMsg::PulledAccountInfo(
                        s,
                        info,
                        self.adb.selected_pkg.clone().unwrap_or_default(),
                    )));
                }
                if let AdbMessage::LoadSave(data, pkg) = adb_message {
                    let save = SaveFile::load_detect_cc(&data);
                    match save {
                        Ok(s) => {
                            let iq = s.save.get_inquiry_code().map(|v| v.to_string());
                            if let Some(iq) = iq {
                                return Task::done(Message::LoadSave(LoadSaveMsg::Adb(
                                    AdbMessage::PullAccountInfo(s, iq),
                                )));
                            } else {
                                return Task::done(Message::LoadSave(
                                    LoadSaveMsg::PulledAccountInfo(
                                        s,
                                        GameAccountInfo::default(),
                                        pkg,
                                    ),
                                ));
                            }
                        }
                        Err(e) => return Task::done(Message::Error(e.to_string())),
                    }
                }
                return self
                    .adb
                    .update(adb_message, locale_manager)
                    .map(|m| Message::LoadSave(LoadSaveMsg::Adb(m)));
            }
            LoadSaveMsg::Adb(_) => {}
            LoadSaveMsg::PulledAccountInfo(save_file, game_account_info, pkg) => {
                let save_file = SaveFileAccount {
                    save_file,
                    account_info: EditorAccountInfo {
                        account_info: game_account_info,
                        managed_items: Vec::new(),
                    },
                };
                return Task::done(Message::LoadedSave(Box::new(LoadedSaveFile {
                    source: SaveSource::Adb(pkg),
                    save_file,
                    codes: None,
                })));
            }
        };
        Task::none()
    }

    fn load_save_from_path(&self) -> Result<LoadedSaveFile, Box<dyn std::error::Error>> {
        let path = Path::new(self.save_path.as_str());
        let save_file = SaveFileAccount::load_from_path(path, None)?;
        let path = std::fs::canonicalize(Path::new(&self.save_path))?;
        Ok(LoadedSaveFile {
            source: SaveSource::Path(path),
            save_file,
            codes: None,
        })
    }
    fn load_save_from_data(&self) -> Result<LoadedSaveFile, Box<dyn std::error::Error>> {
        let save_data = self
            .save_data
            .as_ref()
            .ok_or(std::io::Error::other("no save data"))?;
        let is_zip = zip::ZipArchive::new(std::io::Cursor::new(save_data)).is_ok();
        let save_file = if is_zip {
            SaveFileAccount::load_from_zip_data(save_data, None)?
        } else {
            SaveFileAccount {
                save_file: SaveFile::load_detect_cc(&save_data)?,
                account_info: EditorAccountInfo::default(),
            }
        };
        Ok(LoadedSaveFile {
            source: SaveSource::Data,
            save_file,
            codes: None,
        })
    }
    pub fn view(&self, theme: &Theme, locale_manager: &LocaleManager) -> Element<'_, Message> {
        let transfer_code_layout = self.view_transfer_code_layout(theme, locale_manager);
        let open_save_btn = labeled_box(
            theme,
            "load-save-dialog".localize(locale_manager),
            iced::widget::button(
                iced::widget::text("load-save-system".localize(locale_manager))
                    .width(Length::Fill)
                    .align_x(Horizontal::Center),
            )
            .width(Length::Fill)
            .on_press(Message::LoadSave(LoadSaveMsg::SelectPath))
            .into(),
        );
        let mut cols = Vec::new();
        cols.push(open_save_btn);

        #[cfg(not(feature = "wasm"))]
        cols.push(self.view_save_path_layout(theme, locale_manager));

        cols.push(transfer_code_layout);

        if self.adb.adb_installed.is_none_or(|i| i) {
            cols.push(
                self.adb
                    .view(theme, locale_manager)
                    .map(|m| Message::LoadSave(LoadSaveMsg::Adb(m))),
            );
        }

        iced::widget::container(iced::widget::column(cols).spacing(10)).into()
    }

    fn view_transfer_code_layout(
        &self,
        theme: &Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        let transfer_code_layout = iced::widget::row([
            iced::widget::text("transfer-code".localize(locale_manager))
                .align_y(Vertical::Center)
                .height(Length::Fill)
                .into(),
            iced::widget::text_input(
                &"transfer-code".localize(locale_manager),
                &self.transfer_code,
            )
            .line_height(LineHeight::Relative(1.5))
            .on_input(|t| Message::LoadSave(LoadSaveMsg::OnTransferInput(t)))
            .into(),
            iced::widget::text("confirmation-code".localize(locale_manager))
                .align_y(Vertical::Center)
                .height(Length::Fill)
                .into(),
            iced::widget::text_input(
                &"confirmation-code".localize(locale_manager),
                &self.confirmation_code,
            )
            .line_height(LineHeight::Relative(1.5))
            .on_input(|c| Message::LoadSave(LoadSaveMsg::OnConfirmationInput(c)))
            .into(),
            iced::widget::text("country-code".localize(locale_manager))
                .align_y(Vertical::Center)
                .height(Length::Fill)
                .into(),
            iced::widget::pick_list(
                LocalizedCC::all(locale_manager),
                self.selected_cc
                    .map(|v| LocalizedCC::from_cc(v, locale_manager)),
                |c| Message::LoadSave(LoadSaveMsg::SelectCC(c.0)),
            )
            .into(),
            iced::widget::button(iced::widget::text("load".localize(locale_manager)))
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
        .height(Length::Shrink)
        .spacing(10);

        labeled_box(
            theme,
            "load-save-from-codes".localize(locale_manager),
            transfer_code_layout.into(),
        )
    }

    #[cfg(not(feature = "wasm"))]
    fn view_save_path_layout(
        &self,
        theme: &Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        let select_save_layout = iced::widget::row([
            iced::widget::text("save-path".localize(locale_manager))
                .align_y(Vertical::Center)
                .height(Length::Fill)
                .into(),
            iced::widget::container(
                iced::widget::text_input(&"save-path".localize(locale_manager), &self.save_path)
                    .line_height(LineHeight::Relative(1.5))
                    .on_input(|p| Message::LoadSave(LoadSaveMsg::OnSaveInput(p))),
            )
            .align_y(Vertical::Center)
            .into(),
            iced::widget::button(iced::widget::text("load".localize(locale_manager)))
                .on_press_maybe(match std::fs::exists(&self.save_path) {
                    Ok(exists) => match exists {
                        true => Some(Message::LoadSave(LoadSaveMsg::LoadPath)),
                        false => None,
                    },
                    Err(_) => None,
                })
                .into(),
        ])
        .height(Length::Shrink)
        .spacing(10);
        labeled_box(
            theme,
            "load-save-from-path".localize(locale_manager),
            select_save_layout.into(),
        )
    }

    pub fn init(&mut self) -> Task<Message> {
        self.adb
            .init()
            .map(|m| Message::LoadSave(LoadSaveMsg::Adb(m)))
    }
}

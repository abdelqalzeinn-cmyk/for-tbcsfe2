use std::path::PathBuf;

use iced::{
    Alignment, Element, Length, Task,
    alignment::{Horizontal, Vertical},
    widget::{container::bordered_box, text::LineHeight},
};

use crate::{
    country_code::CountryCode,
    network::{
        account_info::{EditorAccountInfo, SaveFileAccount},
        password::{NewAccountInfo, TransferCodes, UploadInfo, upload_save},
    },
    ui::{
        adb::{AdbDirection, AdbMessage, AdbView},
        app::Message,
        helper::labeled_box,
        loadsave::{LoadedSaveFile, LocalizedCC, SaveSource},
        localization::{LocaleManager, Localizable},
    },
};

#[derive(Debug, Clone)]
pub enum SaveSaveMsg {
    SaveSystem,
    SavePath,
    OnSaveInput(String),
    UploadSave,
    Uploaded(TransferCodes, SaveFileAccount),
    DoneTransfer,
    Adb(crate::ui::adb::AdbMessage),
}

#[derive(Debug, Clone)]
pub struct SaveSave {
    pub save_path: String,
    pub cc: CountryCode,
    pub is_transferring: bool,
    pub codes: Option<TransferCodes>,
    pub adb: AdbView,
    pub save_data: Vec<u8>,
}

impl Default for SaveSave {
    fn default() -> Self {
        Self {
            save_path: "".to_string(),
            cc: CountryCode::En,
            is_transferring: false,
            codes: None,
            adb: AdbView::new(true, super::adb::AdbDirection::LoadSave),
            save_data: Vec::new(),
        }
    }
}

impl SaveSave {
    pub fn update(
        &mut self,
        message: SaveSaveMsg,
        save: &mut SaveFileAccount,
        locale_manager: &LocaleManager,
    ) -> Task<Message> {
        match message {
            SaveSaveMsg::SaveSystem => {
                let save_save = "saved-save".localize(locale_manager);
                let no_save_save = "no-save-save".localize(locale_manager);
                return Task::perform(Self::save_system(self.save_data.to_vec()), |r| match r {
                    Ok(o) => {
                        if o.is_some() {
                            Message::Notif(save_save)
                        } else {
                            Message::Notif(no_save_save)
                        }
                    }

                    Err(e) => Message::Error(e.to_string()),
                });
            }
            SaveSaveMsg::SavePath => match self.save_save_to_path() {
                Ok(s) => return Task::done(Message::SavedSave(s)),
                Err(e) => return Task::done(Message::Error(e.to_string())),
            },
            SaveSaveMsg::OnSaveInput(p) => self.save_path = p,
            SaveSaveMsg::UploadSave => {
                self.is_transferring = true;
                let save_info = UploadInfo::from_save(&save.save_file);
                let save_c = save.clone();
                let account_info = save.account_info.clone();
                return Task::done(Message::Notif("upload-start".localize(locale_manager)))
                    .chain(Task::perform(
                        async move { upload_save(save_c, save_info, account_info).await },
                        |r| match r {
                            Ok(codes) => Message::SaveSave(SaveSaveMsg::Uploaded(codes.0, codes.1)),
                            Err(e) => Message::Error(e.to_string()),
                        },
                    ))
                    .chain(Task::done(Message::SaveSave(SaveSaveMsg::DoneTransfer)));
            }
            SaveSaveMsg::Uploaded(codes, save_file_account) => {
                self.codes = Some(codes.clone());
                *save = save_file_account;

                return Task::done(Message::Notif(
                    "successfully-uploaded".localize(locale_manager),
                ))
                .chain(Task::done(Message::Codes(codes)));
            }
            SaveSaveMsg::DoneTransfer => self.is_transferring = false,
            SaveSaveMsg::Adb(adb_message) => {
                if let AdbMessage::SaveSave(_) = adb_message {
                    let iq = save.save_file.save.get_inquiry_code();
                    let mt = Task::done(Message::Notif("pushed-adb".localize(locale_manager)));
                    if let Some(iq) = iq {
                        return mt.chain(Task::done(Message::SaveSave(SaveSaveMsg::Adb(
                            AdbMessage::PushAccountInfo(
                                iq.to_string(),
                                save.account_info.account_info.clone(),
                            ),
                        ))));
                    }
                    return mt;
                }
                if matches!(adb_message, AdbMessage::PushedAccountInfo) {
                    return Task::done(Message::Notif("pushed-account".localize(locale_manager)));
                }
                if let AdbMessage::ReranGame(_) = adb_message {
                    return Task::done(Message::Notif("reran-game".localize(locale_manager)));
                }
                return self
                    .adb
                    .update(adb_message, locale_manager)
                    .map(|m| Message::SaveSave(SaveSaveMsg::Adb(m)));
            }
        };
        Task::none()
    }

    pub fn view(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        let save_system_layout = self.view_system_dialog_layout(theme, locale_manager);
        let save_path_layout = self.view_save_path_layout(theme, locale_manager);
        let upload_save_layout = self.view_upload_save_layout(theme, locale_manager);
        let mut col = vec![save_system_layout, save_path_layout, upload_save_layout];
        if let Some(ref codes) = self.codes {
            let save_codes_layout = self.view_save_codes_layout(theme, locale_manager, codes);

            col.push(save_codes_layout);
        }
        col.push(
            self.adb
                .view(theme, locale_manager)
                .map(|m| Message::SaveSave(SaveSaveMsg::Adb(m))),
        );
        iced::widget::container(iced::widget::column(col).spacing(10)).into()
    }

    fn view_save_codes_layout<'a>(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
        codes: &'a TransferCodes,
    ) -> Element<'a, Message> {
        let save_codes_layout = iced::widget::container(
            iced::widget::row([
                labeled_box(
                    theme,
                    "transfer-code".localize(locale_manager),
                    iced::widget::text(&codes.transfer_code).into(),
                ),
                labeled_box(
                    theme,
                    "confirmation-code".localize(locale_manager),
                    iced::widget::text(&codes.confirmation_code).into(),
                ),
                labeled_box(
                    theme,
                    "country-code".localize(locale_manager),
                    iced::widget::text(LocalizedCC::from_cc(self.cc, locale_manager).to_string())
                        .into(),
                ),
            ])
            .width(Length::Fill)
            .spacing(10),
        )
        .style(bordered_box)
        .padding(10)
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .into();
        save_codes_layout
    }

    fn view_upload_save_layout(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        labeled_box(
            theme,
            "upload-save-file".localize(locale_manager),
            iced::widget::button(
                iced::widget::text("upload-save".localize(locale_manager))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
            )
            .on_press_maybe(if self.is_transferring {
                None
            } else {
                Some(Message::SaveSave(SaveSaveMsg::UploadSave))
            })
            .width(Length::Fill)
            .into(),
        )
    }
    fn view_system_dialog_layout(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        labeled_box(
            theme,
            "write-save-file".localize(locale_manager),
            iced::widget::button(
                iced::widget::text("save-save-system-dialog".localize(locale_manager))
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
            )
            .on_press_maybe(if self.is_transferring {
                None
            } else {
                Some(Message::SaveSave(SaveSaveMsg::SaveSystem))
            })
            .width(Length::Fill)
            .into(),
        )
    }

    fn view_save_path_layout(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        let save_path_layout: Element<Message> = iced::widget::container(
            iced::widget::column([
                iced::widget::text("save-save-save-path".localize(locale_manager))
                    .color(theme.palette().primary)
                    .into(),
                iced::widget::row([
                    iced::widget::text("save-path".localize(locale_manager))
                        .align_y(Vertical::Center)
                        .height(Length::Fill)
                        .into(),
                    iced::widget::container(
                        iced::widget::text_input(
                            &"save-path".localize(locale_manager),
                            &self.save_path,
                        )
                        .line_height(LineHeight::Relative(1.5))
                        .on_input(|p| Message::SaveSave(SaveSaveMsg::OnSaveInput(p))),
                    )
                    .align_y(Vertical::Center)
                    .into(),
                    iced::widget::button("Save")
                        .on_press(Message::SaveSave(SaveSaveMsg::SavePath))
                        .into(),
                ])
                .height(Length::Shrink)
                .spacing(10)
                .into(),
            ])
            .spacing(10),
        )
        .padding(10)
        .style(bordered_box)
        .into();
        save_path_layout
    }

    pub fn init(&mut self, save_file: &LoadedSaveFile) -> Task<Message> {
        match &save_file.source {
            SaveSource::Path(path_buf) => self.save_path = path_buf.to_string_lossy().to_string(),
            SaveSource::TransferCodes => {}
            SaveSource::Data => {}
            SaveSource::Adb(_) => {}
        }
        self.cc = save_file.save_file.save_file.gvcc.cc;
        let save_data = save_file.save_file.write_to_zip_data();
        match save_data {
            Ok(s) => {
                self.save_data = s.clone();

                match save_file.save_file.save_file.write_with_hash() {
                    Ok(d) => self.adb.direction = AdbDirection::SaveSave(d),
                    Err(e) => return Task::done(Message::Error(e.to_string())),
                }
            }
            Err(e) => return Task::done(Message::Error(e.to_string())),
        }
        return self
            .adb
            .init()
            .map(|m| Message::SaveSave(SaveSaveMsg::Adb(m)));
    }

    fn save_save_to_path(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = PathBuf::from(&self.save_path);
        std::fs::write(&path, &self.save_data)?;

        Ok(path)
    }

    async fn save_system(data: Vec<u8>) -> Result<Option<()>, std::io::Error> {
        let file = rfd::AsyncFileDialog::new()
            .set_file_name("SAVE_DATA.zip")
            .save_file()
            .await;
        if let Some(file) = file {
            file.write(&data).await?;
            return Ok(Some(()));
        }
        Ok(None)
    }
}

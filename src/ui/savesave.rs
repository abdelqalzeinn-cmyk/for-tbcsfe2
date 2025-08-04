use std::path::PathBuf;

use iced::{
    Alignment, Element, Length, Task,
    alignment::{Horizontal, Vertical},
    widget::{container::bordered_box, text::LineHeight},
};

use crate::{
    country_code::CountryCode,
    network::password::{NewAccountInfo, TransferCodes, UploadInfo, create_and_upload},
    save::SaveFile,
    ui::{
        app::Message,
        loadsave::{LoadedSaveFile, LocalizedCC, SaveSource},
        localization::{LocaleManager, Localizable},
    },
};

#[derive(Debug, Clone)]
pub enum SaveSaveMsg {
    SelectPath,
    SavePath,
    OnSaveInput(String),
    SelectedData(Option<PathBuf>),
    UploadSave,
    Uploaded((TransferCodes, NewAccountInfo)),
    DoneTransfer,
}

#[derive(Debug, Clone, Default)]
pub struct SaveSave {
    pub save_path: String,
    pub cc: CountryCode,
    pub is_transferring: bool,
    pub codes: Option<TransferCodes>,
}

impl SaveSave {
    pub fn update(
        &mut self,
        message: SaveSaveMsg,
        save: &mut SaveFile,
        locale_manager: &LocaleManager,
    ) -> Task<Message> {
        match message {
            SaveSaveMsg::SelectPath => {
                todo!();
                // return Task::perform(select_save(), |p| {
                //     Message::SaveSave(SaveSaveMsg::SelectedData(p))
                // });
            }
            SaveSaveMsg::SavePath => match self.save_save_to_path(save) {
                Ok(s) => return Task::done(Message::SavedSave(s)),
                Err(e) => return Task::done(Message::Error(e.to_string())),
            },
            SaveSaveMsg::OnSaveInput(p) => self.save_path = p,
            SaveSaveMsg::SelectedData(path_buf) => {
                if let Some(path) = path_buf {
                    self.save_path = path.to_string_lossy().to_string();
                }
            }
            SaveSaveMsg::UploadSave => {
                self.is_transferring = true;
                let save_data = save.write_with_hash_no_zip();
                let save_info = UploadInfo::from_save(save);
                match save_data {
                    Ok(save_data) => {
                        return Task::done(Message::Notif("upload-start".localize(locale_manager)))
                            .chain(Task::perform(
                                create_and_upload(save_data, save_info),
                                |r| match r {
                                    Ok(codes) => Message::SaveSave(SaveSaveMsg::Uploaded(codes)),
                                    Err(e) => Message::Error(e.to_string()),
                                },
                            ))
                            .chain(Task::done(Message::SaveSave(SaveSaveMsg::DoneTransfer)));
                    }
                    Err(e) => {
                        return Task::done(Message::Error(e.to_string()))
                            .chain(Task::done(Message::SaveSave(SaveSaveMsg::DoneTransfer)));
                    }
                }
            }
            SaveSaveMsg::Uploaded((codes, new_account_info)) => {
                save.save.set_inquiry_code(new_account_info.inquiry_code);
                save.save
                    .set_password_refresh_token(new_account_info.password_refresh_token);
                save.account_info = Some(new_account_info.account_info);
                self.codes = Some(codes.clone());

                return Task::done(Message::Notif(
                    "successfully-uploaded".localize(locale_manager),
                ))
                .chain(Task::done(Message::Codes(codes)));
            }
            SaveSaveMsg::DoneTransfer => self.is_transferring = false,
        };
        Task::none()
    }

    pub fn view(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
        let save_path_layout = self.view_save_path_layout(locale_manager);
        let upload_save_layout = self.view_upload_save_layout(locale_manager);
        let mut col = vec![save_path_layout, upload_save_layout];
        if let Some(ref codes) = self.codes {
            let save_codes_layout = iced::widget::container(
                iced::widget::row([
                    iced::widget::container(iced::widget::column([
                        iced::widget::text("transfer-code".localize(&locale_manager))
                            .color(theme.palette().primary)
                            .into(),
                        iced::widget::text(&codes.transfer_code).into(),
                    ]))
                    .style(bordered_box)
                    .padding(10)
                    .width(Length::Fill)
                    .into(),
                    iced::widget::container(iced::widget::column([
                        iced::widget::text("confirmation-code".localize(&locale_manager))
                            .color(theme.palette().primary)
                            .into(),
                        iced::widget::text(&codes.confirmation_code).into(),
                    ]))
                    .style(bordered_box)
                    .width(Length::Fill)
                    .padding(10)
                    .into(),
                    iced::widget::container(iced::widget::column([
                        iced::widget::text("country-code".localize(&locale_manager))
                            .color(theme.palette().primary)
                            .into(),
                        iced::widget::text(
                            LocalizedCC::from_cc(self.cc, locale_manager).to_string(),
                        )
                        .into(),
                    ]))
                    .style(bordered_box)
                    .padding(10)
                    .width(Length::Fill)
                    .into(),
                ])
                .width(Length::Fill)
                .spacing(10),
            )
            .style(bordered_box)
            .padding(10)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into();

            col.push(save_codes_layout);
        }
        iced::widget::container(iced::widget::column(col).spacing(10)).into()
    }

    fn view_upload_save_layout(&self, locale_manager: &LocaleManager) -> Element<'_, Message> {
        let save_upload_layout: Element<Message> = iced::widget::container(
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
            .width(Length::Fill),
        )
        .padding(10)
        .style(bordered_box)
        .into();
        save_upload_layout
    }

    fn view_save_path_layout(&self, locale_manager: &LocaleManager) -> Element<'_, Message> {
        let save_path_layout: Element<Message> = iced::widget::container(
            iced::widget::row([
                iced::widget::text("save-path".localize(locale_manager))
                    .align_y(Vertical::Center)
                    .height(Length::Fill)
                    .into(),
                iced::widget::button(iced::widget::text("select-path".localize(locale_manager)))
                    .on_press(Message::SaveSave(SaveSaveMsg::SelectPath))
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
            .spacing(10),
        )
        .padding(10)
        .style(bordered_box)
        .into();
        save_path_layout
    }

    pub fn init(&mut self, save_file: &LoadedSaveFile) {
        match &save_file.source {
            SaveSource::Path(path_buf) => self.save_path = path_buf.to_string_lossy().to_string(),
            SaveSource::TransferCodes => {}
            SaveSource::Data => {}
        }
        self.cc = save_file.save_file.gvcc.cc;
    }

    fn save_save_to_path(&self, save: &SaveFile) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = PathBuf::from(&self.save_path);
        save.write_to_path(path.as_path())?;

        Ok(path)
    }
}

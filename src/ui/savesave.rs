use std::path::PathBuf;

use iced::{
    Element, Length, Task,
    alignment::Vertical,
    widget::{container::bordered_box, text::LineHeight},
};

use crate::{
    save::SaveFile,
    ui::{
        app::Message,
        loadsave::{LoadedSaveFile, SaveSource},
        localization::{LocaleManager, Localizable},
    },
};

#[derive(Debug, Clone)]
pub enum SaveSaveMsg {
    SelectPath,
    SavePath,
    OnSaveInput(String),
    SelectedData(Option<PathBuf>),
}

#[derive(Debug, Clone, Default)]
pub struct SaveSave {
    pub save_path: String,
}

impl SaveSave {
    pub fn update(&mut self, message: SaveSaveMsg, save: &SaveFile) -> Task<Message> {
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
        };
        Task::none()
    }

    pub fn view(
        &self,
        _theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, Message> {
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
        iced::widget::container(iced::widget::column([save_path_layout])).into()
    }

    pub fn init(&mut self, save_file: &LoadedSaveFile) {
        match &save_file.source {
            SaveSource::Path(path_buf) => self.save_path = path_buf.to_string_lossy().to_string(),
            SaveSource::TransferCodes => {}
            SaveSource::Data => {}
        }
    }

    fn save_save_to_path(&self, save: &SaveFile) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let path = PathBuf::from(&self.save_path);
        save.write_to_path(path.as_path())?;

        Ok(path)
    }
}

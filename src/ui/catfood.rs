use std::num::ParseIntError;

use iced::{Element, Length, Task, widget::row};

use crate::{save::SaveFile, ui::app::Message};

#[derive(Debug, Clone, Default)]
pub struct CatfoodView {
    pub current_value: String,
}

impl CatfoodView {
    pub fn init(&mut self, save_file: &SaveFile) {
        self.current_value = save_file.save.catfood.to_string();
    }

    pub fn get_value(&self) -> Result<i32, CatfoodError> {
        let val: i32 = self
            .current_value
            .parse()
            .map_err(CatfoodError::InvalidInt)?;

        if val < 0 {
            return Err(CatfoodError::TooSmall(0));
        }
        if val > 9999 {
            return Err(CatfoodError::TooLarge(9999));
        }

        Ok(val)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum CatfoodError {
    #[error("invalid integer: {0}")]
    InvalidInt(ParseIntError),
    #[error("too large, value cannot be greater than: {0}")]
    TooLarge(usize),
    #[error("too large, value cannot be smaller than: {0}")]
    TooSmall(usize),
}

#[derive(Debug, Clone)]
pub enum CatfoodMsg {
    Submit,
    TextInput(String),
}

impl CatfoodView {
    pub fn view(&self) -> Element<'_, Message> {
        let input = iced::widget::text_input("Catfood", &self.current_value)
            .on_submit(Message::Catfood(CatfoodMsg::Submit))
            .on_input(|s| Message::Catfood(CatfoodMsg::TextInput(s)))
            .width(Length::FillPortion(2))
            .into();

        let apply_button = iced::widget::button("Apply")
            .on_press(Message::Catfood(CatfoodMsg::Submit))
            .width(Length::FillPortion(1))
            .into();

        let hoz_space = iced::widget::horizontal_space()
            .width(Length::FillPortion(3))
            .into();

        row([input, apply_button, hoz_space]).spacing(10).into()
    }

    pub fn update(&mut self, message: CatfoodMsg, save_file: &mut SaveFile) -> Task<Message> {
        match message {
            CatfoodMsg::Submit => {
                let value = self.get_value();
                match value {
                    Ok(v) => {
                        save_file.save.catfood = v;
                        return Task::done(Message::Notif(format!("set catfood to: {v}")));
                    }
                    Err(e) => return Task::done(Message::Error(e.to_string())),
                }
            }
            CatfoodMsg::TextInput(s) => self.current_value = s,
        };

        Task::none()
    }
}

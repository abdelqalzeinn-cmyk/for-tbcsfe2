use std::{fmt::Display, num::ParseIntError};

use fluent::FluentArgs;
use iced::{Element, Length, Task};

use crate::{
    save::SaveFile,
    ui::{
        app::Message,
        localization::{LocaleManager, Localizable},
    },
};

pub trait EditView {
    type Message;

    fn init(&mut self, save_file: &SaveFile);

    fn view(&self, theme: &iced::Theme, locale_manager: &LocaleManager) -> Element<'_, Message>;

    fn update(
        &mut self,
        message: Self::Message,
        save_file: &mut SaveFile,
        locale_manager: &LocaleManager,
    ) -> Task<Message>;
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum BasicItemError {
    #[error("invalid integer: {0}")]
    InvalidInt(ParseIntError),
    #[error("too large, value cannot be greater than: {0}")]
    TooLarge(i32),
    #[error("too large, value cannot be smaller than: {0}")]
    TooSmall(i32),
}

#[derive(Debug, Clone, Default)]
pub struct BasicItemView<T> {
    pub item: T,
    pub current_value: String,
}

impl<T> BasicItemView<T> {
    pub fn get_value(&self, max: i32, min: i32) -> Result<i32, BasicItemError> {
        let val: i32 = self
            .current_value
            .parse()
            .map_err(BasicItemError::InvalidInt)?;

        if val < min {
            return Err(BasicItemError::TooSmall(min));
        }
        if val > max {
            return Err(BasicItemError::TooLarge(max));
        }

        Ok(val)
    }
}

pub trait BasicItem {
    fn get_save_value(save_file: &SaveFile) -> i32;
    fn set_save_value(save_file: &mut SaveFile, value: i32);

    fn feature() -> BasicItemFeature;

    fn max_value() -> i32 {
        9999
    }
    fn min_value() -> i32 {
        0
    }
}

#[derive(Debug, Clone)]
pub enum BasicItemMessage {
    Submit,
    TextInput(String),
    Max,
}

#[derive(Debug, Copy, Clone)]
pub enum BasicItemFeature {
    Catfood,
    Xp,
}

impl Display for BasicItemFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BasicItemFeature::Catfood => "Catfood",
                BasicItemFeature::Xp => "XP",
            }
        )
    }
}

impl<T: BasicItem> EditView for BasicItemView<T> {
    type Message = BasicItemMessage;
    fn init(&mut self, save_file: &SaveFile) {
        self.current_value = T::get_save_value(save_file).to_string();
    }

    fn view(&self, _theme: &iced::Theme, locale_manager: &LocaleManager) -> Element<'_, Message> {
        let input = iced::widget::text_input(&T::feature().to_string(), &self.current_value)
            .on_submit(Message::BasicItem(BasicItemMessage::Submit))
            .on_input(|s| Message::BasicItem(BasicItemMessage::TextInput(s)))
            .width(Length::FillPortion(2))
            .into();

        let apply_button =
            iced::widget::button(iced::widget::text("apply".localize(locale_manager)))
                .on_press(Message::BasicItem(BasicItemMessage::Submit))
                .into();

        let max_button =
            iced::widget::button(iced::widget::text("set-max".localize(locale_manager)))
                .on_press(Message::BasicItem(BasicItemMessage::Max))
                .into();

        let hoz_space = iced::widget::horizontal_space()
            .width(Length::FillPortion(3))
            .into();

        iced::widget::row([input, max_button, apply_button, hoz_space])
            .spacing(10)
            .into()
    }

    fn update(
        &mut self,
        message: Self::Message,
        save_file: &mut SaveFile,
        locale_manager: &LocaleManager,
    ) -> Task<Message> {
        match message {
            BasicItemMessage::Submit => {
                let value = self.get_value(T::max_value(), T::min_value());
                match value {
                    Ok(v) => {
                        T::set_save_value(save_file, v);
                        return {
                            let mut args = FluentArgs::with_capacity(2);
                            args.set("feature", T::feature().to_string());
                            args.set("value", v);
                            Task::done(Message::Notif(
                                "set-x-to-x".localize_with_args(locale_manager, &args),
                            ))
                        };
                    }
                    Err(e) => return Task::done(Message::Error(e.to_string())),
                }
            }
            BasicItemMessage::TextInput(s) => self.current_value = s,
            BasicItemMessage::Max => self.current_value = T::max_value().to_string(),
        };
        Task::none()
    }
}

use crate::ui::{
    app::Message,
    localization::{LocaleManager, Localizable},
};

pub fn localized_box<'a>(
    theme: &iced::Theme,
    label: &str,
    inner: iced::Element<'a, Message>,
    locale_manager: &LocaleManager,
) -> iced::Element<'a, Message> {
    labeled_box(theme, label.localize(locale_manager), inner)
}

pub fn labeled_box<'a>(
    theme: &iced::Theme,
    label: String,
    inner: iced::Element<'a, Message>,
) -> iced::Element<'a, Message> {
    iced::widget::container(
        iced::widget::column([
            iced::widget::text(label)
                .color(theme.palette().primary)
                .into(),
            inner,
        ])
        .spacing(10),
    )
    .padding(10)
    .style(iced::widget::container::bordered_box)
    .width(iced::Length::Fill)
    .into()
}

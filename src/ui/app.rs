use std::{fmt::Display, path::PathBuf};

use iced::{
    Element, Length, Pixels, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{button::Catalog, column, container, container::bordered_box, row},
};

use crate::ui::{
    catfood::{CatfoodMsg, CatfoodView},
    loadsave::{LoadSave, LoadSaveMsg, LoadedSaveFile},
    savesave::{SaveSave, SaveSaveMsg},
};

#[derive(Debug)]
pub struct ApplicationState {
    pub save_file: Option<LoadedSaveFile>,
    pub theme: Theme,
    pub current_error: Option<String>,
    pub current_notif: Option<String>,
    pub selected_screen: Option<UIOption>,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            save_file: None,
            theme: Theme::CatppuccinMocha,
            current_error: None,
            current_notif: None,
            selected_screen: Some(UIOption::LoadSave(LoadSave::default())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum UIOption {
    LoadSave(LoadSave),
    SaveSave(SaveSave),
    Catfood(CatfoodView),
}

impl UIOption {
    pub fn init(&mut self, save_file: Option<&LoadedSaveFile>) {
        if let Some(save_file) = save_file {
            match self {
                UIOption::LoadSave(_) => {}
                UIOption::SaveSave(save_save) => save_save.init(save_file),
                UIOption::Catfood(catfood_view) => catfood_view.init(&save_file.save_file),
            }
        }
    }
    pub fn base_matches(&self, other: &Self) -> bool {
        macro_rules! matches_opt {
            [$($var:pat),+] => {
                match self {
                    $($var => matches!(other, $var),)+
                }
            };
        }
        matches_opt![Self::LoadSave(_), Self::SaveSave(_), Self::Catfood(_)]
    }
    pub fn all() -> Vec<UIOption> {
        vec![
            UIOption::LoadSave(LoadSave::default()),
            Self::SaveSave(SaveSave::default()),
            Self::Catfood(CatfoodView::default()),
        ]
    }

    pub fn view(&self) -> Option<Element<'_, Message>> {
        Some(match self {
            UIOption::LoadSave(load_save) => load_save.view(),
            UIOption::SaveSave(save_save) => save_save.view(),
            UIOption::Catfood(catfood) => catfood.view(),
        })
    }

    pub fn needs_save_file(&self) -> bool {
        !matches!(self, Self::LoadSave(_))
    }
}

impl Display for &UIOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UIOption::LoadSave(_) => "Load Save",
                UIOption::Catfood(_) => "Catfood",
                UIOption::SaveSave(_) => "Save Save",
            }
        )
    }
}

impl ApplicationState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {}
            Message::LoadSave(msg) => {
                if let Some(UIOption::LoadSave(ref mut selected)) = self.selected_screen {
                    return selected.update(msg);
                }
            }
            Message::ChangePane(mut uioption) => {
                uioption.init(self.save_file.as_ref());
                self.selected_screen = Some(uioption);
            }
            Message::LoadedSave(save_file) => self.save_file = Some(*save_file),
            Message::Error(e) => self.current_error = Some(e),
            Message::Catfood(msg) => {
                if let Some(UIOption::Catfood(ref mut selected)) = self.selected_screen
                    && let Some(ref mut save_file) = self.save_file
                {
                    return selected.update(msg, &mut save_file.save_file);
                }
            }
            Message::SaveSave(save_save_msg) => {
                if let Some(UIOption::SaveSave(ref mut selected)) = self.selected_screen
                    && let Some(ref save_file) = self.save_file
                {
                    return selected.update(save_save_msg, &save_file.save_file);
                }
            }
            Message::SavedSave(path_buf) => {
                self.current_notif = Some(format!("saved save to: {}", path_buf.to_string_lossy()))
            }
            Message::Notif(notif) => self.current_notif = Some(notif),
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut notif_row: Vec<Element<Message>> = Vec::new();
        if let Some(ref error) = self.current_error {
            let errors =
                iced::widget::text(error).color(self.theme.extended_palette().danger.strong.color);
            notif_row.push(errors.into());
        }
        if let Some(ref notif) = self.current_notif {
            let notifs = iced::widget::text(notif);
            notif_row.push(notifs.into());
        }
        let title: Element<Message> = iced::widget::text("Battle Cats Save File Editor")
            .size(30)
            .color(self.theme.palette().primary)
            .width(Length::Fill)
            .align_y(Vertical::Center)
            .into();

        let mut title_row: Vec<Element<Message>> = Vec::new();

        title_row.push(title);

        if let Some(ref save_file) = self.save_file {
            let save_info: Element<Message> = container(save_file.view(&self.theme))
                .style(bordered_box)
                .align_x(Horizontal::Right)
                .align_y(Vertical::Center)
                .padding(10)
                .into();

            title_row.push(save_info);
        }

        notif_row.push(row(title_row).spacing(10).into());

        let mut option_row: Vec<Element<Message>> = Vec::new();

        for opt in UIOption::all() {
            let is_selected = self
                .selected_screen
                .as_ref()
                .is_some_and(|s| s.base_matches(&opt));
            let mut text = iced::widget::text((&opt).to_string());
            if is_selected {
                text = text.color(self.theme.extended_palette().success.strong.text);
            }
            let mut button = iced::widget::button(text)
                .width(Length::FillPortion(2))
                .style(move |t: &Theme, s| {
                    let mut s = t.style(&<Theme as iced::widget::button::Catalog>::default(), s);

                    if is_selected {
                        s = s.with_background(t.extended_palette().success.strong.color)
                    }

                    s
                });
            if !opt.needs_save_file() || self.save_file.is_some() {
                button = button.on_press_with(move || Message::ChangePane(opt.clone()));
            }
            option_row.push(button.into());
        }

        let mut pannel2: Vec<Element<Message>> = Vec::new();

        pannel2.push(column(option_row).spacing(Pixels(10.0)).into());

        if let Some(ref selected) = self.selected_screen {
            let heading = iced::widget::text(selected.to_string())
                .size(20)
                .color(self.theme.palette().primary)
                .into();
            let selected_view = selected.view();
            let mut col = Vec::new();
            col.push(heading);
            if let Some(view) = selected_view {
                col.push(view);
            }
            let second_pannel: Element<Message> = container(column(col).spacing(10))
                .style(bordered_box)
                .width(Length::FillPortion(8))
                .height(Length::Fill)
                .padding(5)
                .into();

            pannel2.push(second_pannel);
        }

        container(
            column([
                column(notif_row).into(),
                row(pannel2)
                    .height(Length::Fill)
                    .spacing(Pixels(10.0))
                    .into(),
            ])
            .spacing(10),
        )
        .padding(10)
        .into()
    }

    pub fn new() -> (Self, Task<Message>) {
        let app = <Self as Default>::default();

        (app, Task::none())
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    LoadedSave(Box<LoadedSaveFile>),
    ChangePane(UIOption),
    LoadSave(LoadSaveMsg),
    Catfood(CatfoodMsg),
    Error(String),
    Notif(String),
    SaveSave(SaveSaveMsg),
    SavedSave(PathBuf),
}

pub fn run() -> iced::Result {
    let application = iced::application("BCSFE", ApplicationState::update, ApplicationState::view)
        .theme(|s| s.theme.clone());

    application.run_with(ApplicationState::new)
}

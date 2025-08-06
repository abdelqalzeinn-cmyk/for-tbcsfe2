use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

use iced::{
    Element, Length, Pixels, Task, Theme,
    alignment::{Horizontal, Vertical},
    widget::{button::Catalog, column, container::bordered_box, row},
};
use unic_langid::LanguageIdentifier;

use crate::{
    network::password::TransferCodes,
    save::SaveFile,
    ui::{
        asset::AssetManager,
        catfood::{CatfoodView, XPView},
        editview::{BasicItemMessage, BasicItemView, EditView},
        loadsave::{LoadSave, LoadSaveMsg, LoadedSaveFile},
        localization::{LocaleManager, Localizable},
        mainstory::{MainStory, MainStoryMsg},
        savesave::{SaveSave, SaveSaveMsg},
    },
};

pub struct ApplicationState {
    pub save_file: Option<LoadedSaveFile>,
    pub theme: Theme,
    pub current_error: Option<String>,
    pub current_notif: Option<String>,
    pub selected_screen: Option<UIOption>,
    pub locale_manager: LocaleManager,
}

#[derive(Debug, Clone)]
pub enum UIOption {
    LoadSave(LoadSave),
    SaveSave(SaveSave),
    Catfood(BasicItemView<CatfoodView>),
    Xp(BasicItemView<XPView>),
    MainStory(MainStory),
}

impl UIOption {
    pub fn init(&mut self, save_file: Option<&LoadedSaveFile>) {
        if let Some(save_file) = save_file {
            macro_rules! init {
                [$($var:ident),+] => {
                    match self {
                        $(UIOption::$var(view) => view.init(&save_file.save_file),)+
                        _ => {}
                    }
                };
            }
            init![Catfood, Xp, MainStory];
            if let UIOption::SaveSave(save_save) = self {
                save_save.init(save_file)
            }
        }
    }
    pub fn base_matches(&self, other: &Self) -> bool {
        macro_rules! matches_opt {
            [$($var:ident),+] => {
                match self {
                    $(UIOption::$var(_) => matches!(other, UIOption::$var(_)),)+
                }
            };
        }
        matches_opt![LoadSave, SaveSave, Catfood, Xp, MainStory]
    }
    pub fn all() -> Vec<UIOption> {
        macro_rules! all {
            [$($var:ident => $typ:tt),+] => {
                vec![
                    $(
                     UIOption::$var($typ::default()),
                    )+
                ]
            };
        }
        all![
            LoadSave => LoadSave,
            SaveSave => SaveSave,
            Catfood => BasicItemView,
            Xp => BasicItemView,
            MainStory => MainStory

        ]
    }

    pub fn view(
        &self,
        theme: &Theme,
        locale_manager: &LocaleManager,
    ) -> Option<Element<'_, Message>> {
        macro_rules! view {
            [$($var:ident),+] => {
                match self {
                    $(UIOption::$var(view) => view.view(theme, locale_manager),)+
                }
            };
        }
        Some(view![LoadSave, SaveSave, Catfood, Xp, MainStory])
    }

    pub fn needs_save_file(&self) -> bool {
        !matches!(self, Self::LoadSave(_))
    }

    pub fn update_basic_item(
        &mut self,
        msg: BasicItemMessage,
        save_file: &mut SaveFile,
        locale_manager: &LocaleManager,
    ) -> Task<Message> {
        match self {
            UIOption::Catfood(basic_item_view) => {
                basic_item_view.update(msg, save_file, locale_manager)
            }
            UIOption::Xp(basic_item_view) => basic_item_view.update(msg, save_file, locale_manager),
            _ => Task::none(),
        }
    }

    pub fn get_str(&self) -> &'static str {
        macro_rules! get_str {
            [$($var:ident => $name:literal),+] => {
                match self {
                    $(UIOption::$var(_) => $name,)+
                }
            };
        }
        get_str![
            LoadSave => "load-save",
            Catfood => "catfood",
            SaveSave => "save-save",
            Xp => "xp",
            MainStory => "main-story"
        ]
    }
}

impl Display for &UIOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

impl ApplicationState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {}
            Message::LoadSave(msg) => {
                if let Some(UIOption::LoadSave(ref mut selected)) = self.selected_screen {
                    return selected.update(msg, &self.locale_manager);
                }
            }
            Message::ChangePane(mut uioption) => {
                uioption.init(self.save_file.as_ref());
                self.selected_screen = Some(uioption);
            }
            Message::LoadedSave(save_file) => self.save_file = Some(*save_file),
            Message::Error(e) => self.current_error = Some(e),
            Message::BasicItem(msg) => {
                if let Some(ref mut save_file) = self.save_file
                    && let Some(ref mut option) = self.selected_screen
                {
                    return option.update_basic_item(
                        msg,
                        &mut save_file.save_file,
                        &self.locale_manager,
                    );
                }
            }
            Message::SaveSave(save_save_msg) => {
                if let Some(UIOption::SaveSave(ref mut selected)) = self.selected_screen
                    && let Some(ref mut save_file) = self.save_file
                {
                    return selected.update(
                        save_save_msg,
                        &mut save_file.save_file,
                        &self.locale_manager,
                    );
                }
            }
            Message::SavedSave(path_buf) => {
                self.current_notif = Some(format!("saved save to: {}", path_buf.to_string_lossy()))
            }
            Message::Notif(notif) => self.current_notif = Some(notif),
            Message::MainStory(msg) => {
                if let Some(UIOption::MainStory(ref mut selected)) = self.selected_screen
                    && let Some(ref mut save_file) = self.save_file
                {
                    return selected.update(msg, &mut save_file.save_file, &self.locale_manager);
                }
            }
            Message::Codes(c) => {
                if let Some(ref mut save) = self.save_file {
                    save.codes = Some(c)
                }
            }
        };
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut notif_row: Vec<Element<Message>> = Vec::new();
        if let Some(ref error) = self.current_error {
            let errors =
                iced::widget::text(error).color(self.theme.extended_palette().danger.base.color);
            notif_row.push(errors.into());
        }
        if let Some(ref notif) = self.current_notif {
            let notifs = iced::widget::text(notif);
            notif_row.push(notifs.into());
        }
        let title: Element<Message> = iced::widget::text(self.locale_manager.localize("title"))
            .size(30)
            .color(self.theme.palette().primary)
            .width(Length::Fill)
            .align_y(Vertical::Center)
            .height(Length::Fill)
            .into();

        let mut title_row: Vec<Element<Message>> = Vec::new();

        title_row.push(title);

        if let Some(ref save_file) = self.save_file {
            let save_info: Element<Message> =
                iced::widget::container(save_file.view(&self.theme, &self.locale_manager))
                    .style(bordered_box)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Center)
                    .padding(10)
                    .into();

            title_row.push(save_info);
        }

        notif_row.push(row(title_row).height(Length::Shrink).spacing(10).into());

        let mut option_row: Vec<Element<Message>> = Vec::new();

        for opt in UIOption::all() {
            let is_selected = self
                .selected_screen
                .as_ref()
                .is_some_and(|s| s.base_matches(&opt));
            let mut text = iced::widget::text((&opt).localize(&self.locale_manager));
            if is_selected {
                text = text.color(self.theme.extended_palette().success.base.text);
            }
            let mut button = iced::widget::button(text)
                .width(Length::FillPortion(2))
                .style(move |t: &Theme, s| {
                    let mut s = t.style(&<Theme as iced::widget::button::Catalog>::default(), s);

                    if is_selected {
                        s = s.with_background(t.extended_palette().success.base.color)
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
            let heading = iced::widget::text(selected.localize(&self.locale_manager))
                .size(20)
                .color(self.theme.palette().primary)
                .into();
            let selected_view = selected.view(&self.theme, &self.locale_manager);
            let mut col = Vec::new();
            col.push(heading);
            if let Some(view) = selected_view {
                col.push(view);
            }
            let second_pannel: Element<Message> = iced::widget::container(column(col).spacing(10))
                .style(bordered_box)
                .width(Length::FillPortion(8))
                .height(Length::Fill)
                .padding(5)
                .into();

            pannel2.push(second_pannel);
        }

        iced::widget::container(
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

    pub fn new(
        filepath: Option<PathBuf>,
        locale_manager: LocaleManager,
    ) -> Result<(Self, Task<Message>), Box<dyn std::error::Error>> {
        let mut app = Self {
            save_file: None,
            theme: Theme::CatppuccinMocha,
            current_error: None,
            current_notif: None,
            selected_screen: Some(UIOption::LoadSave(LoadSave::default())),
            locale_manager,
        };

        if let Some(path) = filepath {
            let save = SaveFile::load_from_path_detect_cc(&path)?;
            app.save_file = Some(LoadedSaveFile {
                source: super::loadsave::SaveSource::Path(path),
                save_file: save,
                codes: None,
            });
        }

        Ok((app, Task::none()))
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    LoadedSave(Box<LoadedSaveFile>),
    ChangePane(UIOption),
    LoadSave(LoadSaveMsg),
    BasicItem(BasicItemMessage),
    Error(String),
    Notif(String),
    SaveSave(SaveSaveMsg),
    SavedSave(PathBuf),
    MainStory(MainStoryMsg),
    Codes(TransferCodes),
}

pub fn run_wasm() -> Result<(), Box<dyn std::error::Error>> {
    let lang = LanguageIdentifier::from_str("en")?;
    let application = iced::application(
        move || ApplicationState::new(None, LocaleManager::new_wasm(lang.clone())).unwrap(),
        ApplicationState::update,
        ApplicationState::view,
    )
    .theme(|s| s.theme.clone())
    .title(|a: &ApplicationState| a.locale_manager.localize("title"));

    application.run()?;

    Ok(())
}

pub fn run(
    filepath: Option<PathBuf>,
    assets_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let lang = LanguageIdentifier::from_str("en")?;
    let asset_manager = AssetManager::new(assets_path)?;
    let application = iced::application(
        move || {
            ApplicationState::new(
                filepath.clone(),
                LocaleManager::new(lang.clone(), &asset_manager).unwrap(),
            )
            .unwrap()
        },
        ApplicationState::update,
        ApplicationState::view,
    )
    .theme(|s| s.theme.clone())
    .title(|a: &ApplicationState| a.locale_manager.localize("title"));

    application.run()?;

    Ok(())
}

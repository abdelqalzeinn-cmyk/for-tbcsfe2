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
    edits::{Applyable, Edit},
    localization::{LocaleManager, Localizable},
    network::{account_info::SaveFileAccount, password::TransferCodes},
    ui::{
        asset::AssetManager,
        catfood::{CatfoodView, NormalTicketView, RareTicketView, XPView},
        editview::{BasicItemMessage, BasicItemView, EditLog, EditViewable},
        loadsave::{LoadSave, LoadSaveMsg, LoadedSaveFile},
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
    pub edits: Vec<crate::edits::Edit>,
    pub current_edits: Vec<Edit>,
    pub locale_manager: LocaleManager,
}

#[derive(Debug, Clone)]
pub enum UIOption {
    LoadSave(LoadSave),
    SaveSave(SaveSave),
    Catfood(BasicItemView<CatfoodView>),
    Xp(BasicItemView<XPView>),
    NormalTickets(BasicItemView<NormalTicketView>),
    RareTickets(BasicItemView<RareTicketView>),
    MainStory(MainStory),
    EditLog(EditLog),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIType {
    LoadSave,
    SaveSave,
    Catfood,
    Xp,
    MainStory,
    EditLog,
    NormalTickets,
    RareTickets,
}

impl UIType {
    async fn to_opt(self) -> UIOption {
        macro_rules! matches_opt {
            [$($var:ident => $type:ident),+] => {
                match self {
                    $(UIType::$var => UIOption::$var($type::new().await),)+
                }
            };
        }
        matches_opt![
            LoadSave => LoadSave,
            SaveSave => SaveSave,
            Catfood => BasicItemView,
            Xp => BasicItemView,
            MainStory => MainStory,
            EditLog => EditLog,
            NormalTickets => BasicItemView,
            RareTickets => BasicItemView
        ]
    }
}
impl From<&UIOption> for UIType {
    fn from(value: &UIOption) -> Self {
        macro_rules! matches_opt {
            [$($var:ident),+] => {
                match value {
                    $(UIOption::$var(_) => UIType::$var,)+
                }
            };
        }
        matches_opt![
            LoadSave,
            SaveSave,
            Catfood,
            Xp,
            MainStory,
            EditLog,
            NormalTickets,
            RareTickets
        ]
    }
}
impl UIType {
    pub fn all() -> Vec<UIType> {
        vec![
            Self::LoadSave,
            Self::SaveSave,
            Self::Catfood,
            Self::Xp,
            Self::NormalTickets,
            Self::RareTickets,
            Self::MainStory,
            Self::EditLog,
        ]
    }

    pub fn matches_option(&self, other: &UIOption) -> bool {
        macro_rules! matches_opt {
            [$($var:ident),+] => {
                match self {
                    $(UIType::$var => matches!(other, UIOption::$var(_)),)+
                }
            };
        }
        matches_opt![
            LoadSave,
            SaveSave,
            Catfood,
            Xp,
            MainStory,
            EditLog,
            NormalTickets,
            RareTickets
        ]
    }

    pub fn get_str(&self) -> &'static str {
        macro_rules! get_str {
            [$($var:ident => $name:literal),+] => {
                match self {
                    $(UIType::$var => $name,)+
                }
            };
        }
        get_str![
            LoadSave => "load-save",
            Catfood => "catfood",
            SaveSave => "save-save",
            Xp => "xp",
            MainStory => "main-story",
            EditLog => "edit-log",
            NormalTickets => "normal-tickets",
            RareTickets => "rare-tickets"
        ]
    }

    pub fn needs_save_file(&self) -> bool {
        !matches!(self, Self::LoadSave)
    }
}

impl UIOption {
    pub fn init(
        &mut self,
        save_file: Option<&LoadedSaveFile>,
        edit_log: &Vec<Edit>,
    ) -> Task<Message> {
        if let Some(save_file) = save_file {
            macro_rules! init {
                [$($var:ident),+] => {
                    match self {
                        $(UIOption::$var(view) => view.init(&save_file.save_file),)+
                        _ => {}
                    }
                };
            }
            init![Catfood, Xp, MainStory, NormalTickets, RareTickets];
            if let UIOption::SaveSave(save_save) = self {
                return save_save.init(save_file);
            }
        }
        if let UIOption::LoadSave(load_save) = self {
            return load_save.init();
        }
        if let UIOption::EditLog(log) = self {
            log.init(edit_log.clone());
        }
        return Task::none();
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
        Some(view![
            LoadSave,
            SaveSave,
            Catfood,
            Xp,
            MainStory,
            EditLog,
            RareTickets,
            NormalTickets
        ])
    }

    pub fn update_basic_item(
        &mut self,
        msg: BasicItemMessage,
        locale_manager: &LocaleManager,
    ) -> Task<Message> {
        macro_rules! update {
            [$($var:ident),+] => {
                match self {
                    $(UIOption::$var(view) => view.update(msg, locale_manager),)+
                    _ => Task::none()
                }
            };
        }
        update![Catfood, Xp, NormalTickets, RareTickets]
    }
}

impl Display for UIType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

impl ApplicationState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Init => {
                if let Some(ref mut sel) = self.selected_screen {
                    return sel.init(None, &self.edits);
                } else {
                    return Task::done(Message::ChangePane(UIType::LoadSave));
                }
            }
            Message::LoadSave(msg) => {
                if let Some(UIOption::LoadSave(ref mut selected)) = self.selected_screen {
                    return selected.update(msg, &self.locale_manager);
                }
            }
            Message::ChangePane(uitype) => {
                return Task::perform(uitype.to_opt(), Message::ChangedScreen);
            }
            Message::ChangedScreen(mut uioption) => {
                if let Some(ref mut save) = self.save_file {
                    let len = self.current_edits.len();
                    for edit in self.current_edits.drain(0..len) {
                        edit.apply(&mut save.save_file.save_file);
                        self.edits.push(edit);
                    }
                }
                let m = uioption.init(self.save_file.as_ref(), &self.edits);
                self.selected_screen = Some(uioption);
                return m;
            }
            Message::LoadedSave(save_file) => {
                self.save_file = Some(*save_file);
                return Task::done(Message::Notif("loaded-save".localize(&self.locale_manager)));
            }
            Message::Error(e) => self.current_error = Some(e),
            Message::BasicItem(msg) => {
                if let Some(ref mut option) = self.selected_screen {
                    return option.update_basic_item(msg, &self.locale_manager);
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
                if let Some(UIOption::MainStory(ref mut selected)) = self.selected_screen {
                    return selected.update(msg, &self.locale_manager);
                }
            }
            Message::Codes(c) => {
                if let Some(ref mut save) = self.save_file {
                    save.codes = Some(c)
                }
            }
            Message::Edit(edit) => self.current_edits.push(edit),
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

        for opt in UIType::all() {
            let is_selected = self
                .selected_screen
                .as_ref()
                .is_some_and(|s| opt.matches_option(s));
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
            let sel_type: UIType = selected.into();
            let heading = iced::widget::text(sel_type.localize(&self.locale_manager))
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
            selected_screen: None,
            edits: Vec::new(),
            current_edits: Vec::new(),
            locale_manager,
        };

        if let Some(path) = filepath {
            let save = SaveFileAccount::load_from_path(&path, None)?;
            // dbg!(
            //     save.save_file
            //         .save
            //         .gv_70100
            //         .clone()
            //         .unwrap_or_default()
            //         .catamin_stages
            // );
            app.save_file = Some(LoadedSaveFile {
                source: super::loadsave::SaveSource::Path(path),
                save_file: save,
                codes: None,
            });
        }

        Ok((app, Task::done(Message::Init)))
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Init,
    Edit(crate::edits::Edit),
    LoadedSave(Box<LoadedSaveFile>),
    ChangePane(UIType),
    LoadSave(LoadSaveMsg),
    BasicItem(BasicItemMessage),
    Error(String),
    Notif(String),
    SaveSave(SaveSaveMsg),
    SavedSave(PathBuf),
    MainStory(MainStoryMsg),
    Codes(TransferCodes),
    ChangedScreen(UIOption),
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

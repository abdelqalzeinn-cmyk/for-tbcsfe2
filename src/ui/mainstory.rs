use std::fmt::Display;

use iced::{Length, Task, alignment::Vertical, widget::container::bordered_box};

use crate::{
    game::main_story::StoryChapterType,
    network::account_info::SaveFileAccount,
    ui::{
        app::Message,
        editview::EditView,
        localization::{LocaleManager, Localizable},
    },
};

#[derive(Debug, Clone)]
pub struct MainStory {
    selected_chapters: [bool; 9],
    clear_count_chapters: String,
}

impl Default for MainStory {
    fn default() -> Self {
        Self {
            selected_chapters: [false; 9],
            clear_count_chapters: 1.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chapters {
    Eoc1,
    Eoc2,
    Eoc3,
    Itf1,
    Itf2,
    Itf3,
    Cotc1,
    Cotc2,
    Cotc3,
}
impl Chapters {
    // const ALL: [Chapters; 9] = [
    //     Self::Eoc1,
    //     Self::Eoc2,
    //     Self::Eoc3,
    //     Self::Itf1,
    //     Self::Itf2,
    //     Self::Itf3,
    //     Self::Cotc1,
    //     Self::Cotc2,
    //     Self::Cotc3,
    // ];

    const ALL_GROUPED: [[Chapters; 3]; 3] = [
        [Self::Eoc1, Self::Eoc2, Self::Eoc3],
        [Self::Itf1, Self::Itf2, Self::Itf2],
        [Self::Cotc1, Self::Cotc2, Self::Cotc3],
    ];
}

impl Display for Chapters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Chapters::Eoc1 => "chapter_1",
                Chapters::Eoc2 => "chapter_2",
                Chapters::Eoc3 => "chapter_3",
                Chapters::Itf1 => "chapter_4",
                Chapters::Itf2 => "chapter_5",
                Chapters::Itf3 => "chapter_6",
                Chapters::Cotc1 => "chapter_7",
                Chapters::Cotc2 => "chapter_8",
                Chapters::Cotc3 => "chapter_9",
            }
        )
    }
}

impl From<Chapters> for String {
    fn from(value: Chapters) -> Self {
        value.to_string()
    }
}

#[derive(Debug, Clone)]
pub enum MainStoryMsg {
    SelectChapter(usize, bool),
    ToggleAll,
    ClearChapters,
    EditClearCountChapters(String),
}

impl EditView for MainStory {
    type Message = MainStoryMsg;
    fn init(&mut self, _save_file: &SaveFileAccount) {}
    fn view(
        &self,
        _theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> iced::Element<'_, super::app::Message> {
        let mut items = Vec::new();
        for (i, chaps) in Chapters::ALL_GROUPED.into_iter().enumerate() {
            let mut row_items = Vec::new();
            for (j, chap) in chaps.into_iter().enumerate() {
                let index = i * 3 + j;
                let selected = self.selected_chapters[index];
                let checkbox = iced::widget::checkbox(chap.localize(locale_manager), selected)
                    .on_toggle(move |tog| {
                        Message::MainStory(MainStoryMsg::SelectChapter(index, tog))
                    });
                row_items.push(checkbox.width(Length::Fill).into())
            }

            items.push(iced::widget::column(row_items).width(Length::Fill).into());
        }

        let row = iced::widget::row(items).width(Length::Fill).into();

        let toggle_all = iced::widget::button(iced::widget::text(
            "toggle-select-all".localize(locale_manager),
        ))
        .on_press(Message::MainStory(MainStoryMsg::ToggleAll))
        .into();

        let select_pannel = iced::widget::column([row, toggle_all]).spacing(10).into();

        let clear_chapters = iced::widget::button(iced::widget::text(
            "clear-all-selected-chapters".localize(locale_manager),
        ))
        .on_press_maybe(
            if self.selected_chapters.iter().all(|s| !*s)
                || self.clear_count_chapters.parse::<usize>().is_err()
            {
                None
            } else {
                Some(Message::MainStory(MainStoryMsg::ClearChapters))
            },
        )
        .into();

        let clear_count_box = iced::widget::text_input(
            &"clear-count".localize(locale_manager),
            &self.clear_count_chapters,
        )
        .on_input(|inp: String| Message::MainStory(MainStoryMsg::EditClearCountChapters(inp)))
        .into();

        let clear_chapters_pannel = iced::widget::container(
            iced::widget::row([
                clear_chapters,
                iced::widget::text("clear-count".localize(locale_manager))
                    .align_y(Vertical::Center)
                    .height(Length::Fill)
                    .into(),
                clear_count_box,
            ])
            .height(Length::Shrink)
            .spacing(10),
        )
        .padding(10)
        .style(bordered_box)
        .into();

        let edit_pannel = iced::widget::row([clear_chapters_pannel]).into();

        iced::widget::column([select_pannel, edit_pannel])
            .spacing(10)
            .into()
    }
    fn update(
        &mut self,
        message: Self::Message,
        save_file: &mut SaveFileAccount,
        _locale_manager: &LocaleManager,
    ) -> iced::Task<super::app::Message> {
        match message {
            MainStoryMsg::SelectChapter(ind, enabled) => self.selected_chapters[ind] = enabled,
            MainStoryMsg::ToggleAll => {
                if self.selected_chapters.iter().all(|f| *f) {
                    self.selected_chapters = [false; 9]
                } else {
                    self.selected_chapters = [true; 9]
                }
            }
            MainStoryMsg::ClearChapters => {
                for (i, selected) in self.selected_chapters.iter().enumerate() {
                    if *selected {
                        save_file.save_file.save.story_chapters.clear_chapter(
                            crate::game::main_story::ClearChapterOptions {
                                chapter: StoryChapterType::from_usize_human(i)
                                    .unwrap_or_else(|| panic!("{i} was not between 0 and 8")),
                                clear_amount: self
                                    .clear_count_chapters
                                    .parse()
                                    .expect("clear count was valid"),
                                add_to_clears: false,
                            },
                        );
                    }
                }

                return Task::done(Message::Notif("cleared story chapters".to_string()));
            }
            MainStoryMsg::EditClearCountChapters(v) => self.clear_count_chapters = v,
        };

        Task::none()
    }
}

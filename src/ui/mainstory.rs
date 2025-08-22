use std::collections::HashMap;
use std::fmt::Display;

use fluent::{FluentArgs, FluentValue};
use iced::Border;
use iced::{Length, Task, alignment::Vertical, widget::container::bordered_box};

use crate::edits::EditMemory;
use crate::edits::main_story::{ClearStoryChapters, StoryChaptersEdit};
use crate::game::main_story::{ClearStageOptions, TOTAL_STORY_CHAPTERS};
use crate::localization::{LocaleManager, Localizable};
use crate::{
    game::main_story::{
        InnerChapterType, StageId, StoryChapterType, StoryChapters, StoryStage, TOTAL_INGAME_STAGES,
    },
    network::account_info::SaveFileAccount,
    ui::{app::Message, editview::EditViewable, helper::labeled_box},
};

#[derive(Debug, Clone)]
pub struct MainStory {
    selected_chapters: [bool; 9],
    clear_count_chapters: String,
    story: StoryChapters,
    inputs: HashMap<StoryStage, String>,
    current_tab: Option<StoryChapterType>,
    stage_searches: HashMap<StoryChapterType, String>,
}

impl MainStory {
    pub async fn new() -> Self {
        Self {
            selected_chapters: [false; 9],
            clear_count_chapters: 1.to_string(),
            story: StoryChapters::default(),
            inputs: HashMap::new(),
            stage_searches: HashMap::new(),
            current_tab: None,
        }
    }

    fn get_input(&self, id: StoryStage) -> String {
        self.inputs
            .get(&id)
            .cloned()
            .unwrap_or(self.story.get_clear_amount(id).to_string())
    }

    fn view_stage_clear_pannel(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
        chapter_id: StoryChapterType,
    ) -> iced::Element<'_, Message> {
        let mut stage_cols: Vec<iced::Element<'_, Message>> =
            Vec::with_capacity(TOTAL_INGAME_STAGES);

        let progress = self.story.get_chapter_progress(chapter_id);
        let clear_count_label = "clear-count".localize(locale_manager);

        let searched_value = self.get_stage_search(chapter_id);

        for stage_index in 0..TOTAL_INGAME_STAGES {
            let stage_id = StageId::new(stage_index as u8);

            if let Some(stage_id) = stage_id {
                let stage = StoryStage::new(chapter_id, stage_id);
                let key = stage.localize_stage(locale_manager);

                if !(stage_index + 1)
                    .to_string()
                    .contains(&searched_value.to_lowercase())
                    && !key.to_lowercase().contains(&searched_value.to_lowercase())
                {
                    continue;
                }

                let num = iced::widget::text(stage_index + 1)
                    .width(Length::FillPortion(1))
                    .height(Length::Fill)
                    .align_y(Vertical::Center);

                let label = iced::widget::text(key)
                    .width(Length::FillPortion(4))
                    .height(Length::Fill)
                    .align_y(Vertical::Center);

                let clear_up_to =
                    iced::widget::button(iced::widget::text("clear".localize(locale_manager)))
                        .on_press(Message::MainStory(MainStoryMsg::ClearUpTo(stage)));
                let unclear_down_to =
                    iced::widget::button(iced::widget::text("unclear".localize(locale_manager)))
                        .on_press(Message::MainStory(MainStoryMsg::UnClearDownTo(stage)));

                let clear_count = self.get_input(stage);

                let clear_amount = self.story.get_clear_amount(stage);

                let clear_count_entry = iced::widget::text_input(&clear_count_label, &clear_count)
                    .width(Length::FillPortion(4))
                    .on_input(move |i| {
                        Message::MainStory(MainStoryMsg::EditClearCountStages(i, stage))
                    })
                    .on_submit_maybe(if clear_count.parse::<i32>().is_ok() {
                        Some(Message::MainStory(MainStoryMsg::SubmitClearAmountStages(
                            stage,
                        )))
                    } else {
                        None
                    });

                let mut row_data = vec![num.into(), label.into()];

                if progress <= (stage_index as i32) {
                    row_data.push(clear_up_to.into());
                }
                if clear_amount != 0 {
                    row_data.push(unclear_down_to.into());
                }

                row_data.push(clear_count_entry.into());

                let row = iced::widget::row(row_data).spacing(10).height(32);

                let len = stage_cols.len();

                stage_cols.push(
                    iced::widget::container(row)
                        .style(move |t| {
                            bordered_box(t)
                                .background(match len.is_multiple_of(2) {
                                    true => t.extended_palette().background.weak.color,
                                    false => t.extended_palette().background.strong.color,
                                })
                                .border(Border::default())
                        })
                        .padding(5)
                        // .height(Length::Shrink)
                        .into(),
                );
            }
        }

        let scroll_area = iced::widget::scrollable(iced::widget::column(stage_cols))
            .spacing(10)
            .width(Length::Fill);

        let search_box = iced::widget::text_input(
            &"filter".localize(locale_manager),
            &self.get_stage_search(chapter_id),
        )
        .on_input(move |s| Message::MainStory(MainStoryMsg::SearchStage(s, chapter_id)));

        labeled_box(
            theme,
            chapter_id.localize(locale_manager),
            iced::widget::column([search_box.into(), scroll_area.into()])
                .spacing(10)
                .into(),
        )
    }

    fn get_stage_search(&self, chapter: StoryChapterType) -> String {
        self.stage_searches
            .get(&chapter)
            .cloned()
            .unwrap_or_default()
    }

    fn view_select_chapter_pannel(
        &self,
        locale_manager: &LocaleManager,
    ) -> iced::Element<'_, Message> {
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
        select_pannel
    }

    fn view_clear_all_chapters_pannel(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> iced::Element<'_, Message> {
        let select_pannel = self.view_select_chapter_pannel(locale_manager);
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

        labeled_box(
            theme,
            "clear-whole-chapters".localize(locale_manager),
            iced::widget::column([
                select_pannel,
                iced::widget::row([
                    clear_chapters,
                    iced::widget::text("clear-count".localize(locale_manager))
                        .align_y(Vertical::Center)
                        .height(Length::Fill)
                        .into(),
                    clear_count_box,
                ])
                .height(Length::Shrink)
                .spacing(10)
                .into(),
            ])
            .spacing(10)
            .into(),
        )
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
                Chapters::Eoc1 => "chapter-1",
                Chapters::Eoc2 => "chapter-2",
                Chapters::Eoc3 => "chapter-3",
                Chapters::Itf1 => "chapter-4",
                Chapters::Itf2 => "chapter-5",
                Chapters::Itf3 => "chapter-6",
                Chapters::Cotc1 => "chapter-7",
                Chapters::Cotc2 => "chapter-8",
                Chapters::Cotc3 => "chapter-9",
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
    EditClearCountStages(String, StoryStage),
    SubmitClearAmountStages(StoryStage),
    SearchStage(String, StoryChapterType),
    ClearUpTo(StoryStage),
    UnClearDownTo(StoryStage),
    SelectChapterTab(StoryChapterType),
}

impl EditViewable for MainStory {
    type Message = MainStoryMsg;
    fn init(&mut self, save_file: &SaveFileAccount) {
        self.story = save_file.save_file.save.story_chapters;
    }

    fn view(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> iced::Element<'_, super::app::Message> {
        let clear_chapters_pannel = self.view_clear_all_chapters_pannel(theme, locale_manager);

        let mut col = vec![clear_chapters_pannel];

        let mut tabs = Vec::new();

        for i in 0..9 {
            let chap_type = StoryChapterType::from_usize_human(i).expect("i is within 0 - 8");

            let mut btn =
                iced::widget::button(iced::widget::text(chap_type.localize(locale_manager)))
                    .on_press(Message::MainStory(MainStoryMsg::SelectChapterTab(
                        chap_type,
                    )));
            if self.current_tab == Some(chap_type) {
                btn = btn.style(move |t: &iced::Theme, s| {
                    iced::widget::button::Catalog::style(
                        t,
                        &<iced::Theme as iced::widget::button::Catalog>::default(),
                        s,
                    )
                    .with_background(t.extended_palette().success.base.color)
                });
            }

            tabs.push(btn.into());
        }

        col.push(labeled_box(
            theme,
            "stage-clear".localize(locale_manager),
            iced::widget::row(tabs)
                .spacing(10)
                .wrap()
                .vertical_spacing(10)
                .into(),
        ));

        if let Some(current_tab) = self.current_tab {
            let clear_stages_pannel =
                self.view_stage_clear_pannel(theme, locale_manager, current_tab);
            col.push(clear_stages_pannel);
        }

        iced::widget::column(col).spacing(10).into()
    }
    fn update(
        &mut self,
        message: Self::Message,
        locale_manager: &LocaleManager,
    ) -> iced::Task<super::app::Message> {
        match message {
            MainStoryMsg::SelectChapter(ind, enabled) => {
                self.selected_chapters[ind] = enabled;
            }
            MainStoryMsg::ToggleAll => {
                if self.selected_chapters.iter().all(|f| *f) {
                    self.selected_chapters = [false; 9]
                } else {
                    self.selected_chapters = [true; 9]
                }
            }
            MainStoryMsg::ClearChapters => {
                let mut edits = Vec::with_capacity(self.selected_chapters.len());
                for (i, selected) in self.selected_chapters.iter().enumerate() {
                    if *selected {
                        let opts = crate::game::main_story::ClearChapterOptions {
                            chapter: StoryChapterType::from_usize_human(i)
                                .unwrap_or_else(|| panic!("{i} was not between 0 and 8")),
                            clear_amount: self
                                .clear_count_chapters
                                .parse()
                                .expect("clear count was valid"),
                        };
                        self.story.clear_chapter(opts);
                        edits.push(crate::edits::Edit::MainStory(StoryChaptersEdit(
                            crate::edits::EditMemory::new(
                                ClearStoryChapters::ClearChapter(opts),
                                self.story,
                            ),
                        )));
                    }
                }

                return Task::done(Message::Edit(edits)).chain(Task::done(Message::Notif(
                    "cleared story chapters".to_string(),
                )));
            }
            MainStoryMsg::EditClearCountChapters(v) => self.clear_count_chapters = v,
            MainStoryMsg::EditClearCountStages(v, id) => {
                self.inputs.insert(id, v);
            }
            MainStoryMsg::SubmitClearAmountStages(story_stage) => {
                let clear_amount: i32 = self.get_input(story_stage).parse().unwrap_or_default();

                let mut edits = Vec::new();

                if clear_amount != 0 {
                    for stage_index in story_stage.stage_id.iter_from_start() {
                        let story_stage2 = StoryStage {
                            chapter: story_stage.chapter,
                            stage_id: stage_index,
                        };
                        if self.story.get_clear_amount(story_stage2) != 0 {
                            continue;
                        }
                        let opts2 = ClearStageOptions::default()
                            .with_stage(story_stage2)
                            .with_clear_amount(clear_amount);
                        edits.push(
                            StoryChaptersEdit(EditMemory::new(
                                ClearStoryChapters::ClearStage(opts2),
                                self.story,
                            ))
                            .into(),
                        );
                        self.story.clear_stage(opts2);
                    }
                } else {
                    for stage_index in story_stage.stage_id.iter_to_end().rev() {
                        let story_stage2 = StoryStage {
                            chapter: story_stage.chapter,
                            stage_id: stage_index,
                        };
                        if self.story.get_clear_amount(story_stage2) == 0 {
                            continue;
                        }
                        let opts2 = ClearStageOptions::default()
                            .with_stage(story_stage2)
                            .with_clear_amount(0);
                        edits.push(
                            StoryChaptersEdit(EditMemory::new(
                                ClearStoryChapters::ClearStage(opts2),
                                self.story,
                            ))
                            .into(),
                        );
                        self.story.clear_stage(opts2);
                    }
                }

                let opts = ClearStageOptions::default()
                    .with_stage(story_stage)
                    .with_clear_amount(clear_amount)
                    .with_progress_type(
                        crate::game::main_story::ProgressType::OnlySetProgressIfLaterStage,
                    );

                edits.push(
                    StoryChaptersEdit(EditMemory::new(
                        ClearStoryChapters::ClearStage(opts),
                        self.story,
                    ))
                    .into(),
                );
                self.story.clear_stage(opts);
                self.inputs.remove(&story_stage);

                return Task::done(Message::Edit(edits)).chain(Task::done(Message::Notif(
                    if clear_amount == 0 {
                        "uncleared-stage"
                    } else {
                        "cleared-stage"
                    }
                    .localize_with_args(
                        locale_manager,
                        &FluentArgs::from_iter([
                            ("chapter", story_stage.chapter.localize(locale_manager)),
                            ("stage", story_stage.localize_stage(locale_manager)),
                        ]),
                    ),
                )));
            }
            MainStoryMsg::SearchStage(v, story_chapter_type) => {
                self.stage_searches.insert(story_chapter_type, v);
            }
            MainStoryMsg::ClearUpTo(stage_id) => {
                let mut edits = Vec::new();
                for stage in stage_id.stage_id.iter_from_start() {
                    let story_stage = StoryStage {
                        chapter: stage_id.chapter,
                        stage_id: stage,
                    };
                    if self.story.get_clear_amount(story_stage) == 0 {
                        let opt = ClearStageOptions::default().with_stage(story_stage);

                        edits.push(crate::edits::Edit::MainStory(StoryChaptersEdit(
                            EditMemory {
                                new: ClearStoryChapters::ClearStage(opt),
                                old: self.story,
                            },
                        )));
                        self.story.clear_stage(opt);
                    }
                }

                return Task::done(Message::Edit(edits)).chain(Task::done(Message::Notif(
                    "cleared-stage-up-to".localize_with_args(
                        locale_manager,
                        &FluentArgs::from_iter([
                            ("chapter", stage_id.chapter.localize(locale_manager)),
                            ("stage", stage_id.localize_stage(locale_manager)),
                        ]),
                    ),
                )));
            }
            MainStoryMsg::UnClearDownTo(stage) => {
                let mut edits = Vec::new();
                for stage_index in stage.stage_id.iter_to_end().rev() {
                    let stage_id: StageId = stage_index.try_into().expect("stage id is valid");
                    let opt = ClearStageOptions::default()
                        .with_chapter(stage.chapter)
                        .with_clear_amount(0)
                        .with_stage_id(stage_id);

                    edits.push(crate::edits::Edit::MainStory(StoryChaptersEdit(
                        EditMemory {
                            new: ClearStoryChapters::ClearStage(opt),
                            old: self.story,
                        },
                    )));
                    self.story.clear_stage(opt);
                }

                return Task::done(Message::Edit(edits)).chain(Task::done(Message::Notif(
                    "uncleared-stage-down-to".localize_with_args(
                        locale_manager,
                        &FluentArgs::from_iter([
                            ("chapter", stage.chapter.localize(locale_manager)),
                            ("stage", stage.localize_stage(locale_manager)),
                        ]),
                    ),
                )));
            }
            MainStoryMsg::SelectChapterTab(story_chapter_type) => {
                self.current_tab = Some(story_chapter_type);
            }
        };

        Task::none()
    }
}

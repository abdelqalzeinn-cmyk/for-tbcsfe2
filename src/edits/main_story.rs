use crate::{
    edits::{Applyable, Edit, EditMemory},
    game::main_story::{
        ClearAllChaptersOptions, ClearChapterOptions, ClearStageOptions, StoryChapters,
    },
    save::SaveFile,
};

#[derive(Debug, Clone, Copy)]
pub enum ClearStoryChapters {
    ClearStage(ClearStageOptions),
    ClearChapter(ClearChapterOptions),
    ClearAll(ClearAllChaptersOptions),
}

#[cfg(feature = "localization")]
impl crate::localization::Localizable for ClearStoryChapters {
    fn localize_with_args(
        &self,
        manager: &crate::localization::LocaleManager,
        _args: &fluent::FluentArgs,
    ) -> String {
        match self {
            ClearStoryChapters::ClearStage(clear_stage_options) => {
                if clear_stage_options.clear_amount == 0 {
                    "uncleared-stage"
                } else {
                    "cleared-stage"
                }
                .localize_with_args(
                    manager,
                    &fluent::FluentArgs::from_iter([
                        (
                            "chapter",
                            clear_stage_options.stage.chapter.localize(manager),
                        ),
                        ("stage", clear_stage_options.stage.localize_stage(manager)),
                    ]),
                )
            }
            ClearStoryChapters::ClearChapter(clear_chapter_options) => "clear-chapter"
                .localize_with_args(
                    manager,
                    &fluent::FluentArgs::from_iter([(
                        "chapter",
                        clear_chapter_options.chapter.localize(manager),
                    )]),
                ),
            ClearStoryChapters::ClearAll(clear_all_chapters_options) => {
                "clear-all-chapters".localize(manager)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StoryChaptersEdit(pub EditMemory<ClearStoryChapters, StoryChapters>);

impl Applyable for StoryChaptersEdit {
    fn apply(&self, save_file: &mut SaveFile) {
        match self.0.new {
            ClearStoryChapters::ClearStage(clear_stage_options) => save_file
                .save
                .story_chapters
                .clear_stage(clear_stage_options),
            ClearStoryChapters::ClearChapter(clear_chapter_options) => save_file
                .save
                .story_chapters
                .clear_chapter(clear_chapter_options),
            ClearStoryChapters::ClearAll(clear_all_chapters_options) => save_file
                .save
                .story_chapters
                .clear_all(clear_all_chapters_options),
        }
    }
    fn revert(&self, save_file: &mut SaveFile) {
        save_file.save.story_chapters = self.0.old;
    }
}

impl From<StoryChaptersEdit> for Edit {
    fn from(value: StoryChaptersEdit) -> Self {
        Self::MainStory(value)
    }
}

#[cfg(feature = "localization")]
impl crate::localization::Localizable for StoryChaptersEdit {
    fn localize_with_args(
        &self,
        manager: &crate::localization::LocaleManager,
        _args: &fluent::FluentArgs,
    ) -> String {
        self.0.new.localize(manager)
    }
}

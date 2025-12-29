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

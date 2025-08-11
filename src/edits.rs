use std::fmt::Display;

use crate::{
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

impl Display for ClearStoryChapters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClearStoryChapters::ClearStage(clear_stage_options) => format!("clear-stage"),
                ClearStoryChapters::ClearChapter(clear_chapter_options) => format!("clear-chapter"),
                ClearStoryChapters::ClearAll(clear_all_chapters_options) =>
                    format!("clear-all-chapters"),
            }
        )
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

impl Display for StoryChaptersEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.new.fmt(f)
    }
}

type EditMemoryi32 = EditMemory<i32, i32>;

#[derive(Debug, Clone)]
pub struct CatfoodEdit(pub EditMemoryi32);

impl EditReadable for CatfoodEdit {
    fn read(save_file: &SaveFile) -> Self {
        Self(EditMemory::init_same(save_file.save.catfood))
    }
}

impl Applyable for CatfoodEdit {
    fn apply(&self, save_file: &mut SaveFile) {
        save_file.save.catfood = self.0.new;
    }
    fn revert(&self, save_file: &mut SaveFile) {
        save_file.save.catfood = self.0.old;
    }
}

impl From<CatfoodEdit> for Edit {
    fn from(value: CatfoodEdit) -> Self {
        Self::Catfood(value)
    }
}

impl Display for CatfoodEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XPEdit(pub EditMemoryi32);

#[derive(Debug, Clone, Copy)]
pub struct EditMemory<N, O> {
    pub new: N,
    pub old: O,
}

impl<N, O> EditMemory<N, O> {
    pub fn new(new: N, old: O) -> Self {
        Self { new, old }
    }

    pub fn init_same(val: N) -> Self
    where
        N: Into<O> + Clone,
    {
        Self::new(val.clone(), val.into())
    }
    pub fn swap(self) -> Self
    where
        N: Into<O>,
        O: Into<N>,
    {
        let tmp = self.new;
        Self {
            new: self.old.into(),
            old: tmp.into(),
        }
    }
}

impl<N: Display, O: Display> Display for EditMemory<N, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.old, self.new)
    }
}

impl EditReadable for XPEdit {
    fn read(save_file: &SaveFile) -> Self {
        Self(EditMemory::init_same(save_file.save.xp))
    }
}

impl Applyable for XPEdit {
    fn apply(&self, save_file: &mut SaveFile) {
        save_file.save.xp = self.0.new;
    }
    fn revert(&self, save_file: &mut SaveFile) {
        save_file.save.xp = self.0.old;
    }
}

impl From<XPEdit> for Edit {
    fn from(value: XPEdit) -> Self {
        Self::XP(value)
    }
}

impl Display for XPEdit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub enum Edit {
    Catfood(CatfoodEdit),
    XP(XPEdit),
    MainStory(StoryChaptersEdit),
}

impl Display for Edit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        macro_rules! to_string {
            [$($var:ident),+] => {
                match self {
                    $(Self::$var(v) => v.to_string(),)+
                }
            };
        }
        write!(f, "{}", to_string![Catfood, XP, MainStory])
    }
}

impl Edit {
    pub fn get_name(&self) -> String {
        macro_rules! get_name {
            [$($var:ident => $name:literal),+] => {
                match self {
                    $(Self::$var(_) => $name,)+
                }
            };
        }

        get_name![
            Catfood => "catfood",
            XP => "xp",
            MainStory => "main-story"
        ]
        .to_string()
    }
}

pub trait EditReadable {
    fn read(save_file: &SaveFile) -> Self;
}

pub trait Applyable {
    fn apply(&self, save_file: &mut SaveFile);
    fn revert(&self, save_file: &mut SaveFile);
}

impl Applyable for Edit {
    fn apply(&self, save_file: &mut SaveFile) {
        macro_rules! apply {
            [$($var:ident),+] => {
                match self {
                    $(Self::$var(v) => v.apply(save_file),)+
                }
            };
        }
        apply![Catfood, XP, MainStory]
    }
    fn revert(&self, save_file: &mut SaveFile) {
        macro_rules! revert {
            [$($var:ident),+] => {
                match self {
                    $(Self::$var(v) => v.revert(save_file),)+
                }
            };
        }
        revert![Catfood, XP, MainStory]
    }
}

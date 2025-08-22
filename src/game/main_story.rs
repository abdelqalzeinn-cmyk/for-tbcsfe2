use std::str::FromStr;

use bcsfe_derive::{Readable, Writable};

pub const TOTAL_STORY_CHAPTERS: usize = 10;
pub const TOTAL_CLEAR_TIME_STAGES: usize = 51;
pub const TOTAL_STORY_STAGES: usize = 49;
pub const TOTAL_INGAME_STAGES: usize = 48;

#[derive(Debug, Copy, Clone, Readable, Writable)]
pub struct StoryChapters {
    pub selected_stages: [i32; TOTAL_STORY_CHAPTERS],
    pub chapter_progress: [i32; TOTAL_STORY_CHAPTERS],
    pub clear_times: [[i32; TOTAL_CLEAR_TIME_STAGES]; TOTAL_STORY_CHAPTERS],
    pub treasures: [[i32; TOTAL_STORY_STAGES]; TOTAL_STORY_CHAPTERS],
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord, Default)]
pub enum InnerChapterType {
    #[default]
    First,
    Second,
    Third,
}

impl FromStr for InnerChapterType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "first" => Self::First,
            "second" => Self::Second,
            "third" => Self::Third,
            _ => return Err("invalid inner chapter type".to_string()),
        })
    }
}

impl InnerChapterType {
    pub fn into_usize(self) -> usize {
        self.into()
    }
}

impl From<InnerChapterType> for usize {
    fn from(value: InnerChapterType) -> Self {
        match value {
            InnerChapterType::First => 0,
            InnerChapterType::Second => 1,
            InnerChapterType::Third => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum StoryChapterType {
    Eoc(InnerChapterType),
    Itf(InnerChapterType),
    Cotc(InnerChapterType),
}

impl std::fmt::Display for StoryChapterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chapter-{}", self.into_human_chapter_index() + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, PartialOrd, Ord, Default)]
pub enum StoryChapterTypeOuter {
    #[default]
    Eoc,
    Itf,
    Cotc,
}

impl FromStr for StoryChapterTypeOuter {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "eoc" => Self::Eoc,
            "itf" => Self::Itf,
            "cotc" => Self::Cotc,
            _ => return Err("invalid story chapter type".to_string()),
        })
    }
}

impl std::fmt::Display for StoryChapterTypeOuter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StoryChapterTypeOuter::Eoc => "eoc",
                StoryChapterTypeOuter::Itf => "itf",
                StoryChapterTypeOuter::Cotc => "cotc",
            }
        )
    }
}

impl Default for StoryChapterType {
    fn default() -> Self {
        Self::Eoc(InnerChapterType::default())
    }
}

impl StoryChapterType {
    pub fn into_outer(self) -> StoryChapterTypeOuter {
        match self {
            StoryChapterType::Eoc(_) => StoryChapterTypeOuter::Eoc,
            StoryChapterType::Itf(_) => StoryChapterTypeOuter::Itf,
            StoryChapterType::Cotc(_) => StoryChapterTypeOuter::Cotc,
        }
    }
}

impl StoryChapterTypeOuter {
    pub fn into_main(self, inner: InnerChapterType) -> StoryChapterType {
        match self {
            StoryChapterTypeOuter::Eoc => StoryChapterType::Eoc(inner),
            StoryChapterTypeOuter::Itf => StoryChapterType::Itf(inner),
            StoryChapterTypeOuter::Cotc => StoryChapterType::Cotc(inner),
        }
    }
}

impl StoryChapterType {
    pub const ALL: [Self; 9] = [
        Self::Eoc(InnerChapterType::First),
        Self::Eoc(InnerChapterType::Second),
        Self::Eoc(InnerChapterType::Third),
        Self::Itf(InnerChapterType::First),
        Self::Itf(InnerChapterType::Second),
        Self::Itf(InnerChapterType::Third),
        Self::Cotc(InnerChapterType::First),
        Self::Cotc(InnerChapterType::Second),
        Self::Cotc(InnerChapterType::Third),
    ];

    pub fn from_usize_human(i: usize) -> Option<StoryChapterType> {
        Some(match i {
            0 => Self::Eoc(InnerChapterType::First),
            1 => Self::Eoc(InnerChapterType::Second),
            2 => Self::Eoc(InnerChapterType::Third),
            3 => Self::Itf(InnerChapterType::First),
            4 => Self::Itf(InnerChapterType::Second),
            5 => Self::Itf(InnerChapterType::Third),
            6 => Self::Cotc(InnerChapterType::First),
            7 => Self::Cotc(InnerChapterType::Second),
            8 => Self::Cotc(InnerChapterType::Third),
            _ => return None,
        })
    }
}

impl StoryChapterType {
    pub fn into_base_chapter_index(self) -> usize {
        match self {
            StoryChapterType::Eoc(_) => 0,
            // gap between eoc and itf for some reason
            StoryChapterType::Itf(_) => 4,
            StoryChapterType::Cotc(_) => 7,
        }
    }
    pub fn into_human_base_chapter_index(self) -> usize {
        match self {
            StoryChapterType::Eoc(_) => 0,
            StoryChapterType::Itf(_) => 3,
            StoryChapterType::Cotc(_) => 6,
        }
    }

    pub fn into_local_chapter_index(self) -> usize {
        match self {
            StoryChapterType::Eoc(inner_chapter_type) => inner_chapter_type.into(),
            StoryChapterType::Itf(inner_chapter_type) => inner_chapter_type.into(),
            StoryChapterType::Cotc(inner_chapter_type) => inner_chapter_type.into(),
        }
    }

    pub fn into_chapter_index(self) -> usize {
        self.into_base_chapter_index() + self.into_local_chapter_index()
    }

    pub fn into_human_chapter_index(self) -> usize {
        self.into_human_base_chapter_index() + self.into_local_chapter_index()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StageId(u8);

impl StageId {
    pub fn previous(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(Self(self.0 - 1))
        }
    }
    pub fn previous_saturating(self) -> Self {
        self.previous().unwrap_or(self)
    }
    pub fn next(self) -> Option<Self> {
        Self::new(self.0 + 1)
    }
    pub fn next_saturating(self) -> Self {
        self.next().unwrap_or(self)
    }
}

impl FromStr for StageId {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: u8 = s
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;

        Self::new(id.saturating_sub(1)).ok_or("invalid stage id".to_string())
    }
}

impl From<StageId> for u8 {
    fn from(value: StageId) -> Self {
        value.0
    }
}
impl From<StageId> for usize {
    fn from(value: StageId) -> Self {
        value.0 as usize
    }
}
impl TryFrom<usize> for StageId {
    type Error = String;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::new(value as u8).ok_or(format!("invalid stage id: {value}"))
    }
}
impl From<StageId> for i32 {
    fn from(value: StageId) -> Self {
        value.0 as i32
    }
}

impl StageId {
    pub fn new(id: u8) -> Option<Self> {
        if id >= (TOTAL_STORY_STAGES as u8) {
            None
        } else {
            Some(Self(id))
        }
    }

    pub fn into_usize(self) -> usize {
        self.into()
    }

    pub fn iter_to_end(self) -> impl DoubleEndedIterator<Item = Self> {
        (self.0..(TOTAL_STORY_STAGES as u8)).map(|id| Self(id))
    }
    pub fn iter_from_start(self) -> impl DoubleEndedIterator<Item = Self> {
        (0..=(self.0)).map(|id| Self(id))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreasureValue {
    None,
    Inferior,
    Normal,
    Superior,
    Custom(i32),
}

impl From<TreasureValue> for i32 {
    fn from(value: TreasureValue) -> Self {
        match value {
            TreasureValue::None => 0,
            TreasureValue::Inferior => 1,
            TreasureValue::Normal => 2,
            TreasureValue::Superior => 3,
            TreasureValue::Custom(v) => v,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StoryStage {
    pub chapter: StoryChapterType,
    pub stage_id: StageId,
}

impl StoryStage {
    pub fn new(chapter: StoryChapterType, stage_id: StageId) -> Self {
        Self { chapter, stage_id }
    }

    #[cfg(feature = "localization")]
    pub fn localize_stage(&self, manager: &crate::localization::LocaleManager) -> String {
        use crate::localization::Localizable;

        format!(
            "chapter-{}-stage-{}",
            self.chapter.into_outer(),
            self.stage_id.into_usize() + 1
        )
        .localize(manager)
    }

    pub fn previous(&self) -> Option<StoryStage> {
        self.stage_id.previous().map(|v| Self {
            chapter: self.chapter,
            stage_id: v,
        })
    }
    pub fn next(&self) -> Option<StoryStage> {
        self.stage_id.next().map(|v| Self {
            chapter: self.chapter,
            stage_id: v,
        })
    }

    pub fn previous_saturating(&self) -> Self {
        Self {
            chapter: self.chapter,
            stage_id: self.stage_id.previous_saturating(),
        }
    }

    fn next_saturating(&self) -> StoryStage {
        Self {
            chapter: self.chapter,
            stage_id: self.stage_id.next_saturating(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClearStageOptions {
    pub stage: StoryStage,
    pub clear_amount: i32,
    pub progress_type: ProgressType,
}

impl ClearStageOptions {
    pub fn with_clear_amount(self, clear_amount: i32) -> Self {
        Self {
            stage: self.stage,
            clear_amount,
            progress_type: self.progress_type,
        }
    }

    pub fn with_chapter(self, chapter: StoryChapterType) -> Self {
        Self {
            stage: StoryStage {
                chapter,
                stage_id: self.stage.stage_id,
            },
            clear_amount: self.clear_amount,
            progress_type: self.progress_type,
        }
    }
    pub fn with_stage_id(self, stage_id: StageId) -> Self {
        Self {
            stage: StoryStage {
                chapter: self.stage.chapter,
                stage_id,
            },
            clear_amount: self.clear_amount,
            progress_type: self.progress_type,
        }
    }
    pub fn with_stage(self, stage: StoryStage) -> Self {
        Self {
            stage,
            clear_amount: self.clear_amount,
            progress_type: self.progress_type,
        }
    }
    pub fn with_progress_type(self, progress: ProgressType) -> Self {
        Self {
            stage: self.stage,
            clear_amount: self.clear_amount,
            progress_type: progress,
        }
    }
}

impl Default for ClearStageOptions {
    fn default() -> Self {
        Self {
            stage: StoryStage::default(),
            clear_amount: 1,
            progress_type: ProgressType::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClearChapterOptions {
    pub chapter: StoryChapterType,
    pub clear_amount: i32,
}

impl ClearChapterOptions {
    pub fn with_clear_amount(self, clear_amount: i32) -> Self {
        Self {
            chapter: self.chapter,
            clear_amount,
        }
    }

    pub fn with_chapter(self, chapter: StoryChapterType) -> Self {
        Self {
            chapter,
            clear_amount: self.clear_amount,
        }
    }
}

impl Default for ClearChapterOptions {
    fn default() -> Self {
        Self {
            chapter: StoryChapterType::default(),
            clear_amount: 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ProgressType {
    #[default]
    AlwaysSetProgress,
    OnlySetProgressIfLaterStage,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClearAllChaptersOptions {
    pub clear_amount: i32,
}

impl Default for ClearAllChaptersOptions {
    fn default() -> Self {
        Self { clear_amount: 1 }
    }
}

impl ClearAllChaptersOptions {
    pub fn with_clear_amount(self, clear_amount: i32) -> Self {
        Self { clear_amount }
    }
}

impl StoryChapters {
    pub fn clear_all(&mut self, opts: ClearAllChaptersOptions) {
        for chapter in StoryChapterType::ALL {
            self.clear_chapter(ClearChapterOptions {
                chapter,
                clear_amount: opts.clear_amount,
            });
        }
    }
    pub fn clear_chapter(&mut self, opts: ClearChapterOptions) {
        for stage_ind in 0..TOTAL_STORY_STAGES {
            self.clear_stage(ClearStageOptions {
                stage: StoryStage {
                    chapter: opts.chapter,
                    stage_id: StageId::new(stage_ind as u8)
                        .expect("TOTAL_STORY_STAGES is the same"),
                },
                clear_amount: opts.clear_amount,
                progress_type: ProgressType::AlwaysSetProgress,
            });
        }
    }
    pub fn clear_stage(&mut self, opts: ClearStageOptions) {
        let chap_progress = if opts.clear_amount == 0 {
            opts.stage
        } else {
            opts.stage.next_saturating()
        };
        match opts.progress_type {
            ProgressType::AlwaysSetProgress => self.set_chapter_progress(chap_progress),
            ProgressType::OnlySetProgressIfLaterStage => {
                self.add_chapter_progress(chap_progress);
            }
        };
        self.set_clear_amount(opts.stage, opts.clear_amount);
    }
    pub fn add_chapter_progress(&mut self, stage: StoryStage) -> bool {
        let current = self.get_chapter_progress(stage.chapter);

        let stage_id: i32 = stage.stage_id.into();

        if stage_id > current {
            self.set_chapter_progress(stage);
            true
        } else {
            false
        }
    }
    pub fn set_chapter_progress(&mut self, stage: StoryStage) {
        let progress = self
            .chapter_progress
            .get_mut(stage.chapter.into_chapter_index())
            .expect("chapter index was correctly calculated");

        *progress = stage.stage_id.into();
    }
    pub fn get_chapter_progress(&self, chapter: StoryChapterType) -> i32 {
        *self
            .chapter_progress
            .get(chapter.into_chapter_index())
            .expect("chapter index was correctly calculated")
    }
    pub fn get_clear_amount(&self, stage: StoryStage) -> i32 {
        let stages = self
            .clear_times
            .get(stage.chapter.into_chapter_index())
            .expect("chapter index was correctly calculated");

        *stages
            .get(stage.stage_id.into_usize())
            .expect("stage id was correctly bounded")
    }

    pub fn add_clear_amount(&mut self, stage: StoryStage, amount: i32) {
        if self.get_clear_amount(stage) == 0 {
            self.set_clear_amount(stage, amount);
        }
    }
    pub fn set_clear_amount(&mut self, stage: StoryStage, amount: i32) {
        let stages = self
            .clear_times
            .get_mut(stage.chapter.into_chapter_index())
            .expect("chapter index was correctly calculated");

        let current = stages
            .get_mut(stage.stage_id.into_usize())
            .expect("stage id was correctly bounded");

        *current = amount;
    }
    pub fn set_treasure(&mut self, stage: StoryStage, treasure_value: TreasureValue) {
        let stages = self
            .treasures
            .get_mut(stage.chapter.into_chapter_index())
            .expect("chapter index was correctly calculated"); // should never occur

        let current = stages
            .get_mut(stage.stage_id.into_usize())
            .expect("stage id was correctly bounded"); // should never happen

        *current = treasure_value.into();
    }
}

impl Default for StoryChapters {
    fn default() -> Self {
        Self {
            selected_stages: [0; TOTAL_STORY_CHAPTERS],
            chapter_progress: [0; TOTAL_STORY_CHAPTERS],
            clear_times: [[0; TOTAL_CLEAR_TIME_STAGES]; TOTAL_STORY_CHAPTERS],
            treasures: [[0; TOTAL_STORY_STAGES]; TOTAL_STORY_CHAPTERS],
        }
    }
}

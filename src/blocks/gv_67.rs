use bcsfe_derive::{Readable, Writable};

use crate::{
    save::{ChaptersGeneric, GenericChapterArgs, StageClear},
    stream::{LengthString, LengthVec, Readable, StreamResult, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 67)]
pub struct GV67Block {
    #[rw(gvcc)]
    pub ranking: DojoRanking,
    pub item_pack_three_days_started: bool,
    pub item_pack_three_days_end: f64,
    pub challenge: ChallengeChapters,
    pub challenge_scores: LengthVec<i32, i32>,
    pub show_challenge_popup: bool,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DojoRanking {
    pub score: i32,
    pub ranking: i32,
    pub has_submitted: bool,
    pub has_completed: bool,
    pub has_seen_results: bool,
    pub start_date: i32,
    pub end_date: i32,
    pub event_number: i32,
    pub should_show_rank_description: bool,
    pub should_show_start_message: bool,
    pub submit_error_flag: bool,
    #[rw(min_gv = 140500)]
    pub other: Option<LengthString<i32>>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChallengeChapters {
    pub chapters: ChaptersGeneric<i32, i32, StageClear<i32>, i32>,
}

impl Readable for ChallengeChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(Self {
            chapters: ChaptersGeneric::read(reader, GenericChapterArgs::new_int(true))?,
        })
    }
}

impl Writable for ChallengeChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(true))
    }
}

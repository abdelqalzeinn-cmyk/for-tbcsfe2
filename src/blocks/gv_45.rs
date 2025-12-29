use crate::{
    game::main_story::TOTAL_CLEAR_TIME_STAGES,
    stream::{LengthVec, Readable, Writable, WritableNoOptions},
};
use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Readable, Writable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 45)]
pub struct GV45Block {
    pub itf1_complete: i32,
    pub itf_timed_scores: [StaticChapter<TOTAL_CLEAR_TIME_STAGES>; 3],
    pub title_chapter_bg: i32,
    #[rw(min_gv = 27)]
    pub combo_unlocks: Option<LengthVec<i32, i32>>,
    pub combo_unlocked_10k_ur: bool,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StaticChapter<const N: usize> {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    pub data: [i32; N],
}

impl<const N: usize> Readable for StaticChapter<N> {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(Self {
            data: <[i32; N]>::read(reader, ())?,
        })
    }
}

impl<const N: usize> Writable for StaticChapter<N> {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<()> {
        self.data.write_no_opts(writer)
    }
}

impl Default for GV45Block {
    fn default() -> Self {
        Self {
            itf_timed_scores: [StaticChapter { data: [0; 51] }; 3],
            itf1_complete: 0,
            title_chapter_bg: 0,
            combo_unlocks: None,
            combo_unlocked_10k_ur: false,
        }
    }
}

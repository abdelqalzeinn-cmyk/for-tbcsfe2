use bcsfe_derive::{Readable, Writable};

use crate::{
    save::{ChaptersGeneric, GenericChapterArgs, LengthType, StageClear},
    stream::{Assertable, HashMapLength, LengthVec, Readable, StreamResult, VecArgs, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV90300Block {
    pub unknown: LengthVec<i16, Unknown90300>,
    pub unknown_2: HashMapLength<i16, i32, f64>,
    pub gauntlet_chapters: GauntletChapters,
    pub _90300: Assertable<90300>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GauntletChapters {
    pub chapters: ChaptersGeneric<i8, i8, StageClear<i16>, i8>,
    pub unknown: Vec<i8>,
}

impl Readable for GauntletChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let chapters = ChaptersGeneric::read(
            reader,
            GenericChapterArgs {
                read_length_every_time: false,
                total_chapters_type: LengthType::I16,
                total_stages_type: LengthType::I8,
                total_stars_type: LengthType::I8,
            },
        )?;

        let total_chapters = chapters.total_chapters();

        Ok(Self {
            chapters,
            unknown: Vec::read(reader, VecArgs::new_empty_fixed(total_chapters))?,
        })
    }
}

impl Writable for GauntletChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters.write(
            writer,
            GenericChapterArgs {
                read_length_every_time: false,
                total_chapters_type: LengthType::I16,
                total_stages_type: LengthType::I8,
                total_stars_type: LengthType::I8,
            },
        )?;

        self.unknown.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.total_chapters()),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unknown90300 {
    pub u1: i32,
    pub u2: i32,
    pub u3: i16,
    pub u4: i32,
    pub u5: i32,
    pub u6: i32,
    pub u7: i16,
}

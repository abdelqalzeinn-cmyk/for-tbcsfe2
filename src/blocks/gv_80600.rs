use bcsfe_derive::{Readable, Writable};

use crate::{
    save::{ChaptersGeneric, GenericChapterArgs, LengthType, TotalStages},
    stream::{
        Assertable, LengthVec, Readable, ReadableNoOptions, StreamResult, VecArgs, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV80600Block {
    pub unknown_vec: LengthVec<i16, i32>,
    pub legend_quest_chapters: LegendQuest,
    pub uknown_short: i16,
    pub unknown_byte: i8,
    pub _80600: Assertable<80600>,
}

impl TotalStages for LegendQuestStage {
    fn total(&self) -> usize {
        self.clear_times.len()
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegendQuestStage {
    pub clear_times: Vec<Vec<i16>>,
    pub attemps: Vec<Vec<i16>>,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegendQuestStageArgs {
    pub total_stages: usize,
    pub total_stars: usize,
}

impl Readable for LegendQuestStage {
    type Args<'a> = VecArgs<VecArgs<()>>;

    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        // let args = VecArgs {
        //     length: VecArgsLength::Fixed(args.total_stages),
        //     item: VecArgs::new_empty_fixed(args.total_stars),
        // };
        Ok(Self {
            clear_times: Vec::read(reader, args)?,
            attemps: Vec::read(reader, args)?,
        })
    }
}

impl Writable for LegendQuestStage {
    type Args<'a> = VecArgs<VecArgs<()>>;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.clear_times.write(writer, args)?;
        self.attemps.write(writer, args)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegendQuestChapters {
    pub chapters: ChaptersGeneric<i8, i8, LegendQuestStage, i8>,
}

impl Readable for LegendQuestChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        Ok(Self {
            chapters: ChaptersGeneric::read(
                reader,
                GenericChapterArgs {
                    read_length_every_time: false,
                    total_chapters_type: LengthType::I8,
                    total_stages_type: LengthType::I8,
                    total_stars_type: LengthType::I8,
                },
            )?,
        })
    }
}

impl Writable for LegendQuestChapters {
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
                total_chapters_type: LengthType::I8,
                total_stages_type: LengthType::I8,
                total_stars_type: LengthType::I8,
            },
        )
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegendQuest {
    pub chapters: LegendQuestChapters,
    pub unknown: Vec<i8>,
    pub ids: Vec<i32>,
}

impl Readable for LegendQuest {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let chapters = LegendQuestChapters::read_no_opts(reader)?;
        let total_chapters = chapters.chapters.total_chapters();
        let total_stages = chapters.chapters.total_stages();

        Ok(Self {
            chapters,
            unknown: Vec::read(reader, VecArgs::new_empty_fixed(total_chapters))?,
            ids: Vec::read(reader, VecArgs::new_empty_fixed(total_stages))?,
        })
    }
}

impl Writable for LegendQuest {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters.write_no_opts(writer)?;

        self.unknown.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.chapters.total_chapters()),
        )?;
        self.ids.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.chapters.total_stages()),
        )?;

        Ok(())
    }
}

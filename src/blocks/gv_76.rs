use bcsfe_derive::{Readable, Writable};

use crate::{
    save::{ChaptersGeneric, GenericChapterArgs, StageClear},
    stream::{Assertable, Readable, StreamResult, VecArgs, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV76Block {
    pub uncanny_chapters: UncannyChapters,
    pub _76: Assertable<76>,
}

#[derive(Debug, Clone, Default)]
pub struct UncannyChapters {
    pub chapters: ChaptersGeneric<i32, i32, StageClear<i32>, i32>,
    pub unknown: Vec<i32>,
}

impl Readable for UncannyChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let chapters = ChaptersGeneric::read(reader, GenericChapterArgs::new_int(false))?;
        let len = chapters.selected_stages.len();
        Ok(Self {
            chapters,
            unknown: Vec::read(reader, VecArgs::new_empty_fixed(len))?,
        })
    }
}

impl Writable for UncannyChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(false))?;

        self.unknown.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.selected_stages.len()),
        )
    }
}

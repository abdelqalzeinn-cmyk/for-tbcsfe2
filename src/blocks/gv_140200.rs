use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_67::DojoRanking,
    save::GVCC,
    stream::{Assertable, HashMapLength, Readable, StreamResult, VecArgs, VecArgsLength, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV140200Block {
    #[rw(gvcc)]
    pub dojo_ranking_2: DojoRanking2,
    pub unknown: HashMapLength<i8, i32, f64>,
    pub hundred_million_ticket: i32,
    pub _140200: Assertable<140200>,
}

#[derive(Debug, Clone, Default)]
pub struct DojoRanking2 {
    pub ranking: Vec<DojoRank2>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct DojoRank2 {
    #[rw(gvcc)]
    pub ranking: DojoRanking,
    pub unknown: bool,
}

impl Readable for DojoRanking2 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        Ok(Self {
            ranking: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::I8,
                    item: args,
                },
            )?,
        })
    }
}

impl Writable for DojoRanking2 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.ranking.write(
            writer,
            VecArgs {
                length: VecArgsLength::I8,
                item: args,
            },
        )?;

        Ok(())
    }
}

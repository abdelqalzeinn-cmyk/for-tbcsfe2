use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_67::DojoRanking,
    save::GVCC,
    stream::{HashMapLength, Readable, StreamResult, VecArgs, VecArgsLength, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 140200)]
pub struct GV140200Block {
    #[rw(gvcc, with = "DojoRanking2")]
    pub dojo_ranking_2: Vec<DojoRank2>,
    #[rw(with = "HashMapLength<i8, i32, f64>")]
    pub unknown: HashMap<i32, f64>,
    pub hundred_million_ticket: i32,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DojoRanking2 {
    pub ranking: Vec<DojoRank2>,
}

impl From<Vec<DojoRank2>> for DojoRanking2 {
    fn from(value: Vec<DojoRank2>) -> Self {
        Self { ranking: value }
    }
}

impl From<DojoRanking2> for Vec<DojoRank2> {
    fn from(value: DojoRanking2) -> Self {
        value.ranking
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        self,
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

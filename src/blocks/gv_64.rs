use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        HashMapLength, LengthVec, Readable, ReadableNoOptions, StreamResult, VecArgs, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 64)]
pub struct GV64Block {
    #[rw(with = "LengthVec<i32, i32>")]
    pub base_materials: Vec<i32>,
    #[rw(gvcc)]
    pub ototo: Ototo,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ototo {
    pub remaining_seconds: f64,
    pub return_flag: bool,
    pub improve_id: i32,
    pub engineers: i32,
    #[rw(with = "HashMapLength<i32, i32, LengthVec<i32, i32>>")]
    pub cannon_levels: HashMap<i32, Vec<i32>>,
    #[rw(gvcc)]
    pub selected_parts: OtotoSelectedParts,
    pub last_checked_castle_time: f64,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OtotoSelectedParts {
    Old([i32; 3]),
    New(Vec<[i8; 3]>),
}

impl Default for OtotoSelectedParts {
    fn default() -> Self {
        Self::New(Vec::new())
    }
}

impl Readable for OtotoSelectedParts {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..80200 => Ok(Self::Old(<[i32; 3]>::read_no_opts(reader)?)),
            80200..90700 => Ok(Self::New(Vec::read(reader, VecArgs::new_empty_fixed(10))?)),
            _ => Ok(Self::New(Vec::read(reader, VecArgs::new_empty_i8())?)),
        }
    }
}

impl Writable for OtotoSelectedParts {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..80200 => match self {
                OtotoSelectedParts::Old(o) => o.write_no_opts(writer)?,
                OtotoSelectedParts::New(items) => {
                    items.first().unwrap_or(&[0; 3]).write_no_opts(writer)?
                }
            },
            80200..90700 => match self {
                OtotoSelectedParts::Old(o) => {
                    o.to_vec().write(writer, VecArgs::new_empty_fixed(10))?
                }
                OtotoSelectedParts::New(items) => {
                    items.write(writer, VecArgs::new_empty_fixed(10))?
                }
            },
            _ => match self {
                OtotoSelectedParts::Old(items) => {
                    items.to_vec().write(writer, VecArgs::new_empty_i8())?
                }
                OtotoSelectedParts::New(items) => items.write(writer, VecArgs::new_empty_i8())?,
            },
        };
        Ok(())
    }
}

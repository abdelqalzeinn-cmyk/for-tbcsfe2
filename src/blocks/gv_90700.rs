use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        HashMapLength, Readable, ReadableNoOptions, StreamResult, Writable, WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 90700)]
pub struct GV90700Block {
    #[rw(gvcc)]
    pub talent_orbs: TalentOrbs,
    #[rw(with = "HashMapLength<i16, i16, HashMapLength<i8, i8, i16>>")]
    pub unknown: HashMap<i16, HashMap<i8, i16>>,
    pub unknown_2: bool,
}

impl Default for TalentOrbs {
    fn default() -> Self {
        Self::New(HashMap::default())
    }
}

impl Readable for TalentOrbs {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        match args.gv.0 {
            0..110400 => Ok(Self::Old(
                <HashMapLength<i16, i16, i8>>::read_no_opts(reader)?.into(),
            )),
            _ => Ok(Self::New(
                <HashMapLength<i16, i16, i16>>::read_no_opts(reader)?.into(),
            )),
        }
    }
}

impl Writable for TalentOrbs {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..110400 => match self {
                TalentOrbs::Old(hash_map_length) => {
                    <HashMapLength<i16, i16, i8>>::write_no_opts(hash_map_length.into(), writer)?
                }
                TalentOrbs::New(hash_map_length) => {
                    let other: HashMap<i16, i8> = hash_map_length
                        .into_iter()
                        .map(|(k, v)| (k, v as i8))
                        .collect();

                    <HashMapLength<i16, i16, i8>>::write_no_opts(other.into(), writer)?;
                }
            },
            _ => match self {
                TalentOrbs::Old(hash_map_length) => {
                    let other: HashMap<i16, i16> = hash_map_length
                        .into_iter()
                        .map(|(k, v)| (k, v as i16))
                        .collect();

                    <HashMapLength<i16, i16, i16>>::write_no_opts(other.into(), writer)?;
                }
                TalentOrbs::New(hash_map_length) => {
                    <HashMapLength<i16, i16, i16>>::write_no_opts(hash_map_length.into(), writer)?
                }
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TalentOrbs {
    Old(HashMap<i16, i8>),
    New(HashMap<i16, i16>),
}

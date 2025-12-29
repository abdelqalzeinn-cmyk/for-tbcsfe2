use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        Assertable, HashMapLength, Readable, ReadableNoOptions, StreamResult, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV90700Block {
    #[rw(gvcc)]
    pub talent_orbs: TalentOrbs,
    pub unknown: HashMapLength<i16, i16, HashMapLength<i8, i8, i16>>,
    pub unknown_2: bool,
    pub _90700: Assertable<90700>,
}

impl Default for TalentOrbs {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for TalentOrbs {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        match args.gv.0 {
            0..110400 => Ok(Self::Old(HashMapLength::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl Writable for TalentOrbs {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..110400 => match self {
                TalentOrbs::Old(hash_map_length) => hash_map_length.write_no_opts(writer)?,
                TalentOrbs::New(hash_map_length) => {
                    let other: HashMapLength<i16, i16, i8> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
            },
            _ => match self {
                TalentOrbs::Old(hash_map_length) => {
                    let other: HashMapLength<i16, i16, i16> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
                TalentOrbs::New(hash_map_length) => hash_map_length.write_no_opts(writer)?,
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TalentOrbs {
    Old(HashMapLength<i16, i16, i8>),
    New(HashMapLength<i16, i16, i16>),
}

impl From<&HashMapLength<i16, i16, i8>> for HashMapLength<i16, i16, i16> {
    fn from(value: &HashMapLength<i16, i16, i8>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i16);
        }

        Self::new(new_map)
    }
}
impl From<&HashMapLength<i16, i16, i16>> for HashMapLength<i16, i16, i8> {
    fn from(value: &HashMapLength<i16, i16, i16>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i8);
        }

        Self::new(new_map)
    }
}

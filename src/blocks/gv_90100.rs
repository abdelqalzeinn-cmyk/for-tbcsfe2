use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        HashMapLength, Readable, ReadableNoOptions, StreamResult, Writable, WritableNoOptions,
    },
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EventStartTimes90100 {
    Old(HashMapLength<i16, i16, f64>),
    New(HashMapLength<i16, i16, i32>),
}

impl Default for EventStartTimes90100 {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for EventStartTimes90100 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        match args.gv.0 {
            0..90100 => Ok(Self::Old(HashMapLength::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl From<&HashMapLength<i16, i16, f64>> for HashMapLength<i16, i16, i32> {
    fn from(value: &HashMapLength<i16, i16, f64>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i32);
        }

        Self::new(new_map)
    }
}
impl From<&HashMapLength<i16, i16, i32>> for HashMapLength<i16, i16, f64> {
    fn from(value: &HashMapLength<i16, i16, i32>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as f64);
        }

        Self::new(new_map)
    }
}

impl Writable for EventStartTimes90100 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..90100 => match self {
                EventStartTimes90100::Old(hash_map_length) => {
                    hash_map_length.write_no_opts(writer)?
                }
                EventStartTimes90100::New(hash_map_length) => {
                    let other: HashMapLength<i16, i16, f64> = hash_map_length.into();
                    other.write_no_opts(writer)?;
                }
            },
            _ => match self {
                EventStartTimes90100::Old(hash_map_length) => {
                    let other: HashMapLength<i16, i16, i32> = hash_map_length.into();
                    other.write_no_opts(writer)?;
                }
                EventStartTimes90100::New(hash_map_length) => {
                    hash_map_length.write_no_opts(writer)?
                }
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 90100)]
pub struct GV90100Block {
    pub unknown_1: i16,
    pub unknown_2: i16,
    pub unknown_date: i32,
    pub unknown_timestamp: f64,
}

use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        HashMapLength, LengthVec, Readable, ReadableNoOptions, StreamResult, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 55)]
pub struct GV55Block {
    pub gamatoto_skin: i32,
    pub platinum_tickets: i32,
    #[rw(gvcc)]
    pub logins: LoginBonus,
    #[rw(max_gv = 100999, with = "LengthVec<i32, bool>")]
    pub reset_item_reward_flags: Vec<bool>,
    pub reward_remaining_time: f64,
    pub last_checked_reward_time: f64,
    pub announcements: [(i32, i32); 16],
    pub backup_counter: i32,
    pub uknown: [i32; 3],
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoginBonus {
    Old(Vec<i32>),
    New(HashMap<i32, i32>),
}

impl Default for LoginBonus {
    fn default() -> Self {
        Self::New(HashMap::default())
    }
}

impl Readable for LoginBonus {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..80000 => Ok(Self::Old(
                <LengthVec<i32, i32>>::read_no_opts(reader)?.into(),
            )),
            _ => Ok(Self::New(
                <HashMapLength<i32, i32, i32>>::read_no_opts(reader)?.into(),
            )),
        }
    }
}

impl Writable for LoginBonus {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        if args.gv.0 < 80000 {
            match self {
                LoginBonus::Old(items) => {
                    <LengthVec<i32, i32>>::write_no_opts(items.into(), writer)?
                }
                LoginBonus::New(hash_map) => {
                    <LengthVec<i32, i32>>::write_no_opts(LengthVec::new(Vec::new()), writer)? // TODO: actually convert data
                }
            }
        } else {
            match self {
                LoginBonus::Old(items) => <HashMapLength<i32, i32, i32>>::write_no_opts(
                    HashMapLength::new(HashMap::new()), // TODO: actually convert data
                    writer,
                )?,
                LoginBonus::New(hash_map) => {
                    <HashMapLength<i32, i32, i32>>::write_no_opts(hash_map.into(), writer)?
                }
            }
        }

        Ok(())
    }
}

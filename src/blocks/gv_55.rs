use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        Assertable, HashMapLength, LengthVec, Readable, ReadableNoOptions, StreamResult, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV55Block {
    pub gamatoto_skin: i32,
    pub platinum_tickets: i32,
    #[rw(gvcc)]
    pub logins: LoginBonus,
    #[rw(max_gv = 100999)]
    pub reset_item_reward_flags: Option<LengthVec<i32, bool>>,
    pub reward_remaining_time: f64,
    pub last_checked_reward_time: f64,
    pub announcements: [(i32, i32); 16],
    pub backup_counter: i32,
    pub uknown: [i32; 3],
    pub _55: Assertable<55>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoginBonus {
    Old(LengthVec<i32, i32>),
    New(HashMapLength<i32, i32, i32>),
}

impl Default for LoginBonus {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for LoginBonus {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..80000 => Ok(Self::Old(LengthVec::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl Writable for LoginBonus {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        // we don't need to convert any incorrect data types here since they will write to
        // approximately the same thing anyway
        match self {
            LoginBonus::Old(length_vec) => length_vec.write_no_opts(writer)?,
            LoginBonus::New(hash_map_length) => hash_map_length.write_no_opts(writer)?,
        };

        Ok(())
    }
}

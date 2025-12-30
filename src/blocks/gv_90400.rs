use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    blocks::gv_90300::GauntletChapters,
    save::{Formi8, GVCC},
    stream::{
        HashMapLength, LengthVec, Readable, ReadableNoOptions, StreamResult, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 90400)]
pub struct GV90400Block {
    pub enigma_clears: GauntletChapters,
    #[rw(gvcc)]
    pub enigma: Engima,
    pub cleared_slots: ClearedSlots,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnigmaStage {
    pub level: i32,
    pub stage_id: i32,
    pub decoding_status: i8,
    pub start_time: f64,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtraEnigmaDataInner {
    pub u1: i32,
    pub u2: i32,
    pub u3: i8,
    pub u4: f64,
}

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtraEnigmaData(pub Option<ExtraEnigmaDataInner>);

impl Readable for ExtraEnigmaData {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        match args.gv.0 {
            0..140500 => Ok(Self(None)),
            _ => {
                let has_extra = bool::read_no_opts(reader)?;

                match has_extra {
                    true => Ok(Self(Some(ExtraEnigmaDataInner::read_no_opts(reader)?))),
                    false => Ok(Self(None)),
                }
            }
        }
    }
}

impl Writable for ExtraEnigmaData {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..140500 => (),
            _ => match self.0 {
                Some(item) => {
                    true.write_no_opts(writer)?;
                    item.write_no_opts(writer)?;
                }
                None => {
                    false.write_no_opts(writer)?;
                }
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Engima {
    pub energy_since_1: i32,
    pub energy_since_2: i32,
    pub enigma_level: i8,
    pub unknown_1: i8,
    pub unknown_2: bool,
    #[rw(with = "LengthVec<i8, EnigmaStage>")]
    pub stages: Vec<EnigmaStage>,
    #[rw(gvcc)]
    pub extra_data: ExtraEnigmaData,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatSlot {
    pub cat_id: i16,
    pub form: Formi8,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LineupCat {
    pub index: i16,
    pub cats: [CatSlot; 10],
    pub u1: i8,
    pub u2: i8,
    pub u3: i8,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StageLineup {
    pub index: i16,
    #[rw(with = "LengthVec<i16, i32>")]
    pub stages: Vec<i32>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClearedSlots {
    #[rw(with = "LengthVec<i16, LineupCat>")]
    pub cats: Vec<LineupCat>,
    #[rw(with = "LengthVec<i16, StageLineup>")]
    pub stages: Vec<StageLineup>,
    #[rw(with = "HashMapLength<i16, i16, bool>")]
    pub unknown: HashMap<i16, bool>,
}

use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::{ChaptersGeneric, GVCC, GenericChapterArgs, StageClear},
    stream::{
        Assertable, HashMapLength, Readable, ReadableNoOptions, StreamResult, VecArgs,
        VecArgsLength, Writable, WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV66Block {
    pub tower: TowerChapters,
    #[rw(gvcc)]
    pub missions: Missions,
    pub tower_item_obtain_states: TowerItemObtainStates,
    pub _66: Assertable<66>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TowerItemObtainStates {
    pub item_obtain_states: Vec<Vec<bool>>,
}

impl Readable for TowerItemObtainStates {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let total_stars = i32::read_no_opts(reader)?;
        let total_stages = i32::read_no_opts(reader)?;

        Ok(Self {
            item_obtain_states: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::Fixed(total_stars as usize),
                    item: VecArgs::new_empty_fixed(total_stages as usize),
                },
            )?,
        })
    }
}

impl Writable for TowerItemObtainStates {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_stars = self.item_obtain_states.len();
        let total_stages = self.item_obtain_states.first().unwrap_or(&Vec::new()).len();

        (total_stars as i32).write_no_opts(writer)?;
        (total_stages as i32).write_no_opts(writer)?;

        self.item_obtain_states.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_stars),
                item: VecArgs::new_empty_fixed(total_stages),
            },
        )
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Missions {
    pub clear_states: HashMapLength<i32, i32, i32>,
    pub requirements: HashMapLength<i32, i32, i32>,
    pub progress_types: HashMapLength<i32, i32, i32>,
    pub gamatoto_values: HashMapLength<i32, i32, i32>,
    pub nyancombo_values: HashMapLength<i32, i32, i32>,
    pub user_rank_values: HashMapLength<i32, i32, i32>,
    pub expiry_values: HashMapLength<i32, i32, i32>,
    #[rw(gvcc)]
    pub preparing_values: PreparingValues,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreparingValues {
    Old(HashMapLength<i32, i32, bool>),
    New(HashMapLength<i32, i32, i32>),
}

impl From<&HashMapLength<i32, i32, bool>> for HashMapLength<i32, i32, i32> {
    fn from(value: &HashMapLength<i32, i32, bool>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i32);
        }

        Self::new(new_map)
    }
}
impl From<&HashMapLength<i32, i32, i32>> for HashMapLength<i32, i32, bool> {
    fn from(value: &HashMapLength<i32, i32, i32>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v != 0);
        }

        Self::new(new_map)
    }
}

impl Default for PreparingValues {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for PreparingValues {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..90300 => Ok(Self::Old(HashMapLength::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl Writable for PreparingValues {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..90300 => match self {
                PreparingValues::Old(hash_map_length) => hash_map_length.write_no_opts(writer)?,
                PreparingValues::New(hash_map_length) => {
                    let other: HashMapLength<i32, i32, bool> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
            },
            _ => match self {
                PreparingValues::Old(hash_map_length) => {
                    let other: HashMapLength<i32, i32, i32> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
                PreparingValues::New(hash_map_length) => hash_map_length.write_no_opts(writer)?,
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TowerChapters {
    pub chapters: ChaptersGeneric<i32, i32, StageClear<i32>, i32>,
}

impl Readable for TowerChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(Self {
            chapters: ChaptersGeneric::read(reader, GenericChapterArgs::new_int(true))?,
        })
    }
}

impl Writable for TowerChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(true))
    }
}

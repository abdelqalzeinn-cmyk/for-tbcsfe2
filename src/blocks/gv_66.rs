use std::collections::HashMap;

use bcsfe_derive::{Readable, Writable};

use crate::{
    save::{ChaptersGeneric, GVCC, GenericChapterArgs, StageClear},
    stream::{
        HashMapLength, Readable, ReadableNoOptions, StreamResult, VecArgs, VecArgsLength, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 66)]
pub struct GV66Block {
    pub tower: TowerChapters,
    #[rw(gvcc)]
    pub missions: Missions,
    pub tower_item_obtain_states: TowerItemObtainStates,
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
        self,
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
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub clear_states: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub requirements: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub progress_types: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub gamatoto_values: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub nyancombo_values: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub user_rank_values: HashMap<i32, i32>,
    #[rw(with = "HashMapLength<i32, i32, i32>")]
    pub expiry_values: HashMap<i32, i32>,
    #[rw(gvcc)]
    pub preparing_values: PreparingValues,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreparingValues {
    Old(HashMap<i32, bool>),
    New(HashMap<i32, i32>),
}

impl Default for PreparingValues {
    fn default() -> Self {
        Self::New(HashMap::default())
    }
}

impl Readable for PreparingValues {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..90300 => Ok(Self::Old(
                <HashMapLength<i32, i32, bool>>::read_no_opts(reader)?.into(),
            )),
            _ => Ok(Self::New(
                <HashMapLength<i32, i32, i32>>::read_no_opts(reader)?.into(),
            )),
        }
    }
}

impl Writable for PreparingValues {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..90300 => match self {
                PreparingValues::Old(hash_map_length) => {
                    <HashMapLength<i32, i32, bool>>::write_no_opts(hash_map_length.into(), writer)?
                }
                PreparingValues::New(hash_map_length) => {
                    let other: HashMap<i32, bool> = hash_map_length
                        .into_iter()
                        .map(|(k, v)| (k, v != 0))
                        .collect();

                    <HashMapLength<i32, i32, bool>>::write_no_opts(other.into(), writer)?;
                }
            },
            _ => match self {
                PreparingValues::Old(hash_map_length) => {
                    let other: HashMapLength<i32, i32, i32> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
                PreparingValues::New(hash_map_length) => {
                    <HashMapLength<i32, i32, i32>>::write_no_opts(hash_map_length.into(), writer)?
                }
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
        self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(true))
    }
}

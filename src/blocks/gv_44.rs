use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        LengthString, Readable, ReadableNoOptions, StreamResult, VecArgs, VecArgsLength, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 44)]
pub struct GV44Block {
    #[rw(gvcc)]
    pub item_reward_chapters: ItemRewardChapters<bool>,
    #[rw(gvcc)]
    pub timed_score_chapters: ItemRewardChapters<i32>,
    #[rw(with = "LengthString<i32>")]
    pub inquiry_code: String,
    pub play_time: i32,
    pub has_account: i8,
    pub backup_state: i32,
    #[rw(jp = false)]
    pub ub2: bool,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItemRewardChapters<T> {
    pub chapters: Vec<Vec<Vec<T>>>,
}

impl<T: for<'a> Readable<Args<'a> = ()> + std::fmt::Debug> Readable for ItemRewardChapters<T> {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let (total_subchapters, total_stages, total_stars) = match args.gv.0 {
            ..=33 => (50, 12, 3),
            34 => (i32::read_no_opts(reader)?, 12, 3),
            _ => (
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
            ),
        };

        Ok(Self {
            chapters: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters as usize),
                    item: VecArgs {
                        length: VecArgsLength::Fixed(total_stars as usize),
                        item: VecArgs::new_empty_fixed(total_stages as usize),
                    },
                },
            )?,
        })
    }
}

impl<T: for<'a> Writable<Args<'a> = ()> + Default + std::fmt::Debug> Writable
    for ItemRewardChapters<T>
{
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_subchapters = self.chapters.len();
        let total_stages = self
            .chapters
            .first()
            .unwrap_or(&Vec::new())
            .first()
            .unwrap_or(&Vec::new())
            .len();
        let total_stars = self.chapters.first().unwrap_or(&Vec::new()).len();

        let (total_subchapters, total_stages, total_stars) = match args.gv.0 {
            ..=33 => (50, 12, 3),
            34 => {
                (total_subchapters as i32).write_no_opts(writer)?;
                (total_subchapters, 12, 3)
            }
            _ => {
                (total_subchapters as i32).write_no_opts(writer)?;
                (total_stages as i32).write_no_opts(writer)?;
                (total_stars as i32).write_no_opts(writer)?;
                (total_subchapters, total_stages, total_stars)
            }
        };

        self.chapters.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_subchapters),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_stars),
                    item: VecArgs::new_empty_fixed(total_stages),
                },
            },
        )?;

        Ok(())
    }
}

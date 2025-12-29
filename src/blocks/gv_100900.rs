use bcsfe_derive::{Readable, Writable};

use crate::stream::{
    HashMapLength, LengthVec, Readable, ReadableNoOptions, StreamResult, VecArgs, VecArgsLength,
    Writable, WritableNoOptions,
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 100900)]
pub struct GV100900Block {
    pub aku: AkuChapters,
    pub u1: bool,
    pub u2: bool,
    pub u3: HashMapLength<i16, i16, LengthVec<i16, i16>>,
    pub u4: HashMapLength<i16, i16, f64>,
    pub u5: HashMapLength<i16, i16, f64>,
    pub u6: bool,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AkuChapters {
    pub current_stages: Vec<Vec<i8>>,
    pub stages: Vec<Vec<Vec<i16>>>,
}

impl Readable for AkuChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let total_chapters = i16::read_no_opts(reader)? as usize;
        let total_stages = i8::read_no_opts(reader)? as usize;
        let total_stars = i8::read_no_opts(reader)? as usize;

        Ok(Self {
            current_stages: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::Fixed(total_chapters),
                    item: VecArgs::new_empty_fixed(total_stars),
                },
            )?,
            stages: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::Fixed(total_chapters),
                    item: VecArgs {
                        length: VecArgsLength::Fixed(total_stars),
                        item: VecArgs::new_empty_fixed(total_stages),
                    },
                },
            )?,
        })
    }
}

impl Writable for AkuChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_chapters = self.current_stages.len();
        let total_stages = self
            .stages
            .first()
            .unwrap_or(&Vec::new())
            .first()
            .unwrap_or(&Vec::new())
            .len();
        let total_stars = self.current_stages.first().unwrap_or(&Vec::new()).len();

        (total_chapters as i16).write_no_opts(writer)?;
        (total_stages as i8).write_no_opts(writer)?;
        (total_stars as i8).write_no_opts(writer)?;

        self.current_stages.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )?;

        self.stages.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_stars),
                    item: VecArgs::new_empty_fixed(total_stages),
                },
            },
        )?;

        Ok(())
    }
}

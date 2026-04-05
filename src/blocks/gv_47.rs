use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{
        NewResultCtx, Readable, ReadableNoOptions, StreamResult, VecArgs, Writable,
        WritableNoOptions,
    },
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EventCapsules {
    pub event_capsules: Vec<i32>,
    pub event_capsules_counter: Vec<i32>,
}

impl Readable for EventCapsules {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..34 => VecArgs::new_empty_fixed(100),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(Self {
            event_capsules: Vec::read(reader, length).add_context(|| "event capsules")?,
            event_capsules_counter: Vec::read(reader, length)
                .add_context(|| "read event capsules counter")?,
        })
    }
}

impl Writable for EventCapsules {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..34 => VecArgs::new_empty_fixed(100),
            _ => VecArgs::new_empty_i32(),
        };

        self.event_capsules.write(writer, length)?;
        self.event_capsules_counter.write(writer, length)
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GatyaSeed(u32);

impl From<u32> for GatyaSeed {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<GatyaSeed> for u32 {
    fn from(value: GatyaSeed) -> Self {
        value.0
    }
}

impl Default for GatyaSeed {
    fn default() -> Self {
        Self(0)
    }
}

impl Readable for GatyaSeed {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(match args.gv.0 {
            0..33 => Self(u64::read_no_opts(reader)? as u32),
            _ => Self(u32::read_no_opts(reader)?),
        })
    }
}

impl Writable for GatyaSeed {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..33 => (self.0 as u64).write_no_opts(writer)?,
            _ => self.0.write_no_opts(writer)?,
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[rw(end_assert = 47)]
pub struct GV47Block {
    #[rw(gvcc, with = "GatyaSeed")]
    pub event_seed: u32,
    #[rw(gvcc)]
    pub event_capsules: EventCapsules,
}

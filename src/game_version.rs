use std::{fmt::Display, str::FromStr};

use crate::stream::{NewResultCtx, Readable, Writable};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct GameVersion(pub u32);

impl From<u32> for GameVersion {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<GameVersion> for u32 {
    fn from(value: GameVersion) -> Self {
        value.0
    }
}

impl GameVersion {
    pub fn into_segments(self) -> [u8; 3] {
        [
            ((self.0 / 10_000) % 100) as u8,
            ((self.0 / 100) % 100) as u8,
            (self.0 % 100) as u8,
        ]
    }

    pub fn from_segments(segments: [u8; 3]) -> Self {
        Self(segments[0] as u32 * 10_000 + segments[1] as u32 * 100 + segments[2] as u32)
    }
}

impl Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.into_segments().map(|v| format!("{v:02}")).join(".")
        )
    }
}

impl From<[u8; 3]> for GameVersion {
    fn from(value: [u8; 3]) -> Self {
        Self::from_segments(value)
    }
}

pub enum InvalidGameVersionStr {
    InvalidSegment(String, std::num::ParseIntError),
    IncorrectSegmentNumber(usize, Vec<u8>),
}

impl FromStr for GameVersion {
    type Err = InvalidGameVersionStr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.split(".");

        let mut segments_u8 = Vec::with_capacity(3);

        for segment in segments {
            let s: u8 = segment
                .parse()
                .map_err(|e| InvalidGameVersionStr::InvalidSegment(segment.to_string(), e))?;

            segments_u8.push(s);
        }

        let len = segments_u8.len();

        let segments_arr: [u8; 3] = segments_u8
            .try_into()
            .map_err(|e| InvalidGameVersionStr::IncorrectSegmentNumber(len, e))?;

        Ok(segments_arr.into())
    }
}

#[cfg(feature = "network")]
impl serde::Serialize for GameVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl Readable for GameVersion {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(Self(
            u32::read(reader, ()).add_context(|| "read u32 for game version")?,
        ))
    }
}

impl Writable for GameVersion {
    type Args<'a> = ();
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<()> {
        self.0
            .write(writer, ())
            .add_context(|| "write u32 for game version")
    }
}

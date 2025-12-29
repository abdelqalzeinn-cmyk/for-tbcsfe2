use crate::{
    country_code::CountryCode,
    save::GVCC,
    stream::{HashMapLength, LengthString, Readable, StreamError, StreamResult, Writable},
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV120700Block {
    pub u1: Option<HashMapLength<i8, LengthString<i32>, LengthString<i32>>>,
}

impl Readable for GV120700Block {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let min_gv = match args.cc {
            CountryCode::Jp => 130000,
            _ => 120700,
        };

        if args.gv.0 < min_gv {
            Ok(Self { u1: None })
        } else {
            let u1 = <HashMapLength<i8, LengthString<i32>, LengthString<i32>>>::read(reader, ())?;
            let se = Self { u1: Some(u1) };

            let pos = reader.stream_position()?;
            let gv = u32::read(reader, ())?;

            if gv != min_gv {
                return Err(StreamError::new(
                    std::io::Error::other(format!(
                        "assertion error, expected: {min_gv}, got: {gv}"
                    )),
                    pos,
                ));
            }

            Ok(se)
        }
    }
}

impl Writable for GV120700Block {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let min_gv = match args.cc {
            CountryCode::Jp => 130000,
            _ => 120700,
        };

        if args.gv.0 < min_gv {
            Ok(())
        } else {
            self.u1
                .as_ref()
                .unwrap_or(&HashMapLength::default())
                .write(writer, ())?;

            min_gv.write(writer, ())?;

            Ok(())
        }
    }
}

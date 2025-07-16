use bcsfe_derive::{Readable, Writable};

use crate::{
    country_code::CountryCode,
    save::GVCC,
    stream::{Assertable, HashMapLength, LengthString, Readable, StreamResult, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120700BlockInner {
    pub u1: HashMapLength<i8, LengthString<i32>, LengthString<i32>>, // FIXME: may not be a hashmap
    #[rw(jp = false)]
    pub _120700: Option<Assertable<120700>>,
    #[rw(en = false, kr = false, tw = false)]
    pub _130000: Option<Assertable<130000>>,
}

#[derive(Debug, Clone, Default)]
pub struct GV120700Block {
    pub inner: Option<GV120700BlockInner>,
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
            Ok(Self { inner: None })
        } else {
            Ok(Self {
                inner: Some(GV120700BlockInner::read(reader, args)?),
            })
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
            self.inner
                .as_ref()
                .unwrap_or(&GV120700BlockInner::default())
                .write(writer, args)
        }
    }
}

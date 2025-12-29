use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{Assertable, LengthString, Readable, StreamResult, VecArgs, Writable},
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotNames {
    pub names: Vec<LengthString<i32>>,
}

impl Readable for SlotNames {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let total_slots = match args.gv.0 {
            0..110600 => VecArgs::new_empty_fixed(15),
            _ => VecArgs::new_empty_i8(),
        };

        Ok(Self {
            names: Vec::read(reader, total_slots)?,
        })
    }
}

impl Writable for SlotNames {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_slots = match args.gv.0 {
            0..110600 => VecArgs::new_empty_fixed(15),
            _ => VecArgs::new_empty_i8(),
        };

        self.names.write(writer, total_slots)
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV91000Block {
    #[rw(gvcc)]
    pub slot_names: SlotNames,
    pub _91000: Assertable<91000>,
}

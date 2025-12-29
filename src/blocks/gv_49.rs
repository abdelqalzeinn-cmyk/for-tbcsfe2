use bcsfe_derive::{Readable, Writable};

use crate::{
    save::GVCC,
    stream::{Assertable, Readable, StreamResult, VecArgs, Writable},
};

#[derive(Debug, Clone, Readable, Writable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GV49Block {
    #[rw(en = false, kr = false, tw = false)]
    pub energy_notification: Option<bool>,
    pub get_time_save_4: f64,
    #[rw(gvcc)]
    pub gatya_lucky_drops: GatyaSeenLuckyDrops,
    pub show_ban_message: bool,
    pub catfood_beginner_purchased: [bool; 3],
    pub next_week_timestamp: f64,
    pub catfood_beginner_expired: [bool; 3],
    pub rank_up_sale_value: i32,
    pub _49: Assertable<49>,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GatyaSeenLuckyDrops {
    pub drops: Vec<i32>,
}

impl Readable for GatyaSeenLuckyDrops {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..26 => VecArgs::new_empty_fixed(44),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(Self {
            drops: Vec::read(reader, length)?,
        })
    }
}

impl Writable for GatyaSeenLuckyDrops {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..26 => VecArgs::new_empty_fixed(44),
            _ => VecArgs::new_empty_i32(),
        };
        self.drops.write(writer, length)
    }
}

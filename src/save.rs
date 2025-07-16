use std::{collections::HashMap, io::Seek};

use crate::{
    country_code::CountryCode,
    game_version::GameVersion,
    stream::{
        Assertable, HashMapLength, LengthString, LengthVec, NewResultCtx, Readable,
        ReadableNoOptions, StreamError, StreamResult, VariableLengthInt, VecArgs, VecArgsLength,
        Writable, WritableNoOptions,
    },
};
use bcsfe_derive::{Readable, Writable};

#[derive(Debug, Clone, Copy, Default)]
pub struct DateTimeDst {
    pub dst: Option<bool>,
    pub datetime: DateTime,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct DateTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GVCC {
    pub cc: CountryCode,
    pub gv: GameVersion,
}

impl GVCC {
    pub fn should_read_dst(&self) -> bool {
        self.cc != CountryCode::Jp && self.gv.0 >= 49
    }
}

impl Readable for DateTimeDst {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let dst = if args.should_read_dst() {
            Some(bool::read(reader, ()).add_context(|| "read dst for date time")?)
        } else {
            None
        };

        Ok(Self {
            dst,
            datetime: DateTime::read_no_opts(reader)
                .add_context(|| "read datetime for datetimedst")?,
        })
    }
}

impl Writable for DateTimeDst {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        if args.should_read_dst() {
            self.dst.unwrap_or_default().write_no_opts(writer)?;
        }

        self.datetime.write_no_opts(writer)
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct MainDate {
    pub datetime: DateTimeDst,
    pub timestamp: f64,
    pub date: Date,
}

impl Writable for MainDate {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.datetime.datetime.year.write_no_opts(writer)?;
        self.date.year.write_no_opts(writer)?;

        self.datetime.datetime.month.write_no_opts(writer)?;
        self.date.month.write_no_opts(writer)?;

        self.datetime.datetime.day.write_no_opts(writer)?;
        self.date.day.write_no_opts(writer)?;

        self.timestamp.write_no_opts(writer)?;

        self.datetime.datetime.hour.write_no_opts(writer)?;
        self.datetime.datetime.minute.write_no_opts(writer)?;
        self.datetime.datetime.second.write_no_opts(writer)?;

        if args.should_read_dst() {
            self.datetime
                .dst
                .unwrap_or_default()
                .write_no_opts(writer)?;
        }

        Ok(())
    }
}

impl Readable for MainDate {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let year1 = i32::read_no_opts(reader).add_context(|| "read year1 for datetime1")?;
        let year = i32::read_no_opts(reader).add_context(|| "read year for date")?;

        let month1 = i32::read_no_opts(reader).add_context(|| "read month1 for datetime1")?;
        let month = i32::read_no_opts(reader).add_context(|| "read month for date")?;

        let day1 = i32::read_no_opts(reader).add_context(|| "read day1 for datetime1")?;
        let day = i32::read_no_opts(reader).add_context(|| "read day for date")?;

        let timestamp = f64::read_no_opts(reader).add_context(|| "read timestamp for main_date")?;

        let hour = i32::read_no_opts(reader).add_context(|| "read hour for date")?;
        let minute = i32::read_no_opts(reader).add_context(|| "read minute for date")?;
        let second = i32::read_no_opts(reader).add_context(|| "read second for date")?;

        let dst = if args.should_read_dst() {
            Some(bool::read_no_opts(reader).add_context(|| "read dst for datetime")?)
        } else {
            None
        };

        Ok(Self {
            datetime: DateTimeDst {
                dst,
                datetime: DateTime {
                    year: year1,
                    month: month1,
                    day: day1,
                    hour,
                    minute,
                    second,
                },
            },
            timestamp,
            date: Date { year, month, day },
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ub1(pub Option<bool>);

impl Readable for Ub1 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        if args.cc != CountryCode::Jp || args.gv.0 >= 10 {
            Ok(Self(Some(bool::read_no_opts(reader)?)))
        } else {
            Ok(Self(None))
        }
    }
}

impl Writable for Ub1 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        if args.cc != CountryCode::Jp || args.gv.0 >= 10 {
            self.0.unwrap_or_default().write_no_opts(writer)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnlockPopups8 {
    pub popups: Vec<i32>,
}

impl Readable for UnlockPopups8 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let length = match args.gv.0 {
            0 | 1 => 3,
            2 | 3 | 4 => 4,
            5 => 5,
            6 | 7 | 8 | 9 => 6,
            _ => 36,
        };

        Ok(Self {
            popups: Vec::read(reader, VecArgs::new_empty_fixed(length))?,
        })
    }
}

impl Writable for UnlockPopups8 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0 | 1 => 3,
            2 | 3 | 4 => 4,
            5 => 5,
            6 | 7 | 8 | 9 => 6,
            _ => 36,
        };

        self.popups.write(writer, VecArgs::new_empty_fixed(length))
    }
}

#[derive(Debug, Clone, Readable, Writable)]
pub struct Save {
    // pub game_version: GameVersion,
    #[rw(gvcc)]
    pub ub1: Ub1,
    pub mute_bgm: bool,
    pub mute_sfx: bool,
    pub catfood: i32,
    pub current_energy: i32,
    #[rw(gvcc)]
    pub main_date: MainDate,
    pub ui1: i32,
    pub stamp_value_save: i32,
    pub ui2: i32,
    pub upgrade_stage: i32,
    pub xp: i32,
    pub tutorial_state: i32,
    pub ui3: i32,
    pub korea_superior_treasure_state: i32,
    pub unlock_popups: [i32; 3],
    pub ui5: i32,
    pub unlock_enemy_guide: i32,
    pub ui6: i32,
    pub ub0: bool,
    pub ui7: i32,
    pub cleared_eoc_1: i32,
    pub ui8: i32,
    pub unlocked_ending: i32,
    #[rw(gvcc)]
    pub lineups: LineUps,
    pub stamp_data: StampData,
    pub story_chapters: StoryChapters,
    #[rw(gvcc)]
    pub enemy_guide: EnemyGuide,
    #[rw(gvcc)]
    pub unlocked_cats: CatsField<Vec<i32>>,
    #[rw(gvcc)]
    pub cat_upgrades: CatsField<Vec<Upgrade>>,
    #[rw(gvcc)]
    pub cat_current_forms: CatsField<Vec<Formi32>>,
    pub special_skill_upgrades: [Upgrade; 11],
    #[rw(gvcc)]
    pub menu_unlocks: MenuUnlocks1,
    pub battle_items: [i32; TOTAL_BATTLE_ITEMS],
    #[rw(gvcc)]
    pub new_dialogs: NewDialogs,
    pub uil1: [i32; 20],
    pub moneko_bonus: [i32; 1],
    pub daily_reward_initialized: [i32; 1],
    #[rw(gvcc, max_gv = 4)]
    pub unknown_bool_list: Option<UnknownEarlyBoolList>,
    pub locked_battle_items: LockedBattleItems,
    #[rw(gvcc)]
    pub date2: DateTimeDst,
    pub story_treasure_festival: StoryTreasureFestival,
    #[rw(gvcc)]
    pub date3: DateTimeDst,
    #[rw(max_gv = 37)]
    pub ui0: Option<i32>,
    pub stage_unlock_cat_value: i32,
    pub show_ending_value: i32,
    pub chapter_clear_cat_unlock: i32,
    pub ui9: i32,
    pub ios_android_month: i32,
    pub ui10: i32,
    pub save_data_4_hash: LengthString<i32>,
    pub bonus_hash: BonusHash,
    pub chara_flags: [i32; 2],
    #[rw(max_gv = 37)]
    pub uib: Option<(i32, bool)>,
    pub chara_flags2: [i32; 2],
    pub normal_tickets: i32,
    pub rare_tickets: i32,
    #[rw(gvcc)]
    pub gacha_seen_cats: CatsField<Vec<i32>>,
    pub gacha_seen_special_skills: [i32; 10],
    #[rw(gvcc)]
    pub storage: CatStorage,
    #[rw(gvcc)]
    pub event_chapters: EventChapters,
    pub itf1_ending: i32,
    pub continue_flag: i32,
    #[rw(gvcc)]
    pub unlock_popups_8: UnlockPopups8,
    #[rw(gvcc)]
    pub unit_drops: UnitDrops,
    #[rw(gvcc)]
    pub rare_seed: GatyaSeed,
    #[rw(gvcc)]
    pub normal_seed: GatyaSeed,
    pub get_event_data: bool,
    pub achievements: [bool; 7],
    pub os_value: i32,
    #[rw(gvcc)]
    pub date4: DateTimeDst,
    #[rw(gvcc)]
    pub gatya: GatyaData,
    #[rw(jp = false)]
    pub player_id: Option<LengthString<i32>>,
    pub order_ids: LengthVec<i32, LengthString<i32>>,
    #[rw(jp = false)]
    pub some_time_info: Option<SomeTimeInfo>,
    pub selected_slot: i32,
    #[rw(gvcc)]
    pub unlocked_slots: UnlockedSlots,
    #[rw(gvcc)]
    pub legend_restriction: EventStageLegendRestriction,
    #[rw(max_gv = 37)]
    pub uill: Option<[[i32; 7]; 3]>,
    pub g_timestamp: f64,
    pub server_timestamp: f64,
    pub get_time_save: f64,
    pub unknown_timestamp: f64,
    pub gatya_trade_progress: i32,
    #[rw(max_gv = 37)]
    pub usl2: Option<LengthVec<i32, LengthString<i32>>>,
    #[rw(jp = false)]
    pub timesave2: Option<f64>,
    #[rw(en = false, kr = false, tw = false)]
    pub ui11: Option<i32>,
    #[rw(gvcc)]
    pub ubl1: Ubl1,
    #[rw(gvcc)]
    pub max_cat_upgrade_levels: CatsField<Vec<Upgrade>>,
    pub max_special_skill_levels: [Upgrade; 11],
    #[rw(gvcc)]
    pub user_rank_rewards: UserRankRewards,
    #[rw(en = false, kr = false, tw = false)]
    pub timesave3: Option<f64>,
    #[rw(gvcc)]
    pub unlocked_forms: CatsField<Vec<Formi32>>,
    pub transfer_code: LengthString<i32>,
    pub confirmation_code: LengthString<i32>,
    pub transfer_flag: bool,
    #[rw(gvcc, min_gv = 18)]
    pub gv_44: Option<GV44Block>,
    #[rw(gvcc, min_gv = 19)]
    pub gv_45: Option<GV45Block>,
    #[rw(min_gv = 20)]
    pub gv_46: Option<GV46Block>,
    #[rw(gvcc, min_gv = 21)]
    pub gv_47: Option<GV47Block>,
    #[rw(min_gv = 22)]
    pub _48: Option<Assertable<48>>,
    #[rw(gvcc, min_gv = 23)]
    pub gv_49: Option<GV49Block>,
    #[rw(min_gv = 24)]
    pub _50: Option<Assertable<50>>,
    #[rw(min_gv = 25)]
    pub _51: Option<Assertable<51>>,
    #[rw(min_gv = 26)]
    pub gv_52: Option<GV52Block>,
    #[rw(min_gv = 27)]
    pub gv_53: Option<GV53Block>,
    #[rw(min_gv = 29)]
    pub gv_54: Option<GV54Block>,
    #[rw(gvcc, min_gv = 30)]
    pub gv_55: Option<GV55Block>,
    #[rw(min_gv = 31)]
    pub gv_56: Option<GV56Block>,
    #[rw(min_gv = 32)]
    pub gv_57: Option<GV57Block>,
    #[rw(min_gv = 33)]
    pub gv_58: Option<GV58Block>,
    #[rw(min_gv = 34)]
    pub gv_59: Option<GV59Block>,
    #[rw(gvcc, min_gv = 35)]
    pub gv_60: Option<GV60Block>,
    #[rw(min_gv = 36)]
    pub gv_61: Option<GV61Block>,
    #[rw(min_gv = 38)]
    pub gv_63: Option<GV63Block>,
    #[rw(gvcc, min_gv = 39)]
    pub gv_64: Option<GV64Block>,
    #[rw(min_gv = 40)]
    pub gv_65: Option<GV65Block>,
    #[rw(gvcc, min_gv = 41)]
    pub gv_66: Option<GV66Block>,
    #[rw(gvcc, min_gv = 42)]
    pub gv_67: Option<GV67Block>,
    #[rw(min_gv = 43)]
    pub gv_68: Option<GV68Block>,
    #[rw(min_gv = 44)]
    pub gv_69: Option<GV69Block>,
    #[rw(min_gv = 46)]
    pub gv_71: Option<GV71Block>,
    #[rw(min_gv = 47, max_gv = 90299)]
    pub gv_72: Option<GV72Block>,
    #[rw(min_gv = 51)]
    pub gv_76: Option<GV76Block>,
    #[rw(min_gv = 77)]
    pub gv_77: Option<GV77Block>,
    #[rw(gvcc, min_gv = 80000)]
    pub gv_80000: Option<GV80000Block>,
    #[rw(min_gv = 80200)]
    pub gv_80200: Option<GV80200Block>,
    #[rw(min_gv = 80300)]
    pub gv_80300: Option<GV80300Block>,
    #[rw(min_gv = 80500)]
    pub gv_80500: Option<GV80500Block>,
    #[rw(min_gv = 80600)]
    pub gv_80600: Option<GV80600Block>,
    #[rw(min_gv = 80700)]
    pub gv_80700: Option<GV80700Block>,
    #[rw(min_gv = 100600, jp = false, kr = false, tw = false)]
    pub gv_100600_en: Option<GV100600BlockEn>,
    #[rw(min_gv = 81000)]
    pub gv_81000: Option<GV81000Block>,
    #[rw(gvcc, min_gv = 90000)]
    pub gv_90000: Option<GV90000Block>,
    #[rw(min_gv = 90100)]
    pub gv_90100: Option<GV90100Block>,
    #[rw(min_gv = 90300)]
    pub gv_90300: Option<GV90300Block>,
    #[rw(gvcc, min_gv = 90400)]
    pub gv_90400: Option<GV90400Block>,
    #[rw(gvcc, min_gv = 90500)]
    pub gv_90500: Option<GV90500Block>,
    #[rw(gvcc, min_gv = 90700)]
    pub gv_90700: Option<GV90700Block>,
    #[rw(min_gv = 90800)]
    pub gv_90800: Option<GV90800Block>,
    #[rw(min_gv = 90900)]
    pub gv_90900: Option<GV90900Block>,
    #[rw(gvcc, min_gv = 91000)]
    pub gv_91000: Option<GV91000Block>,
    #[rw(min_gv = 100000)]
    pub gv_100000: Option<GV100000Block>,
    #[rw(min_gv = 100100)]
    pub gv_100100: Option<GV100100Block>,
    #[rw(min_gv = 100300)]
    pub gv_100300: Option<GV100300Block>,
    #[rw(min_gv = 100400)]
    pub gv_100400: Option<GV100400Block>,
    #[rw(min_gv = 100600)]
    pub gv_100600: Option<GV100600Block>,
    #[rw(gvcc, min_gv = 100700)]
    pub gv_100700: Option<GV100700Block>,
    #[rw(min_gv = 100900)]
    pub gv_100900: Option<GV100900Block>,
    #[rw(min_gv = 101000)]
    pub gv_101000: Option<GV101000Block>,
    #[rw(min_gv = 110000)]
    pub gv_110000: Option<GV110000Block>,
    #[rw(min_gv = 110500)]
    pub gv_110500: Option<GV110500Block>,
    #[rw(min_gv = 110600)]
    pub gv_110600: Option<GV110600Block>,
    #[rw(gvcc, min_gv = 110700)]
    pub gv_110700: Option<GV110700Block>,
    #[rw(min_gv = 110800)]
    pub gv_110800: Option<GV110800Block>,
    #[rw(min_gv = 111000)]
    pub gv_111000: Option<GV111000Block>,
    #[rw(min_gv = 120000)]
    pub gv_120000: Option<GV120000Block>,
    #[rw(min_gv = 120100)]
    pub gv_120100: Option<GV120100Block>,
    #[rw(min_gv = 120200)]
    pub gv_120200: Option<GV120200Block>,
    #[rw(min_gv = 120400)]
    pub gv_120400: Option<GV120400Block>,
    #[rw(min_gv = 120500)]
    pub gv_120500: Option<GV120500Block>,
    #[rw(min_gv = 120600)]
    pub gv_120600: Option<GV120600Block>,
    #[rw(gvcc)]
    pub gv_120700: GV120700Block,
    #[rw(min_gv = 130100)]
    pub gv_130100: Option<GV130100Block>,
    #[rw(min_gv = 130301)]
    pub gv_130301: Option<GV130301Block>,
    #[rw(min_gv = 130400)]
    pub gv_130400: Option<GV130400Block>,
    #[rw(min_gv = 130500)]
    pub gv_130500: Option<GV130500Block>,
    #[rw(gvcc, min_gv = 130600)]
    pub gv_130600: Option<GV130600Block>,
    #[rw(gvcc, min_gv = 130700)]
    pub gv_130700: Option<GV130700Block>,
    #[rw(min_gv = 140000)]
    pub gv_140000: Option<GV140000Block>,
    #[rw(min_gv = 140100, max_gv = 140499)]
    pub gv_140100: Option<GV140100Block>,
    #[rw(gvcc, min_gv = 140200)]
    pub gv_140200: Option<GV140200Block>,
    #[rw(min_gv = 140300)]
    pub gv_140300: Option<GV140300Block>,
    pub remaing_data: RemainingData,
}

#[derive(Debug, Clone, Default)]
pub struct RemainingData {
    pub data: Vec<u8>,
}

impl Readable for RemainingData {
    type Args<'a> = ();
    fn read<R: std::io::Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let pos = reader.stream_position()?;

        reader.seek(std::io::SeekFrom::End(-32))?;

        let end_pos = reader.stream_position()?;

        let to_read = end_pos - pos;

        reader.seek(std::io::SeekFrom::Start(pos))?;

        Ok(Self {
            data: Vec::read(reader, VecArgs::new_empty_fixed(to_read as usize))?,
        })
    }
}

impl Writable for RemainingData {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.data
            .write(writer, VecArgs::new_empty_fixed(self.data.len()))
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV140300Block {
    pub u1: LengthVec<i8, i8>,
    pub u2: bool,
    pub treasure_chests: LengthVec<i8, i32>,
    pub u3: i32,
    pub u4: LengthVec<i16, i32>,
    pub u5: bool,
    pub _140300: Assertable<140300>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV140200Block {
    #[rw(gvcc)]
    pub dojo_ranking_2: DojoRanking2,
    pub unknown: HashMapLength<i8, i32, f64>,
    pub hundred_million_ticket: i32,
    pub _140200: Assertable<140200>,
}

#[derive(Debug, Clone, Default)]
pub struct DojoRanking2 {
    pub ranking: Vec<DojoRank2>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct DojoRank2 {
    #[rw(gvcc)]
    pub ranking: DojoRanking,
    pub unknown: bool,
}

impl Readable for DojoRanking2 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        Ok(Self {
            ranking: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::I8,
                    item: args,
                },
            )?,
        })
    }
}

impl Writable for DojoRanking2 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.ranking.write(
            writer,
            VecArgs {
                length: VecArgsLength::I8,
                item: args,
            },
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV140100Block {
    pub unknown: i8,
    pub _140100: Assertable<140100>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV140000Block {
    pub u1: i32,
    pub u2: f64,
    pub u3: i8,
    pub u5: HashMapLength<i8, i32, LengthVec<i8, i8>>,
    pub unknown_chapters: NewChapters,
    pub u6: LengthVec<i16, i32>,
    pub u7: bool,
    pub u8: f64,
    pub u9: HashMapLength<i16, i16, i8>,
    pub _140000: Assertable<140000>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130700Block {
    #[rw(en = false, kr = false, tw = false)]
    pub u1: Option<i16>,
    pub u2: f64,
    pub u3: i8,
    pub u4: i8,
    pub u5: i16,
    pub u6: i8,
    pub u7: i8,
    pub u8: i8,
    pub u9: f64,
    pub u10: HashMapLength<i16, i16, (i16, i32, HashMapLength<i16, i16, i16>)>,
    pub _130700: Assertable<130700>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV130600Block {
    pub u1: i8,
    #[rw(jp = false)]
    pub u2: Option<i16>,
    pub _130600: Assertable<130600>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130500Block {
    pub unknown_chapters: NewChapters,
    pub _130500: Assertable<130500>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV130400Block {
    pub u1: f64,
    pub u2: f64,
    pub _130400: Assertable<130400>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130301Block {
    pub unknown: HashMapLength<i32, LengthString<i32>, (i32, f64)>, // uuid, ?, timestamp
    pub _130301: Assertable<130301>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV130100Block {
    pub unknown: HashMapLength<i32, i32, i64>, // FIXME: may not be a hashmap
    pub _130100: Assertable<130100>,
}

#[derive(Debug, Clone, Default)]
pub struct GV120700Block {
    pub inner: Option<GV120700BlockInner>,
}

impl Readable for GV120700Block {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
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
    fn write<W: std::io::Write + Seek>(
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

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120700BlockInner {
    pub u1: HashMapLength<i8, LengthString<i32>, LengthString<i32>>, // FIXME: may not be a hashmap
    #[rw(jp = false)]
    pub _120700: Option<Assertable<120700>>,
    #[rw(en = false, kr = false, tw = false)]
    pub _130000: Option<Assertable<130000>>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV120600Block {
    pub sfx_volume: i8,
    pub bgm_volume: i8,
    pub _120600: Assertable<120600>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV120500Block {
    pub u1: bool,
    pub u2: bool,
    pub u3: bool,
    pub date: i32,
    pub u5: i8,
    pub _120500: Assertable<120500>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV120400Block {
    pub timestamp1: f64,
    pub timestamp2: f64,
    pub _120400: Assertable<120400>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120200Block {
    pub u1: bool,
    pub u2: i16,
    pub u3: HashMapLength<i8, i16, i16>,
    pub _120200: Assertable<120200>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120100Block {
    pub unknown: LengthVec<i16, i16>,
    pub _120100: Assertable<120100>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct NewChapter {
    pub selected_stage: i8,
    pub clear_progress: i8,
    pub unlock_state: i8,
    pub stages: LengthVec<i16, i16>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct NewChapters {
    pub chapters: LengthVec<i16, (i8, LengthVec<i8, NewChapter>)>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV120000Block {
    pub zero_legends: NewChapters,
    pub unknown: i8,
    pub _120000: Assertable<120000>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV111000Block {
    pub u1: i32,
    pub u2: i16,
    pub u3: i8,
    pub u4: i8,
    pub u5: bool,
    pub u6: i8,
    pub u7: LengthVec<i8, i16>,
    pub u8: LengthVec<i16, i16>,
    pub u9: LengthVec<i16, i16>,
    pub u10: i32,
    pub u11: i32,
    pub date1: i32,
    pub date2: i16,
    pub u14: i16,
    pub u15: i16,
    pub u16: i16,
    pub u17: i8,
    pub u18: bool,
    pub u19: bool,
    pub u20: bool,
    pub u21: bool,
    pub u22: bool,
    pub u23: bool,
    pub u24: i8,
    pub u25: LengthVec<i16, i16>,
    pub u26: [bool; 14],
    pub labyrinth_medals: LengthVec<i8, i16>,
    pub _111000: Assertable<111000>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV110800Block {
    pub cat_shrine_dialogs: i32,
    pub u1: bool,
    pub dojo_3x_speed: bool,
    pub u2: bool,
    pub u3: bool,
    pub _110800: Assertable<110800>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV110700Block {
    pub u1: HashMapLength<i32, i32, (f64, f64)>,
    #[rw(jp = false)]
    pub u2: Option<bool>,
    pub _110700: Assertable<110700>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV110600Block {
    pub unknown: bool,
    pub _110600: Assertable<110600>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV110500Block {
    pub behemoth_culling: GauntletChapters,
    pub unknown: bool,
    pub _110500: Assertable<110500>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV110000Block {
    pub u1: HashMapLength<i16, i32, (i8, i8)>,
    pub _110000: Assertable<110000>,
}

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
pub struct GV101000Block {
    pub uknown: i8,
    pub _101000: Assertable<101000>,
}

#[derive(Debug, Clone, Default)]
pub struct AkuChapters {
    pub current_stages: Vec<Vec<i8>>,
    pub stages: Vec<Vec<Vec<i16>>>,
}

impl Readable for AkuChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
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
    fn write<W: std::io::Write + Seek>(
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

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV100900Block {
    pub aku: AkuChapters,
    pub u1: bool,
    pub u2: bool,
    pub u3: HashMapLength<i16, i16, LengthVec<i16, i16>>,
    pub u4: HashMapLength<i16, i16, f64>,
    pub u5: HashMapLength<i16, i16, f64>,
    pub u6: bool,
    pub _100900: Assertable<100900>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV100700Block {
    pub u1: HashMapLength<i16, i16, bool>,
    pub u2: HashMapLength<i16, i16, HashMapLength<i16, i16, i16>>,
    #[rw(gvcc)]
    pub u3: UnknownDict90100,
    pub _100700: Assertable<100700>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100600Block {
    pub timestamp: f64,
    pub platinum_shards: i32,
    pub u2: bool,
    pub _100600: Assertable<100600>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV100400Block {
    pub event_capsules_2: LengthVec<i8, i32>,
    pub two_battle_lines: bool,
    pub _100400: Assertable<100400>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Unknown100300 {
    pub u1: bool,
    pub u2: bool,
    pub u3: i8,
    pub u4: f64,
    pub u5: f64,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100300Block {
    pub unknown: [Unknown100300; 6],
    pub _100300: Assertable<100300>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100100Block {
    pub date: i32,
    pub _100100: Assertable<100100>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV100000Block {
    pub legend_tickets: i32,
    pub u1: HashMapLength<i8, i8, i32>, // FIXME: may not be a hashmap
    pub u2: bool,
    pub u3: bool,
    pub password_refresh_token: LengthString<i32>,
    pub u4: bool,
    pub u5: i8,
    pub u6: i8,
    pub u7: f64,
    pub u8: f64,
    pub _100000: Assertable<100000>,
}

#[derive(Debug, Clone, Default)]
pub struct SlotNames {
    pub names: Vec<LengthString<i32>>,
}

impl Readable for SlotNames {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
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
    fn write<W: std::io::Write + Seek>(
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
pub struct GV91000Block {
    #[rw(gvcc)]
    pub slot_names: SlotNames,
    pub _91000: Assertable<91000>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct CatShrine {
    pub unknown: bool,
    pub stamp_1: f64,
    pub stamp_2: f64,
    pub shrine_gone: bool,
    pub flags: LengthVec<i8, i8>,
    pub xp_offering: i64,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90900Block {
    pub cat_shrine: CatShrine,
    pub u1: f64,
    pub u2: f64,
    pub _90900: Assertable<90900>,
}

#[derive(Debug, Clone)]
pub enum TalentOrbs {
    Old(HashMapLength<i16, i16, i8>),
    New(HashMapLength<i16, i16, i16>),
}

impl From<&HashMapLength<i16, i16, i8>> for HashMapLength<i16, i16, i16> {
    fn from(value: &HashMapLength<i16, i16, i8>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i16);
        }

        Self::new(new_map)
    }
}
impl From<&HashMapLength<i16, i16, i16>> for HashMapLength<i16, i16, i8> {
    fn from(value: &HashMapLength<i16, i16, i16>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i8);
        }

        Self::new(new_map)
    }
}

impl Default for TalentOrbs {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for TalentOrbs {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        match args.gv.0 {
            0..110400 => Ok(Self::Old(HashMapLength::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl Writable for TalentOrbs {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..110400 => match self {
                TalentOrbs::Old(hash_map_length) => hash_map_length.write_no_opts(writer)?,
                TalentOrbs::New(hash_map_length) => {
                    let other: HashMapLength<i16, i16, i8> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
            },
            _ => match self {
                TalentOrbs::Old(hash_map_length) => {
                    let other: HashMapLength<i16, i16, i16> = hash_map_length.into();

                    other.write_no_opts(writer)?;
                }
                TalentOrbs::New(hash_map_length) => hash_map_length.write_no_opts(writer)?,
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90800Block {
    pub u1: LengthVec<i16, i32>,
    pub u2: [bool; 10],
    pub _90800: Assertable<90800>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90700Block {
    #[rw(gvcc)]
    pub talent_orbs: TalentOrbs,
    pub unknown: HashMapLength<i16, i16, HashMapLength<i8, i8, i16>>,
    pub unknown_2: bool,
    pub _90700: Assertable<90700>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Unknown90500_100300 {
    pub u1: i8,
    pub u2: bool,
    pub timestamp1: f64,
    pub timestamp2: f64,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Unknown90500_130700 {
    pub u1: HashMapLength<i16, i32, i8>,
    pub u2: HashMapLength<i16, i32, f64>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90500Block {
    pub collab_gauntlets: GauntletChapters,
    pub u1: bool,
    pub timestamp1: f64,
    pub timestamp2: f64,
    pub u2: i32,
    #[rw(min_gv = 100300)]
    pub u3: Option<Unknown90500_100300>,
    #[rw(min_gv = 130700)]
    pub u4: Option<Unknown90500_130700>,
    #[rw(min_gv = 140100)]
    pub u5: Option<HashMapLength<i16, i32, f64>>,
    pub _90500: Assertable<90500>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct EnigmaStage {
    pub level: i32,
    pub stage_id: i32,
    pub decoding_status: i8,
    pub start_time: f64,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct ExtraEnigmaDataInner {
    pub u1: i32,
    pub u2: i32,
    pub u3: i8,
    pub u4: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExtraEnigmaData(pub Option<ExtraEnigmaDataInner>);

impl Readable for ExtraEnigmaData {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        match args.gv.0 {
            0..140500 => Ok(Self(None)),
            _ => {
                let has_extra = bool::read_no_opts(reader)?;

                match has_extra {
                    true => Ok(Self(Some(ExtraEnigmaDataInner::read_no_opts(reader)?))),
                    false => Ok(Self(None)),
                }
            }
        }
    }
}

impl Writable for ExtraEnigmaData {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..140500 => (),
            _ => match self.0 {
                Some(item) => {
                    true.write_no_opts(writer)?;
                    item.write_no_opts(writer)?;
                }
                None => {
                    false.write_no_opts(writer)?;
                }
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Engima {
    pub energy_since_1: i32,
    pub energy_since_2: i32,
    pub enigma_level: i8,
    pub unknown_1: i8,
    pub unknown_2: bool,
    pub stages: LengthVec<i8, EnigmaStage>,
    #[rw(gvcc)]
    pub extra_data: ExtraEnigmaData,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct CatSlot {
    pub cat_id: i16,
    pub form: Formi8,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct LineupCat {
    pub index: i16,
    pub cats: [CatSlot; 10],
    pub u1: i8,
    pub u2: i8,
    pub u3: i8,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct StageLineup {
    pub index: i16,
    pub stages: LengthVec<i16, i32>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct ClearedSlots {
    pub cats: LengthVec<i16, LineupCat>,
    pub stages: LengthVec<i16, StageLineup>,
    pub unknown: HashMapLength<i16, i16, bool>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90400Block {
    pub gauntlets_2: GauntletChapters,
    #[rw(gvcc)]
    pub enigma: Engima,
    pub cleared_slots: ClearedSlots,
    pub _90400: Assertable<90400>,
}

#[derive(Debug, Clone, Default)]
pub struct ChaptersGeneric<SEL, PROG, STAG, UNLCK> {
    pub selected_stages: Vec<Vec<SEL>>,
    pub clear_progress: Vec<Vec<PROG>>,
    pub stages: Vec<STAG>,
    pub unlock_state: Vec<Vec<UNLCK>>,
}
impl<SEL, PROG, STAG, UNLCK> ChaptersGeneric<SEL, PROG, STAG, UNLCK> {
    pub fn total_chapters(&self) -> usize {
        self.selected_stages.len()
    }

    pub fn total_stars(&self) -> usize {
        self.selected_stages.first().unwrap_or(&Vec::new()).len()
    }
}

pub trait TotalStages {
    fn total(&self) -> usize;
}

impl<T> TotalStages for Vec<Vec<T>> {
    fn total(&self) -> usize {
        self.len()
    }
}
impl TotalStages for LegendQuestStage {
    fn total(&self) -> usize {
        self.clear_times.len()
    }
}

impl<SEL, PROG, STAG: TotalStages + Default, UNLCK> ChaptersGeneric<SEL, PROG, STAG, UNLCK> {
    pub fn total_stages(&self) -> usize {
        self.stages.first().unwrap_or(&STAG::default()).total()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LengthType {
    I8,
    I16,
    I32,
}

impl LengthType {
    pub fn read_usize<R: std::io::Read + std::io::Seek>(
        &self,
        reader: &mut R,
    ) -> StreamResult<usize> {
        Ok(match self {
            LengthType::I8 => i8::read_no_opts(reader)? as usize,
            LengthType::I16 => i16::read_no_opts(reader)? as usize,
            LengthType::I32 => i32::read_no_opts(reader)? as usize,
        })
    }

    fn write_usize<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        length: usize,
    ) -> StreamResult<()> {
        match self {
            LengthType::I8 => (length as i8).write_no_opts(writer),
            LengthType::I16 => (length as i16).write_no_opts(writer),
            LengthType::I32 => (length as i32).write_no_opts(writer),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GenericChapterArgs {
    pub read_length_every_time: bool,
    pub total_chapters_type: LengthType,
    pub total_stages_type: LengthType,
    pub total_stars_type: LengthType,
}

impl GenericChapterArgs {
    pub fn new_int(read_length_every_time: bool) -> Self {
        Self {
            read_length_every_time,
            total_chapters_type: LengthType::I32,
            total_stages_type: LengthType::I32,
            total_stars_type: LengthType::I32,
        }
    }
}

impl<
    SEL: for<'a> Readable<Args<'a> = ()> + std::fmt::Debug,
    PROG: for<'a> Readable<Args<'a> = ()> + std::fmt::Debug,
    STAG: for<'a> Readable<Args<'a> = VecArgs<VecArgs<()>>> + std::fmt::Debug,
    UNLCK: for<'a> Readable<Args<'a> = ()> + std::fmt::Debug,
> Readable for ChaptersGeneric<SEL, PROG, STAG, UNLCK>
{
    type Args<'a> = GenericChapterArgs;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let (total_chapters, total_stages, total_stars) = match args.read_length_every_time {
            true => (
                args.total_chapters_type.read_usize(reader)?,
                0,
                args.total_stars_type.read_usize(reader)?,
            ),
            false => (
                args.total_chapters_type.read_usize(reader)?,
                args.total_stages_type.read_usize(reader)?,
                args.total_stars_type.read_usize(reader)?,
            ),
        };

        let selected_stages = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )
        .add_context(|| "read selected stages")?;

        let (total_chapters, total_stars) = match args.read_length_every_time {
            true => (
                args.total_chapters_type.read_usize(reader)?,
                args.total_stars_type.read_usize(reader)?,
            ),
            false => (total_chapters, total_stars),
        };

        let clear_progress = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )
        .add_context(|| "read clear progress")?;

        let (total_chapters, total_stages, total_stars) = match args.read_length_every_time {
            true => (
                args.total_chapters_type.read_usize(reader)?,
                args.total_stages_type.read_usize(reader)?,
                args.total_stars_type.read_usize(reader)?,
            ),
            false => (total_chapters, total_stages, total_stars),
        };

        let stages = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_stages),
                    item: VecArgs::new_empty_fixed(total_stars),
                },
            },
        )
        .add_context(|| "read clear amount")?;

        let (total_chapters, total_stars) = match args.read_length_every_time {
            true => (
                args.total_chapters_type.read_usize(reader)?,
                args.total_stars_type.read_usize(reader)?,
            ),
            false => (total_chapters, total_stars),
        };

        let unlock_state = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )
        .add_context(|| "read chapter unlock state")?;

        Ok(Self {
            selected_stages,
            clear_progress,
            stages,
            unlock_state,
        })
    }
}

impl<
    SEL: for<'a> Writable<Args<'a> = ()> + std::fmt::Debug + Default,
    PROG: for<'a> Writable<Args<'a> = ()> + Default + std::fmt::Debug,
    STAG: for<'a> Writable<Args<'a> = VecArgs<VecArgs<()>>> + TotalStages + Default + std::fmt::Debug,
    UNLCK: for<'a> Writable<Args<'a> = ()> + Default + std::fmt::Debug,
> Writable for ChaptersGeneric<SEL, PROG, STAG, UNLCK>
{
    type Args<'a> = GenericChapterArgs;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_chapters = self.total_chapters();
        let total_stages = self.total_stages();
        let total_stars = self.total_stars();

        match args.read_length_every_time {
            true => {
                args.total_chapters_type
                    .write_usize(writer, total_chapters)?;
                args.total_stars_type.write_usize(writer, total_stars)?;
            }
            false => {
                args.total_chapters_type
                    .write_usize(writer, total_chapters)?;
                args.total_stages_type.write_usize(writer, total_stages)?;
                args.total_stars_type.write_usize(writer, total_stars)?;
            }
        };

        self.selected_stages.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )?;

        if args.read_length_every_time {
            args.total_chapters_type
                .write_usize(writer, total_chapters)?;
            args.total_stars_type.write_usize(writer, total_stars)?;
        }

        self.clear_progress.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )?;

        if args.read_length_every_time {
            args.total_chapters_type
                .write_usize(writer, total_chapters)?;
            args.total_stages_type.write_usize(writer, total_stages)?;
            args.total_stars_type.write_usize(writer, total_stars)?;
        }

        self.stages.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_stages),
                    item: VecArgs::new_empty_fixed(total_stars),
                },
            },
        )?;

        if args.read_length_every_time {
            args.total_chapters_type
                .write_usize(writer, total_chapters)?;
            args.total_stars_type.write_usize(writer, total_stars)?;
        }

        self.unlock_state.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_chapters),
                item: VecArgs::new_empty_fixed(total_stars),
            },
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Unknown90300 {
    pub u1: i32,
    pub u2: i32,
    pub u3: i16,
    pub u4: i32,
    pub u5: i32,
    pub u6: i32,
    pub u7: i16,
}

#[derive(Debug, Clone, Default)]
pub struct GauntletChapters {
    pub chapters: ChaptersGeneric<i8, i8, StageClear<i16>, i8>,
    pub unknown: Vec<i8>,
}

impl Readable for GauntletChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        let chapters = ChaptersGeneric::read(
            reader,
            GenericChapterArgs {
                read_length_every_time: false,
                total_chapters_type: LengthType::I16,
                total_stages_type: LengthType::I8,
                total_stars_type: LengthType::I8,
            },
        )?;

        let total_chapters = chapters.total_chapters();

        Ok(Self {
            chapters,
            unknown: Vec::read(reader, VecArgs::new_empty_fixed(total_chapters))?,
        })
    }
}

impl Writable for GauntletChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters.write(
            writer,
            GenericChapterArgs {
                read_length_every_time: false,
                total_chapters_type: LengthType::I16,
                total_stages_type: LengthType::I8,
                total_stars_type: LengthType::I8,
            },
        )?;

        self.unknown.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.total_chapters()),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90300Block {
    pub unknown: LengthVec<i16, Unknown90300>,
    pub unknown_2: HashMapLength<i16, i32, f64>,
    pub gauntlet_chapters: GauntletChapters,
    pub _90300: Assertable<90300>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV90100Block {
    pub unknown_1: i16,
    pub unknown_2: i16,
    pub unknown_date: i32,
    pub unknown_timestamp: f64,
    pub _90100: Assertable<90100>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Medals {
    pub u1: i32,
    pub u2: i32,
    pub u3: i32,
    pub data_1: LengthVec<i16, i16>,
    pub data_2: HashMapLength<i16, i16, i8>,
    pub u4: bool,
}

#[derive(Debug, Clone)]
pub enum UnknownDict90100 {
    Old(HashMapLength<i16, i16, f64>),
    New(HashMapLength<i16, i16, i32>),
}

impl Default for UnknownDict90100 {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for UnknownDict90100 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        match args.gv.0 {
            0..90100 => Ok(Self::Old(HashMapLength::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl From<&HashMapLength<i16, i16, f64>> for HashMapLength<i16, i16, i32> {
    fn from(value: &HashMapLength<i16, i16, f64>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as i32);
        }

        Self::new(new_map)
    }
}
impl From<&HashMapLength<i16, i16, i32>> for HashMapLength<i16, i16, f64> {
    fn from(value: &HashMapLength<i16, i16, i32>) -> Self {
        let mut new_map = HashMap::with_capacity(value.0.len());

        for (k, v) in &value.0 {
            new_map.insert(*k, *v as f64);
        }

        Self::new(new_map)
    }
}

impl Writable for UnknownDict90100 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..90100 => match self {
                UnknownDict90100::Old(hash_map_length) => hash_map_length.write_no_opts(writer)?,
                UnknownDict90100::New(hash_map_length) => {
                    let other: HashMapLength<i16, i16, f64> = hash_map_length.into();
                    other.write_no_opts(writer)?;
                }
            },
            _ => match self {
                UnknownDict90100::Old(hash_map_length) => {
                    let other: HashMapLength<i16, i16, i32> = hash_map_length.into();
                    other.write_no_opts(writer)?;
                }
                UnknownDict90100::New(hash_map_length) => hash_map_length.write_no_opts(writer)?,
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV90000Block {
    pub medals: Medals,
    pub unkown: HashMapLength<i16, i16, bool>,
    pub unknown_2: HashMapLength<i16, i16, HashMapLength<i16, i16, i16>>,
    #[rw(gvcc)]
    pub unknown_3: UnknownDict90100,
    _90000: Assertable<90000>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV81000Block {
    pub restart_pack: i8,
    pub _81000: Assertable<81000>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV100600BlockEn {
    pub uknown: i8,
    pub _100600: Assertable<100600>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV80700Block {
    pub unknown: HashMapLength<i32, i32, LengthVec<i32, i32>>,
    pub _80700: Assertable<80700>,
}

#[derive(Debug, Clone, Default)]
pub struct LegendQuestStage {
    pub clear_times: Vec<Vec<i16>>,
    pub attemps: Vec<Vec<i16>>,
}

#[derive(Debug, Clone, Copy)]
pub struct LegendQuestStageArgs {
    pub total_stages: usize,
    pub total_stars: usize,
}

impl Readable for LegendQuestStage {
    type Args<'a> = VecArgs<VecArgs<()>>;

    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        // let args = VecArgs {
        //     length: VecArgsLength::Fixed(args.total_stages),
        //     item: VecArgs::new_empty_fixed(args.total_stars),
        // };
        Ok(Self {
            clear_times: Vec::read(reader, args)?,
            attemps: Vec::read(reader, args)?,
        })
    }
}

impl Writable for LegendQuestStage {
    type Args<'a> = VecArgs<VecArgs<()>>;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.clear_times.write(writer, args)?;
        self.attemps.write(writer, args)
    }
}

#[derive(Debug, Clone, Default)]
pub struct LegendQuestChapters {
    pub chapters: ChaptersGeneric<i8, i8, LegendQuestStage, i8>,
}

impl Readable for LegendQuestChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + Seek>(reader: &mut R, _args: Self::Args<'_>) -> StreamResult<Self> {
        Ok(Self {
            chapters: ChaptersGeneric::read(
                reader,
                GenericChapterArgs {
                    read_length_every_time: false,
                    total_chapters_type: LengthType::I8,
                    total_stages_type: LengthType::I8,
                    total_stars_type: LengthType::I8,
                },
            )?,
        })
    }
}

impl Writable for LegendQuestChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters.write(
            writer,
            GenericChapterArgs {
                read_length_every_time: false,
                total_chapters_type: LengthType::I8,
                total_stages_type: LengthType::I8,
                total_stars_type: LengthType::I8,
            },
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct LegendQuest {
    pub chapters: LegendQuestChapters,
    pub unknown: Vec<i8>,
    pub ids: Vec<i32>,
}

impl Readable for LegendQuest {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let chapters = LegendQuestChapters::read_no_opts(reader)?;
        let total_chapters = chapters.chapters.total_chapters();
        let total_stages = chapters.chapters.total_stages();

        Ok(Self {
            chapters,
            unknown: Vec::read(reader, VecArgs::new_empty_fixed(total_chapters))?,
            ids: Vec::read(reader, VecArgs::new_empty_fixed(total_stages))?,
        })
    }
}

impl Writable for LegendQuest {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters.write_no_opts(writer)?;

        self.unknown.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.chapters.total_chapters()),
        )?;
        self.ids.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.chapters.total_stages()),
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV80600Block {
    pub unknown_vec: LengthVec<i16, i32>,
    pub legend_quest_chapters: LegendQuest,
    pub uknown_short: i16,
    pub unknown_byte: i8,
    pub _80600: Assertable<80600>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV80500Block {
    pub unknown: LengthVec<i32, i32>,
    pub _80500: Assertable<80500>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV80300Block {
    pub filibuster_stage_id: i8,
    pub filibuster_stage_enabled: bool,
    pub _80300: Assertable<80300>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct GV80200Block {
    pub unknown: bool,
    pub leadership: i16,
    pub officer_cat_id: i16,
    pub officer_cat_form: Formi16,
    pub _80200: Assertable<80200>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GoldPass {
    pub officer_id: i32,
    pub total_renewal_times: i32,
    pub start_date_now: f64,
    pub end_date_now: f64,
    pub start_date_next: f64,
    pub end_date_next: f64,
    pub start_date_total: f64,
    pub end_date_total: f64,
    pub time_error_end: f64,
    pub total_state_updates: i32,
    pub login_bonus_date: f64,
    pub claimed_rewards: HashMapLength<i32, i32, i32>,
    pub remaining_days_popup: f64,
    pub first_popup_flag: bool,
    #[rw(min_gv = 80100)]
    pub badge_flag: Option<bool>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Talent {
    pub id: i32,
    pub level: i32,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV80000Block {
    #[rw(gvcc)]
    pub gold_pass: GoldPass,
    pub cat_talents: HashMapLength<i32, i32, LengthVec<i32, Talent>>,
    pub np: i32,
    pub unknown: bool,
    pub _80000: Assertable<80000>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV77Block {
    pub uncanny_chapters2: UncannyChapters,
    pub lucky_tickets: LengthVec<i32, i32>,
    pub unkown: bool,
    pub _77: Assertable<77>,
}

pub type StageClear<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, Default)]
pub struct UncannyChapters {
    pub chapters: ChaptersGeneric<i32, i32, StageClear<i32>, i32>,
    pub unknown: Vec<i32>,
}

impl Readable for UncannyChapters {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let chapters = ChaptersGeneric::read(reader, GenericChapterArgs::new_int(false))?;
        let len = chapters.selected_stages.len();
        Ok(Self {
            chapters,
            unknown: Vec::read(reader, VecArgs::new_empty_fixed(len))?,
        })
    }
}

impl Writable for UncannyChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(false))?;

        self.unknown.write(
            writer,
            VecArgs::new_empty_fixed(self.chapters.selected_stages.len()),
        )
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV76Block {
    pub uncanny_chapters: UncannyChapters,
    pub _76: Assertable<76>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct MapResetData {
    pub yearly_end_timestamp: f64,
    pub monthly_end_timestamp: f64,
    pub weekly_end_timestamp: f64,
    pub daily_end_timestamp: f64,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV72Block {
    pub map_resets: HashMapLength<i32, i32, LengthVec<i32, MapResetData>>,
    pub _72: Assertable<72>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV71Block {
    pub gamatoto_collab_flags: HashMapLength<i32, i32, bool>,
    pub gamatoto_collab_durations: HashMapLength<i32, i32, f64>,
    pub _71: Assertable<71>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV69Block {
    pub event_completed_one_level_in_chapter: HashMapLength<i32, i32, i32>,
    pub event_displayed_cleared_limit_text: HashMapLength<i32, i32, bool>,
    pub event_start_dates: HashMapLength<i32, i32, i32>,
    pub stages_reward_claimed: LengthVec<i32, i32>,
    pub cotc_1_complete: i32,
    pub _69: Assertable<69>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV68Block {
    pub weekly_missions: HashMapLength<i32, i32, bool>,
    pub dojo_ranking_did_win_rewards: bool,
    pub event_update: bool,
    pub _68: Assertable<68>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct DojoRanking {
    pub score: i32,
    pub ranking: i32,
    pub has_submitted: bool,
    pub has_completed: bool,
    pub has_seen_results: bool,
    pub start_date: i32,
    pub end_date: i32,
    pub event_number: i32,
    pub should_show_rank_description: bool,
    pub should_show_start_message: bool,
    pub submit_error_flag: bool,
    #[rw(min_gv = 140500)]
    pub other: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct ChallengeChapters {
    pub chapters: ChaptersGeneric<i32, i32, StageClear<i32>, i32>,
}

impl Readable for ChallengeChapters {
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

impl Writable for ChallengeChapters {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(true))
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV67Block {
    #[rw(gvcc)]
    pub ranking: DojoRanking,
    pub item_pack_three_days_started: bool,
    pub item_pack_three_days_end: f64,
    pub challenge: ChallengeChapters,
    pub challenge_scores: LengthVec<i32, i32>,
    pub show_challenge_popup: bool,
    pub _67: Assertable<67>,
}

#[derive(Debug, Clone, Default)]
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
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.chapters
            .write(writer, GenericChapterArgs::new_int(true))
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
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
    fn write<W: std::io::Write + Seek>(
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

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV66Block {
    pub tower: TowerChapters,
    #[rw(gvcc)]
    pub missions: Missions,
    pub tower_item_obtain_states: TowerItemObtainStates,
    pub _66: Assertable<66>,
}

#[derive(Debug, Clone, Default)]
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
    fn write<W: std::io::Write + Seek>(
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
pub struct GV65Block {
    pub h1: HashMapLength<i32, i32, i32>,
    pub h2: HashMapLength<i32, i32, LengthVec<i32, LengthString<i32>>>,
    pub h3: HashMapLength<i32, i32, bool>,
    pub _65: Assertable<65>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV64Block {
    pub base_materials: LengthVec<i32, i32>,
    #[rw(gvcc)]
    pub ototo: Ototo,
    pub _64: Assertable<64>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct Ototo {
    pub remaining_seconds: f64,
    pub return_flag: bool,
    pub improve_id: i32,
    pub engineers: i32,
    pub cannon_levels: HashMapLength<i32, i32, LengthVec<i32, i32>>,
    #[rw(gvcc)]
    pub selected_parts: OtotoSelectedParts,
    pub last_checked_castle_time: f64,
}

#[derive(Debug, Clone)]
pub enum OtotoSelectedParts {
    Old([i32; 3]),
    New(Vec<[i8; 3]>),
}

impl Default for OtotoSelectedParts {
    fn default() -> Self {
        Self::New(Vec::new())
    }
}

impl Readable for OtotoSelectedParts {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..80200 => Ok(Self::Old(<[i32; 3]>::read_no_opts(reader)?)),
            80200..90700 => Ok(Self::New(Vec::read(reader, VecArgs::new_empty_fixed(10))?)),
            _ => Ok(Self::New(Vec::read(reader, VecArgs::new_empty_i8())?)),
        }
    }
}

impl Writable for OtotoSelectedParts {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..80200 => match self {
                OtotoSelectedParts::Old(o) => o.write_no_opts(writer)?,
                OtotoSelectedParts::New(items) => {
                    items.first().unwrap_or(&[0; 3]).write_no_opts(writer)?
                }
            },
            80200..90700 => match self {
                OtotoSelectedParts::Old(o) => {
                    o.to_vec().write(writer, VecArgs::new_empty_fixed(10))?
                }
                OtotoSelectedParts::New(items) => {
                    items.write(writer, VecArgs::new_empty_fixed(10))?
                }
            },
            _ => match self {
                OtotoSelectedParts::Old(items) => {
                    items.to_vec().write(writer, VecArgs::new_empty_i8())?
                }
                OtotoSelectedParts::New(items) => items.write(writer, VecArgs::new_empty_i8())?,
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV63Block {
    pub unlock_popups: HashMapLength<i32, i32, bool>,
    pub _63: Assertable<63>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV61Block {
    pub new_chara_flags: HashMapLength<i32, i32, i32>,
    pub shown_maxcollab_msg: bool,
    pub displayed_packs: HashMapLength<i32, i32, bool>,
    pub _61: Assertable<61>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV60Block {
    #[rw(max_gv = 43)]
    pub old_current_outbreaks: Option<HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>>,
    pub current_outbreaks: HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>,
    pub first_locks: HashMapLength<i32, i32, bool>,
    pub energy_penalty_timestamp: f64,
    pub _60: Assertable<60>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV59Block {
    pub last_checked_zombie_time: f64,
    pub outbreaks: HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>,
    pub zombie_event_remaining_time: f64,
    pub scheme_items_to_obtain: LengthVec<i32, i32>,
    pub scheme_items_received: LengthVec<i32, i32>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV58Block {
    pub dojo_chapters: HashMapLength<i32, i32, HashMapLength<i32, i32, i32>>,
    pub dojo_item_lock_flag: bool,
    pub dojo_item_locks: [bool; TOTAL_BATTLE_ITEMS],
    pub _58: Assertable<58>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV57Block {
    pub unknown: bool,
    pub favourite_cats: HashMapLength<i32, i32, bool>,
    pub _57: Assertable<57>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV56Block {
    pub uknown: bool,
    pub item_reward_item_obtains: HashMapLength<i32, i32, HashMapLength<i32, i32, bool>>,
    pub item_reward_unobtained_sets: HashMapLength<i32, i32, bool>,
    pub stepup_gatya_stages: HashMapLength<i32, i32, i32>,
    pub stepup_gatya_durations: HashMapLength<i32, i32, f64>,
    pub backup_frame: i32,
    pub _56: Assertable<56>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV55Block {
    pub gamatoto_skin: i32,
    pub platinum_tickets: i32,
    #[rw(gvcc)]
    pub logins: LoginBonus,
    #[rw(max_gv = 100999)]
    pub reset_item_reward_flags: Option<LengthVec<i32, bool>>,
    pub reward_remaining_time: f64,
    pub last_checked_reward_time: f64,
    pub announcements: [(i32, i32); 16],
    pub backup_counter: i32,
    pub uknown: [i32; 3],
    pub _55: Assertable<55>,
}

#[derive(Debug, Clone)]
pub enum LoginBonus {
    Old(LengthVec<i32, i32>),
    New(HashMapLength<i32, i32, i32>),
}

impl Default for LoginBonus {
    fn default() -> Self {
        Self::New(HashMapLength::default())
    }
}

impl Readable for LoginBonus {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        match args.gv.0 {
            0..80000 => Ok(Self::Old(LengthVec::read_no_opts(reader)?)),
            _ => Ok(Self::New(HashMapLength::read_no_opts(reader)?)),
        }
    }
}

impl Writable for LoginBonus {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        // we don't need to convert any incorrect data types here since they will write to
        // approximately the same thing anyway
        match self {
            LoginBonus::Old(length_vec) => length_vec.write_no_opts(writer)?,
            LoginBonus::New(hash_map_length) => hash_map_length.write_no_opts(writer)?,
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV54Block {
    pub gamatoto_helpers: LengthVec<i32, i32>,
    pub is_ad_present: bool,
    pub _54: Assertable<54>,
    pub item_pack: HashMapLength<i32, i32, HashMapLength<i32, LengthString<i32>, bool>>,
    pub _54_2: Assertable<54>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV53Block {
    pub time_since_last_check_cumulative: f64,
    pub server_timestamp: f64,
    pub last_checked_energy_recovery_time: f64,
    pub time_since_last_check: f64,
    pub last_checked_expedition_time: f64,
    pub catfruit: LengthVec<i32, i32>,
    pub cat_fourth_forms: LengthVec<i32, i32>,
    pub cat_catseyes_used: LengthVec<i32, i32>,
    pub catseyes: LengthVec<i32, i32>,
    pub catamins: LengthVec<i32, i32>,
    pub gamatoto: Gamatoto,
    pub unlock_popups: LengthVec<i32, bool>,
    pub ex_stages: LengthVec<i32, [i32; 12]>,
    pub _53: Assertable<53>,
}

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
pub struct Gamatoto {
    pub remaining_seconds: f64,
    pub return_flag: bool,
    pub xp: i32,
    pub dest_id: i32,
    pub recon_length: i32,
    pub unknown: i32,
    pub notif_value: i32,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV52Block {
    pub catguide_collected: LengthVec<i32, bool>,
    pub _52: Assertable<52>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
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

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV47Block {
    #[rw(gvcc)]
    pub event_seed: GatyaSeed,
    #[rw(gvcc)]
    pub event_capsules: EventCapsules,
    pub _47: Assertable<47>,
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV46Block {
    pub _46: Assertable<46>,
}

#[derive(Debug, Clone, Readable, Writable)]
pub struct GV45Block {
    pub itf1_complete: i32,
    pub itf_timed_scores: [[i32; 51]; 3],
    pub title_chapter_bg: i32,
    #[rw(min_gv = 27)]
    pub combo_unlocks: Option<LengthVec<i32, i32>>,
    pub combo_unlocked_10k_ur: bool,
    pub _45: Assertable<45>,
}

impl Default for GV45Block {
    fn default() -> Self {
        Self {
            itf_timed_scores: [[0; 51]; 3],
            itf1_complete: 0,
            title_chapter_bg: 0,
            combo_unlocks: None,
            combo_unlocked_10k_ur: false,
            _45: Assertable,
        }
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct GV44Block {
    #[rw(gvcc)]
    pub item_reward_chapters: ItemRewardChapters<bool>,
    #[rw(gvcc)]
    pub timed_score_chapters: ItemRewardChapters<i32>,
    pub inquiry_code: LengthString<i32>,
    pub play_time: i32,
    pub has_account: i8,
    pub backup_state: i32,
    #[rw(jp = false)]
    pub ub2: Option<bool>,
    pub _44: Assertable<44>,
}

#[derive(Debug, Clone, Default)]
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
    fn write<W: std::io::Write + Seek>(
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

#[derive(Debug, Clone, Default)]
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
    fn write<W: std::io::Write + Seek>(
        &self,
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

#[derive(Debug, Clone, Default)]
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
    fn write<W: std::io::Write + Seek>(
        &self,
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

#[derive(Debug, Clone, Default)]
pub struct UserRankRewards {
    pub rewards: Vec<bool>,
}

impl Readable for UserRankRewards {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..30 => VecArgs::new_empty_fixed(30),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(Self {
            rewards: Vec::read(reader, length)?,
        })
    }
}

impl Writable for UserRankRewards {
    type Args<'a> = GVCC;

    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..30 => VecArgs::new_empty_fixed(30),
            _ => VecArgs::new_empty_i32(),
        };

        self.rewards.write(writer, length)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ubl1(pub Option<Vec<bool>>);

impl Readable for Ubl1 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..20 => {
                return Ok(Self(None));
            }
            20..=25 => VecArgs::new_empty_fixed(12),
            26..39 => VecArgs::new_empty_i32(),
            _ => {
                return Ok(Self(None));
            }
        };

        Ok(Self(Some(Vec::read(reader, length)?)))
    }
}

impl Writable for Ubl1 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..20 => {
                return Ok(());
            }
            20..=25 => VecArgs::new_empty_fixed(12),
            26..39 => VecArgs::new_empty_i32(),
            _ => {
                return Ok(());
            }
        };

        self.0.as_ref().unwrap_or(&Vec::new()).write(writer, length)
    }
}

#[derive(Debug, Clone, Copy, Readable, Writable)]
pub struct GatyaData {
    pub stepup_stage_3_cooldown: i32,
    pub previous_normal_roll: i32,
    pub previous_normal_roll_type: i32,
    pub previous_rare_rool: i32,
    pub previous_rare_roll_type: i32,
    pub unknown: bool,
    #[rw(min_gv = 2)]
    pub roll_single: Option<bool>,
    #[rw(min_gv = 2)]
    pub roll_multi: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct EventStageLegendRestriction {
    pub legend_restriction: Vec<Vec<i32>>,
}

impl Readable for EventStageLegendRestriction {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let (total_map_types, total_subchapters) = match args.gv.0 {
            0..33 => (3, 150),
            33..41 => (4, 150),
            _ => (i32::read_no_opts(reader)?, i32::read_no_opts(reader)?),
        };

        Ok(Self {
            legend_restriction: Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::Fixed(total_map_types as usize),
                    item: VecArgs::new_empty_fixed(total_subchapters as usize),
                },
            )?,
        })
    }
}

impl Writable for EventStageLegendRestriction {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_map_types = self.legend_restriction.len();
        let total_subchapters = self.legend_restriction.first().unwrap_or(&Vec::new()).len();
        let (total_map_types, total_subchapters) = match args.gv.0 {
            0..33 => (3, 150),
            33..41 => (4, 150),
            _ => {
                (total_map_types as i32).write_no_opts(writer)?;
                (total_subchapters as i32).write_no_opts(writer)?;
                (total_map_types, total_subchapters)
            }
        };

        self.legend_restriction.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types),
                item: VecArgs::new_empty_fixed(total_subchapters),
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnlockedSlots {
    Individual([bool; 10]),
    One(i8),
}

impl Readable for UnlockedSlots {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(match args.gv.0 {
            0..90700 => Self::Individual(<[bool; 10]>::read_no_opts(reader)?),
            _ => Self::One(i8::read_no_opts(reader)?),
        })
    }
}

impl Writable for UnlockedSlots {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..90700 => match self {
                UnlockedSlots::Individual(i) => i.write_no_opts(writer)?,
                UnlockedSlots::One(o) => {
                    let mut individual = [false; 10];

                    let times = std::cmp::min(10, *o as usize);

                    for item in individual.iter_mut().take(times) {
                        *item = true;
                    }

                    individual.write_no_opts(writer)?;
                }
            },
            _ => match self {
                UnlockedSlots::Individual(i) => {
                    let one: i8 = i.iter().filter(|item| **item).count() as i8;

                    one.write_no_opts(writer)?;
                }
                UnlockedSlots::One(o) => o.write_no_opts(writer)?,
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct SomeTimeInfo {
    pub timestamp: f64,
    pub server_time_stamp: f64,
    pub get_time_save: f64,
    pub usl1: LengthVec<i32, LengthString<i32>>,
    pub energy_notification: bool,
    pub full_game_version: i32,
}

#[derive(Debug, Clone, Default)]
pub struct UnitDrops {
    pub unit_drops: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Copy)]
pub enum GatyaSeed {
    Old(u64),
    New(u32),
}

impl Default for GatyaSeed {
    fn default() -> Self {
        Self::New(0)
    }
}

impl Readable for GatyaSeed {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        Ok(match args.gv.0 {
            0..33 => Self::Old(u64::read_no_opts(reader)?),
            _ => Self::New(u32::read_no_opts(reader)?),
        })
    }
}

impl Writable for GatyaSeed {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        match args.gv.0 {
            0..33 => match self {
                GatyaSeed::Old(o) => o.write_no_opts(writer)?,
                GatyaSeed::New(n) => (*n as u64).write_no_opts(writer)?,
            },
            _ => match self {
                GatyaSeed::Old(o) => (*o as u32).write_no_opts(writer)?,
                GatyaSeed::New(n) => n.write_no_opts(writer)?,
            },
        };

        Ok(())
    }
}

impl Readable for UnitDrops {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            ..=25 => VecArgs::new_empty_fixed(110),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(Self {
            unit_drops: Some(Vec::read(reader, length)?),
        })
    }
}

impl Writable for UnitDrops {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            ..=25 => VecArgs::new_empty_fixed(110),
            _ => VecArgs::new_empty_i32(),
        };

        self.unit_drops
            .as_ref()
            .unwrap_or(&Vec::new())
            .write(writer, length)
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventChapter<T> {
    pub data: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Default)]
pub struct EventChaptersT<T, T2> {
    pub selected_stages: Vec<Vec<Vec<T>>>,
    pub clear_progress: Vec<Vec<Vec<T>>>,
    pub clear_amounts: Vec<Vec<Vec<Vec<T2>>>>,
    pub unlock_state: Vec<Vec<Vec<T>>>,
}

impl Writable for EventChaptersT<i8, i16> {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_map_types = self.get_total_map_types();
        let total_subchapters = self.get_total_subchapters();
        let stars_per_subchapter = self.get_stars_per_subchapter();
        let stages_per_subchapter = self.get_stages_per_subchapter();

        (total_map_types as i8).write_no_opts(writer)?;
        (total_subchapters as i16).write_no_opts(writer)?;
        (stars_per_subchapter as i8).write_no_opts(writer)?;
        (stages_per_subchapter as i8).write_no_opts(writer)?;

        let args1 = VecArgs {
            length: VecArgsLength::Fixed(total_map_types),
            item: VecArgs {
                length: VecArgsLength::Fixed(total_subchapters),
                item: VecArgs::new_empty_fixed(stars_per_subchapter),
            },
        };

        self.selected_stages.write(writer, args1)?;
        self.clear_progress.write(writer, args1)?;

        self.clear_amounts.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters),
                    item: VecArgs {
                        length: VecArgsLength::Fixed(stages_per_subchapter),
                        item: VecArgs::new_empty_fixed(stars_per_subchapter),
                    },
                },
            },
        )?;

        self.unlock_state.write(writer, args1)?;

        Ok(())
    }
}

impl<T1, T2> EventChaptersT<T1, T2> {
    pub fn get_total_map_types(&self) -> usize {
        self.selected_stages.len()
    }
    pub fn get_total_subchapters(&self) -> usize {
        self.selected_stages.first().unwrap_or(&Vec::new()).len()
    }
    pub fn get_stars_per_subchapter(&self) -> usize {
        self.selected_stages
            .first()
            .unwrap_or(&Vec::new())
            .first()
            .unwrap_or(&Vec::new())
            .len()
    }
    pub fn get_stages_per_subchapter(&self) -> usize {
        self.clear_amounts
            .first()
            .unwrap_or(&Vec::new())
            .first()
            .unwrap_or(&Vec::new())
            .len()
    }
}

impl Readable for EventChaptersT<i8, i16> {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let total_map_types = i8::read_no_opts(reader)?;
        let total_subchapters = i16::read_no_opts(reader)?;
        let stars_per_subchapter = i8::read_no_opts(reader)?;
        let stages_per_subchapter = i8::read_no_opts(reader)?;

        let args1 = VecArgs {
            length: VecArgsLength::Fixed(total_map_types as usize),
            item: VecArgs {
                length: VecArgsLength::Fixed(total_subchapters as usize),
                item: VecArgs::new_empty_fixed(stars_per_subchapter as usize),
            },
        };

        let selected_stages = Vec::read(reader, args1)?;

        let clear_progress = Vec::read(reader, args1)?;

        let clear_amounts = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types as usize),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters as usize),
                    item: VecArgs {
                        length: VecArgsLength::Fixed(stages_per_subchapter as usize),
                        item: VecArgs::new_empty_fixed(stars_per_subchapter as usize),
                    },
                },
            },
        )?;

        let unlock_state = Vec::read(reader, args1)?;

        Ok(Self {
            selected_stages,
            clear_progress,
            clear_amounts,
            unlock_state,
        })
    }
}

impl Readable for EventChaptersT<i32, i32> {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let (total_map_types, total_subchapters, stars_per_subchapter) = match args.gv.0 {
            ..=5 => (3, 50, 1),
            6 => (3, 150, 1),
            7..=32 => (3, 150, 3),
            33..=34 => (4, 150, 3),
            _ => (
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
            ),
        };

        let selected_stages = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types as usize),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters as usize),
                    item: VecArgs::new_empty_fixed(stars_per_subchapter as usize),
                },
            },
        )?;

        let (total_map_types, total_subchapters, stars_per_subchapter) = match args.gv.0 {
            ..=34 => (total_map_types, total_subchapters, stars_per_subchapter),
            _ => (
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
            ),
        };

        let clear_progress = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types as usize),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters as usize),
                    item: VecArgs::new_empty_fixed(stars_per_subchapter as usize),
                },
            },
        )?;

        let (total_map_types, total_subchapters, stars_per_subchapter, stages_per_subchapter) =
            match args.gv.0 {
                ..=34 => (total_map_types, total_subchapters, stars_per_subchapter, 12),
                _ => (
                    i32::read_no_opts(reader)?,
                    i32::read_no_opts(reader)?,
                    i32::read_no_opts(reader)?,
                    i32::read_no_opts(reader)?,
                ),
            };

        let clear_amounts = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types as usize),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters as usize),
                    item: VecArgs {
                        length: VecArgsLength::Fixed(stars_per_subchapter as usize),
                        item: VecArgs::new_empty_fixed(stages_per_subchapter as usize),
                    },
                },
            },
        )?;

        let (total_map_types, total_subchapters, stars_per_subchapter) = match args.gv.0 {
            ..=34 => (total_map_types, total_subchapters, stars_per_subchapter),
            _ => (
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
                i32::read_no_opts(reader)?,
            ),
        };

        let unlock_state = Vec::read(
            reader,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types as usize),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters as usize),
                    item: VecArgs::new_empty_fixed(stars_per_subchapter as usize),
                },
            },
        )?;

        Ok(Self {
            selected_stages,
            clear_progress,
            clear_amounts,
            unlock_state,
        })
    }
}

impl Writable for EventChaptersT<i32, i32> {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_map_types = self.get_total_map_types();
        let total_subchapters = self.get_total_subchapters();
        let stars_per_subchapter = self.get_stars_per_subchapter();
        let stages_per_subchapter = self.get_stages_per_subchapter();
        let (total_map_types, total_subchapters, stars_per_subchapter) = match args.gv.0 {
            ..=5 => (3, 50, 1),
            6 => (3, 150, 1),
            7..=32 => (3, 150, 3),
            33..=34 => (4, 150, 3),
            _ => {
                (total_map_types as i32).write_no_opts(writer)?;
                (total_subchapters as i32).write_no_opts(writer)?;
                (stars_per_subchapter as i32).write_no_opts(writer)?;

                (total_map_types, total_subchapters, stars_per_subchapter)
            }
        };

        self.selected_stages.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters),
                    item: VecArgs::new_empty_fixed(stars_per_subchapter),
                },
            },
        )?;

        let (total_map_types, total_subchapters, stars_per_subchapter) = match args.gv.0 {
            ..=34 => (total_map_types, total_subchapters, stars_per_subchapter),
            _ => {
                (total_map_types as i32).write_no_opts(writer)?;
                (total_subchapters as i32).write_no_opts(writer)?;
                (stars_per_subchapter as i32).write_no_opts(writer)?;

                (total_map_types, total_subchapters, stars_per_subchapter)
            }
        };

        self.clear_progress.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters),
                    item: VecArgs::new_empty_fixed(stars_per_subchapter),
                },
            },
        )?;

        let (total_map_types, total_subchapters, stars_per_subchapter, stages_per_subchapter) =
            match args.gv.0 {
                ..=34 => (total_map_types, total_subchapters, stars_per_subchapter, 12),
                _ => {
                    (total_map_types as i32).write_no_opts(writer)?;
                    (total_subchapters as i32).write_no_opts(writer)?;
                    (stars_per_subchapter as i32).write_no_opts(writer)?;
                    (stages_per_subchapter as i32).write_no_opts(writer)?;

                    (
                        total_map_types,
                        total_subchapters,
                        stars_per_subchapter,
                        stages_per_subchapter,
                    )
                }
            };

        self.clear_amounts.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters),
                    item: VecArgs {
                        length: VecArgsLength::Fixed(stars_per_subchapter),
                        item: VecArgs::new_empty_fixed(stages_per_subchapter),
                    },
                },
            },
        )?;

        let (total_map_types, total_subchapters, stars_per_subchapter) = match args.gv.0 {
            ..=34 => (total_map_types, total_subchapters, stars_per_subchapter),
            _ => {
                (total_map_types as i32).write_no_opts(writer)?;
                (total_subchapters as i32).write_no_opts(writer)?;
                (stars_per_subchapter as i32).write_no_opts(writer)?;

                (total_map_types, total_subchapters, stars_per_subchapter)
            }
        };

        self.unlock_state.write(
            writer,
            VecArgs {
                length: VecArgsLength::Fixed(total_map_types),
                item: VecArgs {
                    length: VecArgsLength::Fixed(total_subchapters),
                    item: VecArgs::new_empty_fixed(stars_per_subchapter),
                },
            },
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum EventChapters {
    Int(EventChaptersT<i32, i32>),
    Byte(EventChaptersT<i8, i16>),
}

impl Default for EventChapters {
    fn default() -> Self {
        Self::Int(EventChaptersT::default())
    }
}

impl From<&EventChaptersT<i32, i32>> for EventChaptersT<i8, i16> {
    fn from(value: &EventChaptersT<i32, i32>) -> Self {
        Self {
            selected_stages: value
                .selected_stages
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| s.iter().map(|t| *t as i8).collect())
                        .collect()
                })
                .collect(),
            clear_progress: value
                .clear_progress
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| s.iter().map(|t| *t as i8).collect())
                        .collect()
                })
                .collect(),
            clear_amounts: value
                .clear_amounts
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| {
                            s.iter()
                                .map(|t| t.iter().map(|v| *v as i16).collect())
                                .collect()
                        })
                        .collect()
                })
                .collect(),
            unlock_state: value
                .unlock_state
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| s.iter().map(|t| *t as i8).collect())
                        .collect()
                })
                .collect(),
        }
    }
}
impl From<&EventChaptersT<i8, i16>> for EventChaptersT<i32, i32> {
    fn from(value: &EventChaptersT<i8, i16>) -> Self {
        Self {
            selected_stages: value
                .selected_stages
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| s.iter().map(|t| *t as i32).collect())
                        .collect()
                })
                .collect(),
            clear_progress: value
                .clear_progress
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| s.iter().map(|t| *t as i32).collect())
                        .collect()
                })
                .collect(),
            clear_amounts: value
                .clear_amounts
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| {
                            s.iter()
                                .map(|t| t.iter().map(|v| *v as i32).collect())
                                .collect()
                        })
                        .collect()
                })
                .collect(),
            unlock_state: value
                .unlock_state
                .iter()
                .map(|r| {
                    r.iter()
                        .map(|s| s.iter().map(|t| *t as i32).collect())
                        .collect()
                })
                .collect(),
        }
    }
}

impl Readable for EventChapters {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let is_int = args.gv.0 < 80100;

        match is_int {
            true => Ok(Self::Int(EventChaptersT::read(reader, args)?)),
            false => Ok(Self::Byte(EventChaptersT::read_no_opts(reader)?)),
        }
    }
}

impl Writable for EventChapters {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let is_int = args.gv.0 < 80100;
        match self {
            EventChapters::Int(event_chapters_t) => match is_int {
                true => event_chapters_t.write(writer, args)?,
                false => {
                    let other_chapters: EventChaptersT<i8, i16> = event_chapters_t.into();
                    other_chapters.write_no_opts(writer)?;
                }
            },
            EventChapters::Byte(event_chapters_t) => match is_int {
                true => {
                    let other_chapters: EventChaptersT<i32, i32> = event_chapters_t.into();
                    other_chapters.write(writer, args)?;
                }
                false => event_chapters_t.write_no_opts(writer)?,
            },
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CatStorage {
    pub item_ids: Vec<i32>,
    pub item_types: Vec<i32>,
}

impl Readable for CatStorage {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let total_storage = if args.gv.0 < 110100 {
            100
        } else {
            i16::read_no_opts(reader).add_context(|| "read total cat storage")?
        };

        let item_ids = Vec::read(reader, VecArgs::new_empty_fixed(total_storage as usize))
            .add_context(|| "read item ids")?;
        let item_types = Vec::read(reader, VecArgs::new_empty_fixed(total_storage as usize))
            .add_context(|| "read item types")?;

        Ok(Self {
            item_ids,
            item_types,
        })
    }
}

impl Writable for CatStorage {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let total_storage = if args.gv.0 < 110100 {
            100
        } else {
            (self.item_ids.len() as i16)
                .write_no_opts(writer)
                .add_context(|| "write total cat storage")?;

            self.item_ids.len()
        };

        self.item_ids
            .write(writer, VecArgs::new_empty_fixed(total_storage))?;
        self.item_types
            .write(writer, VecArgs::new_empty_fixed(total_storage))
    }
}

#[derive(Debug, Clone, Readable, Writable)]
pub struct BonusHash {
    pub unknown_1: HashMapLength<VariableLengthInt, VariableLengthInt, VariableLengthInt>,
    pub unknown_2: HashMapLength<VariableLengthInt, VariableLengthInt, u8>,
}

#[derive(Debug, Copy, Clone, Readable, Writable)]
pub struct StoryTreasureFestival {
    pub time_until_chance: [i32; TOTAL_STORY_CHAPTERS],
    pub duration: [i32; TOTAL_STORY_CHAPTERS],
    pub value: [i32; TOTAL_STORY_CHAPTERS],
    pub stage: [i32; TOTAL_STORY_CHAPTERS],
    pub festival_type: [i32; TOTAL_STORY_CHAPTERS],
}

#[derive(Debug, Copy, Clone, Readable, Writable)]
pub struct LockedBattleItems {
    pub lock_item: bool,
    pub locked_items: [bool; TOTAL_BATTLE_ITEMS],
}

#[derive(Debug, Clone, Default)]
pub struct UnknownEarlyBoolList {
    pub data: Vec<bool>,
}

impl Readable for UnknownEarlyBoolList {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + Seek>(reader: &mut R, args: Self::Args<'_>) -> StreamResult<Self> {
        let length = match args.gv.0 {
            0 | 1 => VecArgs::new_empty_fixed(39),
            2 | 3 | 4 => VecArgs::new_empty_fixed(69),
            _ => VecArgs::new_empty_fixed(0),
        };

        Ok(Self {
            data: Vec::read(reader, length)?,
        })
    }
}

impl Writable for UnknownEarlyBoolList {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0 | 1 => VecArgs::new_empty_fixed(39),
            2 | 3 | 4 => VecArgs::new_empty_fixed(69),
            _ => VecArgs::new_empty_fixed(0),
        };
        self.data.write(writer, length)
    }
}

pub const TOTAL_BATTLE_ITEMS: usize = 6;

#[derive(Debug, Clone, Default)]
pub struct NewDialogs {
    pub new_dialogs: Vec<i32>,
}

impl Readable for NewDialogs {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..10 => VecArgs::new_empty_fixed(12),
            10..27 => VecArgs::new_empty_fixed(17),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(Self {
            new_dialogs: <Vec<i32>>::read(reader, length).add_context(|| "read new dialogs")?,
        })
    }
}

impl Writable for NewDialogs {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..10 => VecArgs::new_empty_fixed(12),
            10..27 => VecArgs::new_empty_fixed(17),
            _ => VecArgs::new_empty_i32(),
        };

        self.new_dialogs.write(writer, length)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MenuUnlocks1 {
    pub unlocks: Vec<i32>,
    pub unlock_popups: Vec<i32>,
}

impl Readable for MenuUnlocks1 {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..=25 => VecArgs::new_empty_fixed(5),
            26 => VecArgs::new_empty_fixed(6),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(Self {
            unlocks: <Vec<i32>>::read(reader, length)?,
            unlock_popups: <Vec<i32>>::read(reader, length)?,
        })
    }
}

impl Writable for MenuUnlocks1 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..=25 => VecArgs::new_empty_fixed(5),
            26 => VecArgs::new_empty_fixed(6),
            _ => VecArgs::new_empty_i32(),
        };

        self.unlocks.write(writer, length)?;
        self.unlock_popups.write(writer, length)
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum Form {
    #[default]
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Formi32(pub Form);
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Formi16(pub Form);
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub struct Formi8(pub Form);

impl From<Form> for i32 {
    fn from(val: Form) -> i32 {
        match val {
            Form::First => 0,
            Form::Second => 1,
            Form::Third => 2,
            Form::Fourth => 3,
        }
    }
}

impl TryFrom<i8> for Form {
    type Error = String;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        (value as i32).try_into()
    }
}

impl TryFrom<i16> for Form {
    type Error = String;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        (value as i32).try_into()
    }
}

impl TryFrom<i32> for Form {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Form::First,
            1 => Form::Second,
            2 => Form::Third,
            3 => Form::Fourth,
            _ => {
                return Err(format!("invalid form type: {value}"));
            }
        })
    }
}

impl Readable for Formi32 {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let pos = reader.stream_position()?;
        let form = i32::read_no_opts(reader).add_context(|| "read form")?;

        Ok(Self(form.try_into().map_err(|e| {
            StreamError::new(std::io::Error::other(e), pos)
        })?))
    }
}
impl Readable for Formi16 {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let pos = reader.stream_position()?;
        let form = i16::read_no_opts(reader).add_context(|| "read form")?;

        Ok(Self(form.try_into().map_err(|e| {
            StreamError::new(std::io::Error::other(e), pos)
        })?))
    }
}
impl Readable for Formi8 {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let pos = reader.stream_position()?;
        let form = i8::read_no_opts(reader).add_context(|| "read form")?;

        Ok(Self(form.try_into().map_err(|e| {
            StreamError::new(std::io::Error::other(e), pos)
        })?))
    }
}

impl Writable for Formi8 {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let val: i32 = self.0.into();

        (val as i8).write_no_opts(writer)
    }
}
impl Writable for Formi16 {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let val: i32 = self.0.into();

        (val as i16).write_no_opts(writer)
    }
}
impl Writable for Formi32 {
    type Args<'a> = ();
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let val: i32 = self.0.into();

        val.write_no_opts(writer)
    }
}

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
pub struct Upgrade {
    pub plus: i16,
    pub base: i16,
}

#[derive(Debug, Clone, Default)]
pub struct CatsField<T>(pub T);

pub fn get_total_cats_from_gv(gv: GameVersion) -> Option<usize> {
    Some(match gv.0 {
        1 => 88,
        2 | 3 | 4 => 122,
        5 => 144,
        6 => 172,
        7 | 8 => 179,
        9 => 185,
        20 => 203,
        21 => 214,
        22 => 231,
        23 => 241,
        24 => 249,
        _ => return None,
    })
}

impl<T: Readable> Readable for CatsField<T>
where
    T: for<'a> Readable<Args<'a> = VecArgs<()>>,
{
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let args = if let Some(t) = get_total_cats_from_gv(args.gv) {
            VecArgs::new_empty_fixed(t)
        } else {
            VecArgs::new_empty_i32()
        };

        Ok(CatsField(
            T::read(reader, args).add_context(|| "read cat data")?,
        ))
    }
}

impl<T: for<'a> Writable<Args<'a> = VecArgs<()>>> Writable for CatsField<T> {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let args = if let Some(t) = get_total_cats_from_gv(args.gv) {
            VecArgs::new_empty_fixed(t)
        } else {
            VecArgs::new_empty_i32()
        };

        self.0.write(writer, args)
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnemyGuide {
    pub enemy_guide: Vec<i32>,
}

impl Readable for EnemyGuide {
    type Args<'a> = GVCC;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..6 => VecArgs::new_empty_fixed(131),
            6..26 => VecArgs::new_empty_fixed(231),
            _ => VecArgs::new_empty_i32(),
        };

        Ok(EnemyGuide {
            enemy_guide: <Vec<i32>>::read(reader, length).add_context(|| "read enemy guide")?,
        })
    }
}

impl Writable for EnemyGuide {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..6 => VecArgs::new_empty_fixed(131),
            6..26 => VecArgs::new_empty_fixed(231),
            _ => VecArgs::new_empty_i32(),
        };

        self.enemy_guide.write(writer, length)
    }
}

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
pub struct LineUp {
    pub slots: [i32; 10],
}

#[derive(Debug, Clone, Default)]
pub struct LineUps {
    pub lineups: Vec<LineUp>,
}

impl Readable for LineUps {
    type Args<'a> = GVCC;

    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> crate::stream::StreamResult<Self> {
        let length = match args.gv.0 {
            0..5 => VecArgs::new_empty_fixed(1),
            5..10 => VecArgs::new_empty_fixed(3),
            _ => VecArgs::new_empty_i8(),
        };

        let lineups = Vec::read(reader, length)?;

        Ok(Self { lineups })
    }
}

impl Writable for LineUps {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0..5 => VecArgs::new_empty_fixed(1),
            5..10 => VecArgs::new_empty_fixed(3),
            _ => VecArgs::new_empty_i8(),
        };

        self.lineups.write(writer, length)
    }
}

#[derive(Debug, Copy, Clone, Readable, Writable)]
pub struct StampData {
    pub current_stamp: i32,
    pub collected_stamps: [i32; 30],
    pub unknown: i32,
    pub daily_reward: i32,
}

pub const TOTAL_STORY_CHAPTERS: usize = 10;

#[derive(Debug, Copy, Clone, Readable, Writable)]
pub struct StoryChapters {
    pub selected_stages: [i32; TOTAL_STORY_CHAPTERS],
    pub chapter_progress: [i32; TOTAL_STORY_CHAPTERS],
    pub clear_times: [[i32; 51]; TOTAL_STORY_CHAPTERS],
    pub treasures: [[i32; 49]; TOTAL_STORY_CHAPTERS],
}

use std::{io::Cursor, path::Path};

use crate::{
    blocks::{gv_47::GatyaSeed, gv_58::TOTAL_BATTLE_ITEMS, *},
    country_code::CountryCode,
    game::main_story::{StoryChapters, TOTAL_STORY_CHAPTERS},
    game_version::GameVersion,
    hash::{add_hash, detect_cc},
    stream::{
        HashMapLength, LengthString, LengthVec, NewResultCtx, Readable, ReadableNoOptions,
        StreamError, StreamResult, VariableLengthInt, VecArgs, VecArgsLength, Writable,
        WritableNoOptions,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let length = match args.gv.0 {
            0 | 1 => 3,
            2..=4 => 4,
            5 => 5,
            6..=9 => 6,
            _ => 36,
        };

        Ok(Self {
            popups: Vec::read(reader, VecArgs::new_empty_fixed(length))?,
        })
    }
}

impl Writable for UnlockPopups8 {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0 | 1 => 3,
            2..=4 => 4,
            5 => 5,
            6..=9 => 6,
            _ => 36,
        };

        self.popups.write(writer, VecArgs::new_empty_fixed(length))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SaveFile {
    pub save: Save,
    pub gvcc: GVCC,
}

impl SaveFile {
    pub fn load_detect_cc(data: &[u8]) -> StreamResult<SaveFile> {
        let cc = detect_cc(data).ok_or(StreamError::new_str(
            "could not detect country code",
            u64::MAX,
        ))?;

        let mut reader = Cursor::new(data);

        SaveFile::read(&mut reader, cc.into())
    }
    pub fn load_cc(data: &[u8], cc: CountryCode) -> StreamResult<SaveFile> {
        let mut reader = Cursor::new(data);

        SaveFile::read(&mut reader, cc)
    }

    pub fn write_with_hash(&self) -> StreamResult<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());
        self.write_no_opts(&mut writer)?;

        let data = writer.into_inner();

        let data = add_hash(&data, self.gvcc.cc.into())
            .ok_or(StreamError::new_str("failed to add hash", u64::MAX))?;

        Ok(data)
    }

    pub fn write_to_path(&self, path: &Path) -> StreamResult<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = self.write_with_hash()?;

        std::fs::write(path, &data)?;

        Ok(())
    }

    pub fn load_from_path_detect_cc(path: &Path) -> StreamResult<SaveFile> {
        let data = std::fs::read(path)?;

        Self::load_detect_cc(&data)
    }

    pub fn load_from_path_cc(path: &Path, cc: CountryCode) -> StreamResult<SaveFile> {
        let data = std::fs::read(path)?;

        Self::load_cc(&data, cc)
    }
}

impl Writable for SaveFile {
    type Args<'a> = ();

    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.gvcc.gv.write_no_opts(writer)?;
        self.save.write(writer, self.gvcc)?;

        Ok(())
    }
}

impl Readable for SaveFile {
    type Args<'a> = CountryCode;
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let gv = GameVersion::read_no_opts(reader)?;

        let gvcc = GVCC { gv, cc: args };

        let save = Save::read(reader, gvcc)?;

        Ok(Self { save, gvcc })
    }
}

#[derive(Debug, Clone, Readable, Writable, Default)]
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
    pub gv_44: Option<gv_44::GV44Block>,
    #[rw(gvcc, min_gv = 19)]
    pub gv_45: Option<gv_45::GV45Block>,
    #[rw(min_gv = 20)]
    pub gv_46: Option<gv_46::GV46Block>,
    #[rw(gvcc, min_gv = 21)]
    pub gv_47: Option<gv_47::GV47Block>,
    #[rw(min_gv = 22)]
    pub gv_48: Option<gv_48::GV48Block>,
    #[rw(gvcc, min_gv = 23)]
    pub gv_49: Option<gv_49::GV49Block>,
    #[rw(min_gv = 24)]
    pub gv_50: Option<gv_50::GV50Block>,
    #[rw(min_gv = 25)]
    pub gv_51: Option<gv_51::GV51Block>,
    #[rw(min_gv = 26)]
    pub gv_52: Option<gv_52::GV52Block>,
    #[rw(min_gv = 27)]
    pub gv_53: Option<gv_53::GV53Block>,
    #[rw(min_gv = 29)]
    pub gv_54: Option<gv_54::GV54Block>,
    #[rw(gvcc, min_gv = 30)]
    pub gv_55: Option<gv_55::GV55Block>,
    #[rw(min_gv = 31)]
    pub gv_56: Option<gv_56::GV56Block>,
    #[rw(min_gv = 32)]
    pub gv_57: Option<gv_57::GV57Block>,
    #[rw(min_gv = 33)]
    pub gv_58: Option<gv_58::GV58Block>,
    #[rw(min_gv = 34)]
    pub gv_59: Option<gv_59::GV59Block>,
    #[rw(gvcc, min_gv = 35)]
    pub gv_60: Option<gv_60::GV60Block>,
    #[rw(min_gv = 36)]
    pub gv_61: Option<gv_61::GV61Block>,
    #[rw(min_gv = 38)]
    pub gv_63: Option<gv_63::GV63Block>,
    #[rw(gvcc, min_gv = 39)]
    pub gv_64: Option<gv_64::GV64Block>,
    #[rw(min_gv = 40)]
    pub gv_65: Option<gv_65::GV65Block>,
    #[rw(gvcc, min_gv = 41)]
    pub gv_66: Option<gv_66::GV66Block>,
    #[rw(gvcc, min_gv = 42)]
    pub gv_67: Option<gv_67::GV67Block>,
    #[rw(min_gv = 43)]
    pub gv_68: Option<gv_68::GV68Block>,
    #[rw(min_gv = 44)]
    pub gv_69: Option<gv_69::GV69Block>,
    #[rw(min_gv = 46)]
    pub gv_71: Option<gv_71::GV71Block>,
    #[rw(min_gv = 47, max_gv = 90299)]
    pub gv_72: Option<gv_72::GV72Block>,
    #[rw(min_gv = 51)]
    pub gv_76: Option<gv_76::GV76Block>,
    #[rw(min_gv = 77)]
    pub gv_77: Option<gv_77::GV77Block>,
    #[rw(gvcc, min_gv = 80000)]
    pub gv_80000: Option<gv_80000::GV80000Block>,
    #[rw(min_gv = 80200)]
    pub gv_80200: Option<gv_80200::GV80200Block>,
    #[rw(min_gv = 80300)]
    pub gv_80300: Option<gv_80300::GV80300Block>,
    #[rw(min_gv = 80500)]
    pub gv_80500: Option<gv_80500::GV80500Block>,
    #[rw(min_gv = 80600)]
    pub gv_80600: Option<gv_80600::GV80600Block>,
    #[rw(min_gv = 80700)]
    pub gv_80700: Option<gv_80700::GV80700Block>,
    #[rw(min_gv = 100600, jp = false, kr = false, tw = false)]
    pub gv_100600_en: Option<gv_100600::GV100600BlockEn>,
    #[rw(min_gv = 81000)]
    pub gv_81000: Option<gv_81000::GV81000Block>,
    #[rw(gvcc, min_gv = 90000)]
    pub gv_90000: Option<gv_90000::GV90000Block>,
    #[rw(min_gv = 90100)]
    pub gv_90100: Option<gv_90100::GV90100Block>,
    #[rw(min_gv = 90300)]
    pub gv_90300: Option<gv_90300::GV90300Block>,
    #[rw(gvcc, min_gv = 90400)]
    pub gv_90400: Option<gv_90400::GV90400Block>,
    #[rw(gvcc, min_gv = 90500)]
    pub gv_90500: Option<gv_90500::GV90500Block>,
    #[rw(gvcc, min_gv = 90700)]
    pub gv_90700: Option<gv_90700::GV90700Block>,
    #[rw(min_gv = 90800)]
    pub gv_90800: Option<gv_90800::GV90800Block>,
    #[rw(min_gv = 90900)]
    pub gv_90900: Option<gv_90900::GV90900Block>,
    #[rw(gvcc, min_gv = 91000)]
    pub gv_91000: Option<gv_91000::GV91000Block>,
    #[rw(min_gv = 100000)]
    pub gv_100000: Option<gv_100000::GV100000Block>,
    #[rw(min_gv = 100100)]
    pub gv_100100: Option<gv_100100::GV100100Block>,
    #[rw(min_gv = 100300)]
    pub gv_100300: Option<gv_100300::GV100300Block>,
    #[rw(min_gv = 100400)]
    pub gv_100400: Option<gv_100400::GV100400Block>,
    #[rw(min_gv = 100600)]
    pub gv_100600: Option<gv_100600::GV100600Block>,
    #[rw(gvcc, min_gv = 100700)]
    pub gv_100700: Option<gv_100700::GV100700Block>,
    #[rw(min_gv = 100900)]
    pub gv_100900: Option<gv_100900::GV100900Block>,
    #[rw(min_gv = 101000)]
    pub gv_101000: Option<gv_101000::GV101000Block>,
    #[rw(min_gv = 110000)]
    pub gv_110000: Option<gv_110000::GV110000Block>,
    #[rw(min_gv = 110500)]
    pub gv_110500: Option<gv_110500::GV110500Block>,
    #[rw(min_gv = 110600)]
    pub gv_110600: Option<gv_110600::GV110600Block>,
    #[rw(gvcc, min_gv = 110700)]
    pub gv_110700: Option<gv_110700::GV110700Block>,
    #[rw(min_gv = 110800)]
    pub gv_110800: Option<gv_110800::GV110800Block>,
    #[rw(min_gv = 111000)]
    pub gv_111000: Option<gv_111000::GV111000Block>,
    #[rw(min_gv = 120000)]
    pub gv_120000: Option<gv_120000::GV120000Block>,
    #[rw(min_gv = 120100)]
    pub gv_120100: Option<gv_120100::GV120100Block>,
    #[rw(min_gv = 120200)]
    pub gv_120200: Option<gv_120200::GV120200Block>,
    #[rw(min_gv = 120400)]
    pub gv_120400: Option<gv_120400::GV120400Block>,
    #[rw(min_gv = 120500)]
    pub gv_120500: Option<gv_120500::GV120500Block>,
    #[rw(min_gv = 120600)]
    pub gv_120600: Option<gv_120600::GV120600Block>,
    #[rw(gvcc)]
    pub gv_120700: gv_120700::GV120700Block,
    #[rw(min_gv = 130100)]
    pub gv_130100: Option<gv_130100::GV130100Block>,
    #[rw(min_gv = 130301)]
    pub gv_130301: Option<gv_130301::GV130301Block>,
    #[rw(min_gv = 130400)]
    pub gv_130400: Option<gv_130400::GV130400Block>,
    #[rw(min_gv = 130500)]
    pub gv_130500: Option<gv_130500::GV130500Block>,
    #[rw(gvcc, min_gv = 130600)]
    pub gv_130600: Option<gv_130600::GV130600Block>,
    #[rw(gvcc, min_gv = 130700)]
    pub gv_130700: Option<gv_130700::GV130700Block>,
    #[rw(min_gv = 140000)]
    pub gv_140000: Option<gv_140000::GV140000Block>,
    #[rw(min_gv = 140100, max_gv = 140499)]
    pub gv_140100: Option<gv_140100::GV140100Block>,
    #[rw(gvcc, min_gv = 140200)]
    pub gv_140200: Option<gv_140200::GV140200Block>,
    #[rw(min_gv = 140300)]
    pub gv_140300: Option<gv_140300::GV140300Block>,
    pub remaing_data: RemainingData,
}

impl Save {
    pub fn get_inquiry_code(&self) -> Option<&str> {
        Some(&self.gv_44.as_ref()?.inquiry_code.0)
    }
    pub fn get_inquiry_code_with_default(&self, default: String) -> String {
        self.get_inquiry_code()
            .map(|v| v.to_string())
            .unwrap_or(default)
    }
}

#[derive(Debug, Clone, Default)]
pub struct RemainingData {
    pub data: Vec<u8>,
}

impl Readable for RemainingData {
    type Args<'a> = ();
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _args: Self::Args<'_>,
    ) -> StreamResult<Self> {
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
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _args: Self::Args<'_>,
    ) -> StreamResult<()> {
        self.data
            .write(writer, VecArgs::new_empty_fixed(self.data.len()))
    }
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
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
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

        let selected_stages = NewResultCtx::add_context(
            Vec::read(
                reader,
                VecArgs {
                    length: VecArgsLength::Fixed(total_chapters),
                    item: VecArgs::new_empty_fixed(total_stars),
                },
            ),
            || "read selected stages",
        )?;

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
    fn write<W: std::io::Write + std::io::Seek>(
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

pub type StageClear<T> = Vec<Vec<T>>;

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

    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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

#[derive(Debug, Clone, Copy, Readable, Writable, Default)]
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
    fn write<W: std::io::Write + std::io::Seek>(
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

impl Default for UnlockedSlots {
    fn default() -> Self {
        Self::One(0)
    }
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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

#[derive(Debug, Clone, Readable, Writable, Default)]
pub struct BonusHash {
    pub unknown_1: HashMapLength<VariableLengthInt, VariableLengthInt, VariableLengthInt>,
    pub unknown_2: HashMapLength<VariableLengthInt, VariableLengthInt, u8>,
}

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
pub struct StoryTreasureFestival {
    pub time_until_chance: [i32; TOTAL_STORY_CHAPTERS],
    pub duration: [i32; TOTAL_STORY_CHAPTERS],
    pub value: [i32; TOTAL_STORY_CHAPTERS],
    pub stage: [i32; TOTAL_STORY_CHAPTERS],
    pub festival_type: [i32; TOTAL_STORY_CHAPTERS],
}

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
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
    fn read<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        args: Self::Args<'_>,
    ) -> StreamResult<Self> {
        let length = match args.gv.0 {
            0 | 1 => VecArgs::new_empty_fixed(39),
            2..=4 => VecArgs::new_empty_fixed(69),
            _ => VecArgs::new_empty_fixed(0),
        };

        Ok(Self {
            data: Vec::read(reader, length)?,
        })
    }
}

impl Writable for UnknownEarlyBoolList {
    type Args<'a> = GVCC;
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        args: Self::Args<'_>,
    ) -> StreamResult<()> {
        let length = match args.gv.0 {
            0 | 1 => VecArgs::new_empty_fixed(39),
            2..=4 => VecArgs::new_empty_fixed(69),
            _ => VecArgs::new_empty_fixed(0),
        };
        self.data.write(writer, length)
    }
}

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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
        2..=4 => 122,
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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
    fn write<W: std::io::Write + std::io::Seek>(
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

#[derive(Debug, Copy, Clone, Readable, Writable, Default)]
pub struct StampData {
    pub current_stamp: i32,
    pub collected_stamps: [i32; 30],
    pub unknown: i32,
    pub daily_reward: i32,
}

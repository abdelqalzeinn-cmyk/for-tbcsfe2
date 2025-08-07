use crate::{network::account_info::SaveFileAccount, ui::editview::BasicItem};

#[derive(Debug, Copy, Clone, Default)]
pub struct CatfoodView {}

impl BasicItem for CatfoodView {
    fn get_save_value(save_file: &SaveFileAccount) -> i32 {
        save_file.save_file.save.catfood
    }
    fn set_save_value(save_file: &mut SaveFileAccount, value: i32) {
        save_file.save_file.save.catfood = value;
    }
    fn feature() -> super::editview::BasicItemFeature {
        super::editview::BasicItemFeature::Catfood
    }
    fn max_value() -> i32 {
        45_000 // TODO: check this
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct XPView {}

impl BasicItem for XPView {
    fn get_save_value(save_file: &SaveFileAccount) -> i32 {
        save_file.save_file.save.xp
    }
    fn set_save_value(save_file: &mut SaveFileAccount, value: i32) {
        save_file.save_file.save.xp = value;
    }
    fn feature() -> super::editview::BasicItemFeature {
        super::editview::BasicItemFeature::Xp
    }
    fn max_value() -> i32 {
        99_999_999
    }
}

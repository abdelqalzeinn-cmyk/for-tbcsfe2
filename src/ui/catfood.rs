use crate::{
    edits::{CatfoodEdit, Edit, EditMemory, XPEdit},
    network::account_info::SaveFileAccount,
    ui::editview::BasicItem,
};

#[derive(Debug, Copy, Clone, Default)]
pub struct CatfoodView {
    pub old: i32,
}

impl BasicItem for CatfoodView {
    fn get_save_value(save_file: &SaveFileAccount) -> i32 {
        save_file.save_file.save.catfood
    }
    fn set_save_value(new: i32, old: i32) -> crate::edits::Edit {
        CatfoodEdit(EditMemory::new(new, old)).into()
    }

    fn feature() -> super::editview::BasicItemFeature {
        super::editview::BasicItemFeature::Catfood
    }
    fn max_value() -> i32 {
        45_000 // TODO: check this
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct XPView {
    pub old: i32,
}

impl BasicItem for XPView {
    fn get_save_value(save_file: &SaveFileAccount) -> i32 {
        save_file.save_file.save.xp
    }
    fn set_save_value(new: i32, old: i32) -> Edit {
        XPEdit(EditMemory::new(new, old)).into()
    }
    fn feature() -> super::editview::BasicItemFeature {
        super::editview::BasicItemFeature::Xp
    }
    fn max_value() -> i32 {
        99_999_999
    }
}

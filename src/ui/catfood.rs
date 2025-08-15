use crate::{
    edits::{Edit, EditMemory, basic_items::XPEdit},
    network::account_info::SaveFileAccount,
    save::Save,
    ui::editview::BasicItem,
};

macro_rules! impl_basic_item {
    ($type:ident, $getter:expr, $edit_type:expr, $feature:expr, $max:expr, $dt:ty) => {
        #[derive(Debug, Copy, Clone, Default)]
        pub struct $type {}

        impl BasicItem for $type {
            fn get_save_value(save_file: &SaveFileAccount) -> i32 {
                $getter(&save_file.save_file.save) as i32
            }
            fn set_save_value(new: i32, old: i32) -> crate::edits::Edit {
                $edit_type(EditMemory::new(new as $dt, old as $dt)).into()
            }

            fn feature() -> super::editview::BasicItemFeature {
                $feature
            }
            fn max_value() -> i32 {
                $max as i32
            }
        }
    };
}

impl_basic_item!(
    CatfoodView,
    Save::get_catfood,
    crate::edits::basic_items::CatfoodEdit,
    super::editview::BasicItemFeature::Catfood,
    45_000, // TODO: check this
    i32
);

impl_basic_item!(
    XPView,
    Save::get_xp,
    crate::edits::basic_items::XPEdit,
    super::editview::BasicItemFeature::Xp,
    99999999,
    i32
);

impl_basic_item!(
    NormalTicketView,
    Save::get_normal_tickets,
    crate::edits::basic_items::NormalTicketEdit,
    super::editview::BasicItemFeature::NormalTickets,
    9999, // TODO: check this
    i32
);
impl_basic_item!(
    RareTicketView,
    Save::get_rare_tickets,
    crate::edits::basic_items::RareTicketEdit,
    super::editview::BasicItemFeature::RareTickets,
    299, // TODO: check this
    i32
);
impl_basic_item!(
    PlatinumTicketView,
    |s| Save::get_platinum_tickets(s).unwrap_or_default(),
    crate::edits::basic_items::PlatinumTicketEdit,
    super::editview::BasicItemFeature::PlatinumTickets,
    9, // TODO: check this
    i32
);
impl_basic_item!(
    LegendTicketView,
    |s| Save::get_legend_tickets(s).unwrap_or_default(),
    crate::edits::basic_items::LegendTicketEdit,
    super::editview::BasicItemFeature::LegendTickets,
    4,
    i32
);
impl_basic_item!(
    NPView,
    |s| Save::get_np(s).unwrap_or_default(),
    crate::edits::basic_items::NPEdit,
    super::editview::BasicItemFeature::NP,
    9999,
    i32
);
impl_basic_item!(
    LeadershipView,
    |s| Save::get_leadership(s).unwrap_or_default(),
    crate::edits::basic_items::LeadershipEdit,
    super::editview::BasicItemFeature::Leadership,
    9999,
    i16
);

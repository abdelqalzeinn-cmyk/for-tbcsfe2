use std::fmt::Display;

use crate::{
    edits::{Applyable, Edit, EditMemory, EditReadable},
    save::{Save, SaveFile},
};

macro_rules! basic_item {
    ($inner:ty, $type:ident => $variant:ident => $getter:expr => $setter:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $type(pub EditMemory<$inner, $inner>);

        impl EditReadable for $type {
            fn read(save_file: &SaveFile) -> Self {
                Self(EditMemory::init_same($getter(&save_file.save)))
            }
        }

        impl Applyable for $type {
            fn apply(&self, save_file: &mut SaveFile) {
                $setter(&mut save_file.save, self.0.new.clone());
            }
            fn revert(&self, save_file: &mut SaveFile) {
                $setter(&mut save_file.save, self.0.old.clone());
            }
        }

        impl From<$type> for Edit {
            fn from(value: $type) -> Self {
                Self::$variant(value)
            }
        }

        impl Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

basic_item!(i32, CatfoodEdit => Catfood => Save::get_catfood => Save::set_catfood);
basic_item!(i32, XPEdit => XP => Save::get_xp => Save::set_xp);
basic_item!(i32, NormalTicketEdit => NormalTickets => Save::get_normal_tickets => Save::set_normal_tickets);
basic_item!(i32, RareTicketEdit => RareTickets => Save::get_rare_tickets => Save::set_rare_tickets);
basic_item!(String, InquiryCodeEdit => InquiryCode => |save| Save::get_inquiry_code_with_default(save, "".to_string()) => Save::set_inquiry_code);

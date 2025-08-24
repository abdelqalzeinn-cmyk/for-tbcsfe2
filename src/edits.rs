use std::fmt::Display;

use crate::save::SaveFile;

pub mod basic_items;
pub mod main_story;

pub type EditMemoryi32 = EditMemory<i32, i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditMemory<N, O> {
    pub new: N,
    pub old: O,
}

impl<N, O> EditMemory<N, O> {
    pub fn new(new: N, old: O) -> Self {
        Self { new, old }
    }

    pub fn init_same(val: N) -> Self
    where
        N: Into<O> + Clone,
    {
        Self::new(val.clone(), val.into())
    }
    pub fn swap(self) -> Self
    where
        N: Into<O>,
        O: Into<N>,
    {
        let tmp = self.new;
        Self {
            new: self.old.into(),
            old: tmp.into(),
        }
    }
}

impl<N: Display, O: Display> Display for EditMemory<N, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.old, self.new)
    }
}

#[derive(Debug, Clone)]
pub enum Edit {
    Catfood(basic_items::CatfoodEdit),
    XP(basic_items::XPEdit),
    InquiryCode(basic_items::InquiryCodeEdit),
    NormalTickets(basic_items::NormalTicketEdit),
    RareTickets(basic_items::RareTicketEdit),
    MainStory(main_story::StoryChaptersEdit),
    PlatinumTickets(basic_items::PlatinumTicketEdit),
    LegendTickets(basic_items::LegendTicketEdit),
    NP(basic_items::NPEdit),
    Leadership(basic_items::LeadershipEdit),
}

#[cfg(feature = "localization")]
impl crate::localization::Localizable for Edit {
    fn localize_with_args(
        &self,
        manager: &crate::localization::LocaleManager,
        _args: &fluent::FluentArgs,
    ) -> String {
        self.localize_edit(manager)
    }
}
macro_rules! name_localize {
    [$($var:ident => $name:literal),+] => {
        pub fn get_name(&self) -> String {
            match self {
                $(Self::$var(_) => $name,)+
            }.to_string()
        }

        #[cfg(feature = "localization")]
        pub fn localize_edit(&self, manager: &crate::localization::LocaleManager) -> String {
            match self {
                $(Self::$var(v) => crate::localization::Localizable::localize(v, manager),)+
            }
        }
    };
}

impl Edit {
    name_localize![
        Catfood => "catfood",
        XP => "xp",
        MainStory => "main-story",
        InquiryCode => "inquiry-code",
        NormalTickets => "normal-tickets",
        RareTickets => "rare-tickets",
        PlatinumTickets => "platinum-tickets",
        LegendTickets => "legend-tickets",
        NP => "np",
        Leadership => "leadership"
    ];
}

pub trait EditReadable {
    fn read(save_file: &SaveFile) -> Self;
}

pub trait Applyable {
    fn apply(&self, save_file: &mut SaveFile);
    fn revert(&self, save_file: &mut SaveFile);

    #[cfg(feature = "network")]
    fn add_managed_item(&self, _save_file: &mut crate::network::account_info::SaveFileAccount) {}
}

macro_rules! apply_revert {
    [$($var:ident),+] => {
        fn apply(&self, save_file: &mut SaveFile) {
            match self {
                $(Self::$var(v) => v.apply(save_file),)+
            }
        }

        fn revert(&self, save_file: &mut SaveFile) {
            match self {
                $(Self::$var(v) => v.revert(save_file),)+
            }
        }

        #[cfg(feature = "network")]
        fn add_managed_item(&self, save_file: &mut crate::network::account_info::SaveFileAccount) {
            match self {
                $(Self::$var(v) => v.add_managed_item(save_file),)+
            }
        }

    };
}

impl Applyable for Edit {
    apply_revert![
        Catfood,
        XP,
        MainStory,
        InquiryCode,
        NormalTickets,
        RareTickets,
        PlatinumTickets,
        LegendTickets,
        NP,
        Leadership
    ];
}

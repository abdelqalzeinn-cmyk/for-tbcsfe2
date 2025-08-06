use adb_client::DeviceShort;
use iced::{Element, Length, Task, alignment::Vertical, widget::container::bordered_box};

use crate::{
    adb::{adb_handler::AdbGameHandler, waydroid_handler::WaydroidGameHandler},
    ext_source::ExternalSaveSource,
    ui::{
        helper::labeled_box,
        localization::{LocaleManager, Localizable},
    },
};

#[derive(Debug, Default, Clone)]
pub struct AdbView {
    pub available_devices: Vec<DeviceShort>,
    pub selected_device: Option<DeviceShort>,
    pub available_pkgs: Vec<String>,
    pub is_waydroid: bool,
}

pub enum AdbCommand {
    Pull,
}

impl AdbView {
    pub fn new(is_waydroid: bool) -> Self {
        let mut s = Self::default();

        s.is_waydroid = is_waydroid;

        s
    }
    pub fn view(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, AdbMessage> {
        let mut devcol = Vec::new();
        devcol.push(
            iced::widget::button(iced::widget::text("refresh".localize(locale_manager)))
                .on_press(AdbMessage::FetchDevices)
                .into(),
        );
        for dev in &self.available_devices {
            let state_str = dev.state.to_string();
            let button: Element<'_, AdbMessage> =
                iced::widget::button(iced::widget::text(&dev.identifier))
                    .on_press(AdbMessage::SelectedDevice(dev.clone()))
                    .into();
            let row = iced::widget::row([
                button,
                iced::widget::text(state_str)
                    .align_y(Vertical::Center)
                    .height(Length::Fill)
                    .into(),
            ])
            .height(Length::Shrink)
            .spacing(10);
            devcol.push(
                iced::widget::container(row)
                    .padding(10)
                    .style(bordered_box)
                    .into(),
            );
        }
        let device_box = labeled_box(
            theme,
            "devices".localize(locale_manager),
            iced::widget::column(devcol).spacing(10).into(),
        );

        let mut cols = vec![device_box];

        if let Some(ref sel) = self.selected_device {
            let sel_box = labeled_box(
                theme,
                "selected-device".localize(locale_manager),
                iced::widget::row([
                    iced::widget::text(&sel.identifier).into(),
                    iced::widget::text(sel.state.to_string()).into(),
                ])
                .spacing(10)
                .into(),
            );
            cols.push(sel_box);

            let mut pkg_els = Vec::new();

            for pkg in &self.available_pkgs {
                pkg_els.push(
                    iced::widget::button(iced::widget::text(pkg))
                        .on_press(AdbMessage::PkgClicked(pkg.to_string()))
                        .into(),
                );
            }

            cols.push(labeled_box(
                theme,
                "packages".localize(locale_manager),
                iced::widget::column(pkg_els).spacing(10).into(),
            ))
        }

        labeled_box(
            theme,
            "adb-pull".localize(locale_manager),
            iced::widget::column(cols).spacing(10).into(),
        )
    }

    pub fn update(
        &mut self,
        message: AdbMessage,
        locale_manager: &LocaleManager,
    ) -> iced::Task<AdbMessage> {
        match message {
            AdbMessage::AvailableDevices(device_shorts) => self.available_devices = device_shorts,
            AdbMessage::FetchDevices => {
                let mut manager = AdbGameHandler::new();
                let devices = manager.get_devices();

                match devices {
                    Ok(d) => {
                        return iced::Task::done(AdbMessage::AvailableDevices(d));
                    }
                    Err(e) => return iced::Task::done(AdbMessage::Error(e.to_string())),
                }
            }
            AdbMessage::Error(_) => panic!("error must be handled further up!"),
            AdbMessage::LoadSave(items, _) => panic!("load save must be handled further up!"),
            AdbMessage::SelectedDevice(device_short) => {
                self.selected_device = Some(device_short.clone());

                if self.is_waydroid {
                    return Task::perform(
                        async move {
                            let mut manager = WaydroidGameHandler::new();
                            manager.set_selected_device(device_short.clone());
                            manager.get_all_game_packages().await
                        },
                        |r| match r {
                            Ok(o) => AdbMessage::AvailablePackages(o),
                            Err(e) => AdbMessage::Error(e.to_string()),
                        },
                    );
                } else {
                    return Task::perform(
                        async move {
                            let mut manager = AdbGameHandler::new();
                            manager.set_selected_device(device_short.clone());
                            manager.get_all_game_packages().await
                        },
                        |r| match r {
                            Ok(o) => AdbMessage::AvailablePackages(o),
                            Err(e) => AdbMessage::Error(e.to_string()),
                        },
                    );
                };
            }
            AdbMessage::AvailablePackages(items) => self.available_pkgs = items,
            AdbMessage::PkgClicked(pkg) => {
                if let Some(ref sel) = self.selected_device {
                    let pkg2 = pkg.clone();
                    if self.is_waydroid {
                        let mut manager = WaydroidGameHandler::new();
                        manager.set_selected_device(sel.clone());

                        return Task::perform(async move { manager.read_save(&pkg).await }, |r| {
                            match r {
                                Ok(o) => AdbMessage::LoadSave(o, pkg2),
                                Err(e) => AdbMessage::Error(e.to_string()),
                            }
                        });
                    } else {
                        let mut manager = AdbGameHandler::new();
                        manager.set_selected_device(sel.clone());

                        return Task::perform(async move { manager.read_save(&pkg).await }, |r| {
                            match r {
                                Ok(o) => AdbMessage::LoadSave(o, pkg2),
                                Err(e) => AdbMessage::Error(e.to_string()),
                            }
                        });
                    };
                }
            }
        };

        iced::Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum AdbMessage {
    AvailableDevices(Vec<DeviceShort>),
    FetchDevices,
    Error(String),
    SelectedDevice(DeviceShort),
    AvailablePackages(Vec<String>),
    PkgClicked(String),
    LoadSave(Vec<u8>, String),
}

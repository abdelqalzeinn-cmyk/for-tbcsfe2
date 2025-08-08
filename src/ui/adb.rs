use std::str::FromStr;

use adb_client::{DeviceShort, DeviceState};
use iced::{Element, Length, Task, alignment::Vertical, widget::container::bordered_box};

use crate::{
    adb::{adb_handler::AdbGameHandler, waydroid_handler::WaydroidGameHandler},
    ext_source::ExternalSaveSource,
    network::account_info::GameAccountInfo,
    save::SaveFile,
    ui::{
        helper::labeled_box,
        localization::{LocaleManager, Localizable},
    },
};

#[derive(Debug, Clone)]
pub struct AdbView {
    pub available_devices: Vec<DeviceShortEq>,
    pub selected_device: Option<DeviceShortEq>,
    pub available_pkgs: Vec<String>,
    pub selected_pkg: Option<String>,
    pub is_waydroid: bool,
    pub direction: AdbDirection,
}

#[derive(Debug, Clone)]
pub enum AdbDirection {
    LoadSave,
    SaveSave(Vec<u8>),
}

pub enum AdbCommand {
    Pull,
}

#[derive(Debug, Clone)]
struct DeviceShortEq {
    dev: DeviceShort,
}

impl PartialEq for DeviceShortEq {
    fn eq(&self, other: &Self) -> bool {
        if self.dev.identifier != other.dev.identifier {
            false
        } else {
            self.dev.state.to_string() == other.dev.state.to_string()
        }
    }
}

impl Eq for DeviceShortEq {}

impl AdbView {
    pub fn new(is_waydroid: bool, direction: AdbDirection) -> Self {
        Self {
            available_devices: Vec::new(),
            selected_device: None,
            available_pkgs: Vec::new(),
            selected_pkg: None,
            is_waydroid,
            direction,
        }
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
            let radio = iced::widget::radio(
                dev.dev.identifier.to_string(),
                dev,
                self.selected_device.as_ref(),
                |m| AdbMessage::SelectedDevice(m.clone()),
            );
            // let state_str = dev.state.to_string();
            // let button: Element<'_, AdbMessage> =
            //     iced::widget::button(iced::widget::text(&dev.identifier))
            //         .on_press(AdbMessage::SelectedDevice(dev.clone()))
            //         .into();
            // let row = iced::widget::row([
            //     button,
            //     iced::widget::text(state_str)
            //         .align_y(Vertical::Center)
            //         .height(Length::Fill)
            //         .into(),
            // ])
            // .height(Length::Shrink)
            // .spacing(10);
            // devcol.push(
            //     iced::widget::container(row)
            //         .padding(10)
            //         .style(bordered_box)
            //         .into(),
            // );
            devcol.push(radio.into());
        }
        let device_box = labeled_box(
            theme,
            "devices".localize(locale_manager),
            iced::widget::column(devcol).spacing(10).into(),
        );

        let mut cols = vec![device_box];

        if let Some(ref sel) = self.selected_device {
            // let sel_box = labeled_box(
            //     theme,
            //     "selected-device".localize(locale_manager),
            //     iced::widget::row([
            //         iced::widget::text(&sel.identifier).into(),
            //         iced::widget::text(sel.state.to_string()).into(),
            //     ])
            //     .spacing(10)
            //     .into(),
            // );
            // cols.push(sel_box);

            let label = match self.direction {
                AdbDirection::LoadSave => "pull-from",
                AdbDirection::SaveSave(_) => "push-to",
            };

            let btn_txt = match self.direction {
                AdbDirection::LoadSave => "pull",
                AdbDirection::SaveSave(_) => "push",
            }
            .localize(locale_manager);

            cols.push(self.view_pkg_list(theme, locale_manager, &label));

            cols.push(
                iced::widget::button(iced::widget::text(btn_txt))
                    .on_press_maybe(if let Some(ref sel) = self.selected_pkg {
                        Some(AdbMessage::PushOrPull(sel.to_string()))
                    } else {
                        None
                    })
                    .into(),
            );

            if let AdbDirection::SaveSave(_) = self.direction {
                cols.push(
                    iced::widget::button(iced::widget::text("rerun-game".localize(locale_manager)))
                        .on_press_maybe(if let Some(ref sel) = self.selected_pkg {
                            Some(AdbMessage::Rerun(sel.to_string()))
                        } else {
                            None
                        })
                        .into(),
                );
            }
        }

        let label = match self.direction {
            AdbDirection::LoadSave => "adb-pull",
            AdbDirection::SaveSave(_) => "adb-push",
        };

        labeled_box(
            theme,
            label.localize(locale_manager),
            iced::widget::column(cols).spacing(10).into(),
        )
    }

    fn view_pkg_list(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
        label: &str,
    ) -> Element<'_, AdbMessage> {
        let mut pkg_els = Vec::new();

        for pkg in &self.available_pkgs {
            pkg_els.push(
                iced::widget::radio(pkg, pkg, self.selected_pkg.as_ref(), |p| {
                    AdbMessage::SelectPkg(p.to_string())
                })
                .into(),
            );
        }

        labeled_box(
            theme,
            label.localize(locale_manager),
            iced::widget::column(pkg_els).spacing(10).into(),
        )
    }

    pub fn update(
        &mut self,
        message: AdbMessage,
        _locale_manager: &LocaleManager,
    ) -> iced::Task<AdbMessage> {
        match message {
            AdbMessage::AvailableDevices(device_shorts) => {
                self.available_devices = device_shorts;

                if let Some(first) = self.available_devices.first() {
                    return iced::Task::done(AdbMessage::SelectedDevice(first.clone()));
                }
            }
            AdbMessage::FetchDevices => {
                let mut manager = AdbGameHandler::new();
                let devices = manager.get_devices();

                match devices {
                    Ok(d) => {
                        return iced::Task::done(AdbMessage::AvailableDevices(
                            d.into_iter().map(|v| DeviceShortEq { dev: v }).collect(),
                        ));
                    }
                    Err(e) => return iced::Task::done(AdbMessage::Error(e.to_string())),
                }
            }
            AdbMessage::Error(_) => panic!("error must be handled further up!"),
            AdbMessage::LoadSave(..) => panic!("load save must be handled further up!"),
            AdbMessage::SelectedDevice(device_short) => {
                self.selected_device = Some(device_short.clone());

                if self.is_waydroid {
                    return Task::perform(
                        async move {
                            let mut manager = WaydroidGameHandler::new();
                            manager.set_selected_device(device_short.clone().dev);
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
                            manager.set_selected_device(device_short.clone().dev);
                            manager.get_all_game_packages().await
                        },
                        |r| match r {
                            Ok(o) => AdbMessage::AvailablePackages(o),
                            Err(e) => AdbMessage::Error(e.to_string()),
                        },
                    );
                };
            }
            AdbMessage::AvailablePackages(items) => {
                self.available_pkgs = items;

                if let Some(first) = self.available_pkgs.first() {
                    return Task::done(AdbMessage::SelectPkg(first.to_string()));
                }
            }
            AdbMessage::PushOrPull(pkg) => {
                if let Some(ref sel) = self.selected_device {
                    self.selected_pkg = Some(pkg.clone());
                    if self.is_waydroid {
                        let mut manager = WaydroidGameHandler::new();
                        manager.set_selected_device(sel.clone().dev);

                        return self.handle_adb_read_or_write(pkg, manager);
                    } else {
                        let mut manager = AdbGameHandler::new();
                        manager.set_selected_device(sel.clone().dev);

                        return self.handle_adb_read_or_write(pkg, manager);
                    };
                }
            }
            AdbMessage::PullAccountInfo(s, inquiry_code) => {
                let selected_device = self.selected_device.clone();
                let selected_pkg = self.selected_pkg.clone();
                let is_waydroid = self.is_waydroid;
                return Task::perform(
                    Self::pull_account_info(
                        inquiry_code,
                        selected_device.map(|v| v.dev),
                        selected_pkg,
                        is_waydroid,
                    ),
                    |r| match r {
                        Ok(o) => AdbMessage::PulledAccountInfo(s, o),
                        Err(e) => AdbMessage::Error(e),
                    },
                );
            }
            AdbMessage::PushAccountInfo(inquiry_code, info) => {
                let selected_device = self.selected_device.clone();
                let selected_pkg = self.selected_pkg.clone();
                let is_waydroid = self.is_waydroid;
                return Task::perform(
                    Self::push_account_info(
                        inquiry_code,
                        selected_device.map(|v| v.dev),
                        selected_pkg,
                        is_waydroid,
                        info,
                    ),
                    |r| match r {
                        Ok(_) => AdbMessage::PushedAccountInfo,
                        Err(e) => AdbMessage::Error(e),
                    },
                );
            }
            AdbMessage::PulledAccountInfo(..) => {
                panic!("pulled account info must be handled further up!")
            }
            AdbMessage::PushedAccountInfo => {
                panic!("pushed account info must be handled further up!")
            }
            AdbMessage::SaveSave(_) => panic!("save save must be handled further up!"),
            AdbMessage::Rerun(pkg) => {
                if let Some(ref sel) = self.selected_device {
                    self.selected_pkg = Some(pkg.clone());
                    if self.is_waydroid {
                        let mut manager = WaydroidGameHandler::new();
                        manager.set_selected_device(sel.clone().dev);

                        return adb_rerun_game(manager, pkg);
                    } else {
                        let mut manager = AdbGameHandler::new();
                        manager.set_selected_device(sel.clone().dev);

                        return adb_rerun_game(manager, pkg);
                    };
                };
            }
            AdbMessage::ReranGame(_) => panic!("reran game must be handled further up!"),
            AdbMessage::SelectPkg(p) => self.selected_pkg = Some(p),
        };

        iced::Task::none()
    }

    fn handle_adb_read_or_write<M: ExternalSaveSource>(
        &mut self,
        pkg: String,
        manager: M,
    ) -> Task<AdbMessage> {
        match self.direction {
            AdbDirection::LoadSave => return adb_read_save(manager, pkg),
            AdbDirection::SaveSave(ref data) => {
                return adb_save_save(manager, data.to_vec(), pkg);
            }
        }
    }

    pub async fn pull_account_info(
        inquiry_code: String,
        selected_device: Option<DeviceShort>,
        selected_pkg: Option<String>,
        is_waydroid: bool,
    ) -> Result<GameAccountInfo, String> {
        let selected_device = selected_device.ok_or("no device selected")?;
        let selected_pkg = selected_pkg.ok_or("no package selected")?;
        let account_data = match is_waydroid {
            true => {
                let mut manager = WaydroidGameHandler::new();
                manager.set_selected_device(selected_device);
                manager
                    .read_account_info(&selected_pkg, &inquiry_code)
                    .await
                    .map_err(|e| e.to_string())?
            }
            false => {
                let mut manager = AdbGameHandler::new();
                manager.set_selected_device(selected_device);
                manager
                    .read_account_info(&selected_pkg, &inquiry_code)
                    .await
                    .map_err(|e| e.to_string())?
            }
        };

        GameAccountInfo::from_data(&account_data).map_err(|e| e.to_string())
    }
    pub async fn push_account_info(
        inquiry_code: String,
        selected_device: Option<DeviceShort>,
        selected_pkg: Option<String>,
        is_waydroid: bool,
        info: GameAccountInfo,
    ) -> Result<(), String> {
        let selected_device = selected_device.ok_or("no device selected")?;
        let selected_pkg = selected_pkg.ok_or("no package selected")?;
        let data = info.to_data().map_err(|e| e.to_string())?;
        match is_waydroid {
            true => {
                let mut manager = WaydroidGameHandler::new();
                manager.set_selected_device(selected_device);
                manager
                    .write_account_info(&selected_pkg, &inquiry_code, data)
                    .await
                    .map_err(|e| e.to_string())?
            }
            false => {
                let mut manager = AdbGameHandler::new();
                manager.set_selected_device(selected_device);
                manager
                    .write_account_info(&selected_pkg, &inquiry_code, data)
                    .await
                    .map_err(|e| e.to_string())?
            }
        };

        Ok(())
    }

    pub fn init(&mut self) -> Task<AdbMessage> {
        return Task::done(AdbMessage::FetchDevices);
    }
}

fn adb_read_save<M: ExternalSaveSource>(mut manager: M, pkg: String) -> Task<AdbMessage> {
    let pkg2 = pkg.clone();
    return Task::perform(async move { manager.read_save(&pkg).await }, |r| match r {
        Ok(o) => AdbMessage::LoadSave(o, pkg2),
        Err(e) => AdbMessage::Error(e.to_string()),
    });
}

fn adb_save_save<M: ExternalSaveSource>(
    mut manager: M,
    data: Vec<u8>,
    pkg: String,
) -> Task<AdbMessage> {
    let pkg2 = pkg.clone();
    return Task::perform(
        async move { manager.write_save(data, &pkg).await },
        |r| match r {
            Ok(_) => AdbMessage::SaveSave(pkg2),
            Err(e) => AdbMessage::Error(e.to_string()),
        },
    );
}
fn adb_rerun_game<M: ExternalSaveSource>(mut manager: M, pkg: String) -> Task<AdbMessage> {
    let pkg2 = pkg.clone();
    return Task::perform(async move { manager.rerun_game(&pkg).await }, |r| match r {
        Ok(_) => AdbMessage::ReranGame(pkg2),
        Err(e) => AdbMessage::Error(e.to_string()),
    });
}

#[derive(Debug, Clone)]
pub enum AdbMessage {
    AvailableDevices(Vec<DeviceShortEq>),
    FetchDevices,
    Error(String),
    SelectedDevice(DeviceShortEq),
    AvailablePackages(Vec<String>),
    PushOrPull(String),
    LoadSave(Vec<u8>, String),
    PullAccountInfo(SaveFile, String),
    PushAccountInfo(String, GameAccountInfo),
    PulledAccountInfo(SaveFile, GameAccountInfo),
    SaveSave(String),
    PushedAccountInfo,
    Rerun(String),
    ReranGame(String),
    SelectPkg(String),
}

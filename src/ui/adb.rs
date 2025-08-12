use std::hash::Hash;

use adb_client::DeviceShort;
use iced::{Element, Task, alignment::Vertical, widget::container::bordered_box};

use crate::{
    adb::{
        adb_handler::{AdbGameHandler, find_adb_path, is_adb_installed},
        waydroid_handler::{WaydroidGameHandler, is_waydroid_installed},
    },
    ext_source::ExternalSaveSource,
    localization::{LocaleManager, Localizable},
    network::account_info::GameAccountInfo,
    save::SaveFile,
    ui::helper::labeled_box,
};

#[derive(Debug, Clone)]
pub struct AdbView {
    pub available_devices: Vec<DeviceShortEq>,
    pub waydroid_enabled: bool,
    pub selected_device: Option<DeviceShortEq>,
    pub available_pkgs: Vec<String>,
    pub selected_pkg: Option<String>,
    pub direction: AdbDirection,
    pub waydroid_installed: bool,
    pub adb_installed: Option<bool>,
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
pub struct DeviceShortEq {
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

impl Hash for DeviceShortEq {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.dev.identifier.as_bytes());
        state.write(self.dev.state.to_string().as_bytes());
    }
}

impl AdbView {
    pub fn new(direction: AdbDirection) -> Self {
        Self {
            available_devices: Vec::new(),
            selected_device: None,
            available_pkgs: Vec::new(),
            selected_pkg: None,
            direction,
            waydroid_enabled: false,
            waydroid_installed: false,
            adb_installed: None,
        }
    }
    pub fn view(
        &self,
        theme: &iced::Theme,
        locale_manager: &LocaleManager,
    ) -> Element<'_, AdbMessage> {
        let mut devcol = Vec::new();
        let refresh_btn =
            iced::widget::button(iced::widget::text("refresh".localize(locale_manager)))
                .on_press(AdbMessage::FetchDevices)
                .into();
        devcol.push(refresh_btn);
        for dev in &self.available_devices {
            let radio: Element<'_, AdbMessage> = iced::widget::radio(
                format!("{} - {}", dev.dev.identifier, dev.dev.state),
                dev,
                self.selected_device.as_ref(),
                |m| AdbMessage::SelectedDevice(m.clone()),
            )
            .into();
            devcol.push(
                iced::widget::container(radio)
                    .align_y(Vertical::Center)
                    .style(bordered_box)
                    .padding(10)
                    .into(),
            );
        }
        if self.available_devices.is_empty() {
            devcol.push(
                iced::widget::text("no-adb-devices".localize(locale_manager))
                    .color(theme.palette().danger)
                    .into(),
            )
        }
        if self.waydroid_installed {
            let switch: Element<'_, AdbMessage> = iced::widget::toggler(self.waydroid_enabled)
                .on_toggle(|t| AdbMessage::ToggleWaydroid(t))
                .label("use-waydroid".localize(locale_manager))
                .into();
            devcol.push(
                iced::widget::container(switch)
                    .style(bordered_box)
                    .padding(10)
                    .into(),
            );
        }
        let device_box = labeled_box(
            theme,
            "devices".localize(locale_manager),
            iced::widget::column(devcol).spacing(10).into(),
        );

        let mut cols = vec![device_box];

        if self.selected_device.is_some() {
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
                    .on_press_maybe(
                        if let Some(ref sel) = self.selected_pkg
                            && !self.available_pkgs.is_empty()
                        {
                            Some(AdbMessage::PushOrPull(sel.to_string()))
                        } else {
                            None
                        },
                    )
                    .into(),
            );

            if let AdbDirection::SaveSave(_) = self.direction {
                cols.push(
                    iced::widget::button(iced::widget::text("rerun-game".localize(locale_manager)))
                        .on_press_maybe(
                            if let Some(ref sel) = self.selected_pkg
                                && !self.available_pkgs.is_empty()
                            {
                                Some(AdbMessage::Rerun(sel.to_string()))
                            } else {
                                None
                            },
                        )
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

                if let Some(first) = self.available_devices.first()
                    && self.selected_device.is_none()
                {
                    return iced::Task::done(AdbMessage::SelectedDevice(first.clone()));
                }
            }
            AdbMessage::CheckWaydroid => {
                return iced::Task::perform(is_waydroid_installed(), AdbMessage::WaydroidInstalled);
            }
            AdbMessage::WaydroidInstalled(installed) => {
                self.waydroid_installed = installed;
            }
            AdbMessage::FetchDevices => {
                return iced::Task::perform(AdbGameHandler::get_devices(), |r| match r {
                    Ok(d) => AdbMessage::GotDevices(d),
                    Err(e) => AdbMessage::Error(e.to_string()),
                });
            }
            AdbMessage::GotDevices(devices) => {
                return iced::Task::done(AdbMessage::AvailableDevices(
                    devices
                        .into_iter()
                        .map(|v| DeviceShortEq { dev: v })
                        .collect(),
                ));
            }
            AdbMessage::Error(_) => panic!("error must be handled further up!"),
            AdbMessage::LoadSave(..) => panic!("load save must be handled further up!"),
            AdbMessage::SelectedDevice(device_short) => {
                self.selected_device = Some(device_short.clone());
                self.available_pkgs = Vec::new();

                if self.waydroid_enabled {
                    return Task::perform(
                        async move {
                            let mut manager = WaydroidGameHandler::new();
                            manager.set_selected_device(device_short.dev);
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
                            manager.set_selected_device(device_short.dev);
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

                if let Some(first) = self.available_pkgs.first()
                    && self.selected_pkg.is_none()
                {
                    return Task::done(AdbMessage::SelectPkg(first.to_string()));
                }
            }
            AdbMessage::PushOrPull(pkg) => {
                if let Some(ref sel) = self.selected_device {
                    self.selected_pkg = Some(pkg.clone());
                    if self.waydroid_enabled {
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
                let is_waydroid = self.waydroid_enabled;
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
                let is_waydroid = self.waydroid_enabled;
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
                    if self.waydroid_enabled {
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
            AdbMessage::ToggleWaydroid(toggle) => {
                self.waydroid_enabled = toggle;
                if let Some(ref sel) = self.selected_device {
                    return iced::Task::done(AdbMessage::SelectedDevice(sel.clone()));
                }
            }
            AdbMessage::CheckAdb => {
                // TODO: adb path config
                return Task::perform(is_adb_installed(find_adb_path()), AdbMessage::AdbInstalled);
            }
            AdbMessage::AdbInstalled(installed) => {
                self.adb_installed = Some(installed);
            }
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
        return Task::done(AdbMessage::CheckAdb)
            .chain(Task::done(AdbMessage::CheckWaydroid))
            .chain(Task::done(AdbMessage::FetchDevices));
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
    ToggleWaydroid(bool),
    GotDevices(Vec<DeviceShort>),
    WaydroidInstalled(bool),
    CheckWaydroid,
    CheckAdb,
    AdbInstalled(bool),
}

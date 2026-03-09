/* subscription.rs
 *
 * Copyright 2025-2026 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::{
    export::ExportStatus,
    ferrix::Ferrix,
    load_state::LoadState,
    messages::{DataReceiverMessage, KeyboardMessage, Message},
    pages::Page,
};
use iced::{Subscription, event, time};
use std::time::Duration;

type Script<T> = Subscription<T>;
type OScript<T> = Option<Script<T>>;

const START_UPERIOD: u64 = 10;

trait PushMaybe<T> {
    fn push_maybe(&mut self, data: Option<T>);
}

impl<T> PushMaybe<T> for Vec<T> {
    fn push_maybe(&mut self, data: Option<T>) {
        if let Some(data) = data {
            self.push(data);
        }
    }
}

impl Ferrix {
    pub fn subscription(&self) -> Script<Message> {
        if self.export_manager.status == ExportStatus::LoadingData {
            return self.export_data_subscription();
        }

        let charts_uperiod = self.settings.charts_update_period_nsecs as f32 * 0.1;
        let mut scripts = vec![
            event::listen().map(|event| Message::Keyboard(KeyboardMessage::Event(event))),
            // Charts
            time::every(Duration::from_secs_f32(charts_uperiod))
                .map(|_| Message::DataReceiver(DataReceiverMessage::AddCPUCoreLineSeries)),
            time::every(Duration::from_secs_f32(charts_uperiod))
                .map(|_| Message::DataReceiver(DataReceiverMessage::AddTotalRAMUsage)),
        ];
        let oscripts = [
            self.cpu_basic_data(),
            self.cpu_stat_data(),
            self.ram_data(),
            self.swap_data(),
            self.cpu_freq_subscription(),
            self.cpu_vuln_subscription(),
            self.storage_subscription(),
            self.networks_subscription(),
            self.dmi_subscription(),
            self.battery_subscription(),
            self.drm_subscription(),
            self.osrel_subscription(),
            self.users_subscription(),
            self.groups_subscription(),
            self.sysd_subscription(),
            self.soft_subscription(),
            self.env_and_sys_subscription(),
            self.kernel_subscription(),
            self.kmods_subscription(),
        ];
        for oscr in oscripts {
            scripts.push_maybe(oscr);
        }
        Subscription::batch(scripts)
    }

    fn export_data_subscription(&self) -> Script<Message> {
        let mut scripts =
            vec![event::listen().map(|event| Message::Keyboard(KeyboardMessage::Event(event)))];
        let oscripts = [
            self.cpu_basic_data(),
            self.cpu_stat_data(),
            self.ram_data(),
            self.swap_data(),
            self.cpu_freq_subscription(),
            self.cpu_vuln_subscription(),
            self.storage_subscription(),
            self.networks_subscription(),
            self.dmi_subscription(),
            self.battery_subscription(),
            self.drm_subscription(),
            self.osrel_subscription(),
            self.users_subscription(),
            self.groups_subscription(),
            self.sysd_subscription(),
            self.soft_subscription(),
            self.env_and_sys_subscription(),
            self.kernel_subscription(),
            self.kmods_subscription(),
        ];
        for oscr in oscripts {
            scripts.push_maybe(oscr);
        }
        Subscription::batch(scripts)
    }

    fn u(&self) -> u64 {
        self.settings.update_period as u64
    }

    pub fn is_export_member(&self, page: Page) -> bool {
        let e = self.export_manager.selected_pages;
        match page {
            Page::Processors => e.proc,
            Page::CPUFrequency => e.cpu_freq,
            Page::CPUVulnerabilities => e.cpu_vuln,
            Page::Memory => e.mem,
            Page::FileSystems => e.fs,
            Page::Network => e.net,
            Page::DMI => e.dmi,
            Page::Battery => e.bat,
            Page::Screen => e.screen,
            Page::Distro => e.distro,
            Page::Users => e.users,
            Page::Groups => e.groups,
            Page::SystemManager => e.sys_mgr,
            Page::Environment => e.env,
            Page::Software => e.soft,
            Page::SystemMisc => e.sysmisc,
            Page::Kernel => e.kernel,
            Page::KModules => e.kmods,
            _ => false,
        }
    }

    fn cpu_basic_data(&self) -> OScript<Message> {
        if ((self.current_page == Page::Dashboard || self.current_page == Page::Processors)
            && self.data.proc_data.is_none())
            || self.is_export_member(Page::Processors)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUData)),
            )
        } else {
            None
        }
    }

    fn cpu_stat_data(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard || self.current_page == Page::SystemMonitor)
            && self.data.curr_proc_stat.is_none()
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetProcStat)),
            )
        } else {
            let uperiod = self.settings.charts_update_period_nsecs as f32 * 0.1;
            Some(
                time::every(Duration::from_secs_f32(uperiod))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetProcStat)),
            )
        }
    }

    fn ram_data(&self) -> OScript<Message> {
        if ((self.current_page == Page::Dashboard || self.current_page == Page::Memory)
            && self.data.ram_data.is_none())
            || self.is_export_member(Page::Memory)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
            )
        } else {
            let uperiod = self.settings.charts_update_period_nsecs as f32 * 0.1;
            Some(
                time::every(Duration::from_secs_f32(uperiod))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetRAMData)),
            )
        }
    }

    fn swap_data(&self) -> OScript<Message> {
        if ((self.current_page == Page::Dashboard || self.current_page == Page::Memory)
            && self.data.swap_data.is_none())
            || self.is_export_member(Page::Memory)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSwapData)),
            )
        } else {
            let uperiod = self.settings.charts_update_period_nsecs as f32 * 0.1;
            Some(
                time::every(Duration::from_secs_f32(uperiod))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSwapData)),
            )
        }
    }

    fn cpu_freq_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::CPUFrequency && self.data.cpu_freq.is_none())
            || self.is_export_member(Page::CPUFrequency)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUFrequency)),
            )
        } else if self.current_page == Page::CPUFrequency {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUFrequency)),
            )
        } else {
            None
        }
    }

    fn cpu_vuln_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::CPUVulnerabilities
            && self.data.cpu_vulnerabilities.is_none())
            || self.is_export_member(Page::CPUVulnerabilities)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetCPUVulnerabilities)),
            )
        } else {
            None
        }
    }

    fn storage_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Dashboard
            || self.current_page == Page::FileSystems && self.data.storages.is_none())
            || self.is_export_member(Page::FileSystems)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetStorageData)),
            )
        } else if self.current_page == Page::FileSystems {
            Some(
                time::every(Duration::from_secs(10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetStorageData)),
            )
        } else {
            None
        }
    }

    fn networks_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Network && self.data.networks.is_none())
            || self.is_export_member(Page::Network)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetNetworksData)),
            )
        } else if self.current_page == Page::Network {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetNetworksData)),
            )
        } else {
            None
        }
    }

    fn dmi_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::DMI && !self.data.is_polkit && self.data.dmi_data.is_none())
            || self.is_export_member(Page::DMI)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDMIData)),
            )
        } else {
            None
        }
    }

    fn battery_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Dashboard
            || self.current_page == Page::Battery
            || self.is_export_member(Page::Battery)
        {
            if self.is_export_member(Page::Battery) {
                return Some(
                    time::every(Duration::from_secs(self.u()))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
                );
            }

            match self.data.bat_data {
                LoadState::Loaded(_) => Some(
                    time::every(Duration::from_secs(self.u()))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
                ),
                _ => Some(
                    time::every(Duration::from_millis(START_UPERIOD))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetBatInfo)),
                ),
            }
        } else {
            None
        }
    }

    fn drm_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Screen && self.data.drm_data.is_none())
            || self.is_export_member(Page::Screen)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDRMData)),
            )
        } else if self.current_page == Page::Screen && self.data.drm_data.is_some() {
            Some(
                time::every(Duration::from_secs(self.u()))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetDRMData)),
            )
        } else {
            None
        }
    }

    fn osrel_subscription(&self) -> OScript<Message> {
        if ((self.current_page == Page::Dashboard || self.current_page == Page::Distro)
            && self.data.osrel_data.is_none())
            || self.is_export_member(Page::Distro)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetOsReleaseData)),
            )
        } else {
            None
        }
    }

    fn users_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Users && self.data.users_list.is_none())
            || self.is_export_member(Page::Users)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetUsersData)),
            )
        } else {
            None
        }
    }

    fn groups_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Groups && self.data.groups_list.is_none())
            || self.is_export_member(Page::Groups)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetGroupsData)),
            )
        } else {
            None
        }
    }

    fn sysd_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::SystemManager && self.data.sysd_services_list.is_none())
            || self.is_export_member(Page::SystemManager)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemdServices)),
            )
        } else if self.current_page == Page::SystemManager && self.data.sysd_services_list.is_some()
        {
            Some(
                time::every(Duration::from_secs(self.u() * 10))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemdServices)),
            )
        } else {
            None
        }
    }

    fn soft_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Software && self.data.installed_pkgs_list.is_none())
            || self.is_export_member(Page::Software)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetPackagesList)),
            )
        } else {
            None
        }
    }

    fn env_and_sys_subscription(&self) -> OScript<Message> {
        if self.current_page == Page::Environment
            || self.current_page == Page::SystemMisc
            || self.current_page == Page::Dashboard
            || self.is_export_member(Page::Environment)
        {
            if self.data.system.is_none() || self.is_export_member(Page::Environment) {
                Some(
                    time::every(Duration::from_millis(START_UPERIOD))
                        .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
                )
            } else {
                match self.current_page {
                    Page::SystemMisc | Page::Dashboard => Some(
                        time::every(Duration::from_secs(self.u()))
                            .map(|_| Message::DataReceiver(DataReceiverMessage::GetSystemData)),
                    ),
                    _ => None,
                }
            }
        } else {
            None
        }
    }

    fn kernel_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::Kernel && self.data.kernel_data.is_none())
            || self.is_export_member(Page::Kernel)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetKernelData)),
            )
        } else {
            None
        }
    }

    fn kmods_subscription(&self) -> OScript<Message> {
        if (self.current_page == Page::KModules && self.data.kmods_data.is_none())
            || self.is_export_member(Page::KModules)
        {
            Some(
                time::every(Duration::from_millis(START_UPERIOD))
                    .map(|_| Message::DataReceiver(DataReceiverMessage::GetKModsData)),
            )
        } else {
            None
        }
    }
}

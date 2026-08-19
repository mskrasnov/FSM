/* pages.rs
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

//! Pages with information about hardware and software

use crate::{Message, ferrix::Ferrix, fl, icons::ERROR_ICON};
use ferrix_widgets::headers::header_text;
use iced::{
    Alignment::Center,
    Element, Task,
    widget::{center, column, container, row, svg::Handle, text},
};

mod about;
pub mod battery;
pub mod cpu;
pub mod cpu_freq;
mod dashboard;
pub mod distro;
mod dmi;
pub mod drm;
mod env;
mod export;
pub mod firmware;
pub mod groups;
mod kernel;
pub mod net;
mod ram;
pub mod session;
mod settings;
pub mod soft;
mod storage;
mod sysmon;
mod system;
mod systemd;
pub mod users;
pub mod vulnerabilities;

pub use sysmon::*;

pub mod page_impl;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Page {
    /************************************
     *       Hardware & dashboard       *
     ************************************/
    #[default]
    Dashboard,
    Processors,
    CPUFrequency,
    CPUVulnerabilities,
    SystemMonitor,
    Memory,
    FileSystems,
    Network,
    NetStat,
    DMI,
    Battery,
    Screen,

    /************************************
     *          Administration          *
     ************************************/
    Distro,
    SystemMisc,
    Session,
    Users,
    Groups,
    SystemManager,
    Software,
    Environment,
    Sensors,

    /************************************
     *               Kernel             *
     ************************************/
    Kernel,
    KModules,
    Firmware,
    Development,

    /************************************
     *              Service             *
     ************************************/
    Settings,
    About,
    Export,
    Todo,
}

impl From<&str> for Page {
    fn from(value: &str) -> Self {
        match value {
            "dash" | "dashboard" => Self::Dashboard,
            sysmon::SysmonPage::PAGE_ID | "monitor" | "system" | "system-monitor" => {
                Self::SystemMonitor
            }
            cpu::ProcPage::PAGE_ID | "proc" | "processors" => Self::Processors,
            cpu_freq::ProcFreqPage::PAGE_ID | "cpu-frequency" => Self::CPUFrequency,
            vulnerabilities::VulnPage::PAGE_ID | "cpu-vuln" | "vulnerabilities" => {
                Self::CPUVulnerabilities
            }
            "memory" | "mem" | "ram" => Self::Memory,
            "fs" | "storage" => Self::FileSystems,
            "net" => Self::Network,
            net::NetStatPage::PAGE_ID => Self::NetStat,
            "dmi" => Self::DMI,
            battery::BatPage::PAGE_ID | "battery" => Self::Battery,
            drm::DRMPage::PAGE_ID | "edid" | "screen" => Self::Screen,
            distro::OsRelPage::PAGE_ID | "distro" => Self::Distro,
            users::UsersPage::PAGE_ID | "users" => Self::Users,
            "groups" => Self::Groups,
            "misc" => Self::SystemMisc,
            session::SessionPage::PAGE_ID => Self::Session,
            "init" | "sysd" | "systemd" => Self::SystemManager,
            soft::SoftPage::PAGE_ID | "software" | "soft" | "pkgs" => Self::Software,
            "env" => Self::Environment,
            "sensors" => Self::Sensors,
            "kernel" | "linux" => Self::Kernel,
            "kmods" | "mod" | "modules" => Self::KModules,
            firmware::FirmwarePage::PAGE_ID | "firmware" | "frmwr" => Self::Firmware,
            "dev" => Self::Development,
            "settings" => Self::Settings,
            "about" | "version" | "--version" | "-V" | "-v" => {
                println!("FSM (Ferrix System Monitor) v{}", env!("CARGO_PKG_VERSION"));

                eprintln!(" *** If you are from Russia, you can send me a donation:");
                eprintln!("     2202 2062 5233 5406\n Thank you!");

                Self::About
            }
            "export" => Self::Export,
            _ => {
                eprintln!("ERROR: Unknown page name: \"{value}\"!\n");
                eprintln!(" *** If you are from Russia, you can send me a donation:");
                eprintln!("     2202 2062 5233 5406\n Thank you!");

                Self::default()
            }
        }
    }
}

impl From<usize> for Page {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Dashboard,
            1 => Self::SystemMonitor,
            2 => Self::Processors,
            3 => Self::CPUFrequency,
            4 => Self::CPUVulnerabilities,
            5 => Self::Memory,
            6 => Self::FileSystems,
            7 => Self::Network,
            8 => Self::NetStat,
            9 => Self::DMI,
            10 => Self::Battery,
            11 => Self::Screen,
            12 => Self::Sensors,
            13 => Self::Distro,
            14 => Self::Users,
            15 => Self::Groups,
            16 => Self::Environment,
            17 => Self::SystemManager,
            18 => Self::Software,
            19 => Self::Kernel,
            20 => Self::KModules,
            21 => Self::Firmware,
            22 => Self::SystemMisc,
            23 => Self::Settings,
            24 => Self::About,
            _ => Page::Dashboard,
        }
    }
}

impl<'a> Page {
    pub const ALL: &'a [Self] = &[
        Self::Dashboard,
        Self::SystemMonitor,
        Self::Processors,
        Self::CPUFrequency,
        Self::CPUVulnerabilities,
        Self::Memory,
        Self::FileSystems,
        Self::Network,
        Self::NetStat,
        Self::DMI,
        Self::Battery,
        Self::Screen,
        Self::Distro,
        Self::Users,
        Self::Groups,
        Self::Environment,
        Self::SystemManager,
        Self::Software,
        Self::Kernel,
        Self::KModules,
        Self::Firmware,
        Self::SystemMisc,
        Self::Export,
        Self::Settings,
        Self::About,
    ];

    pub fn is_special(&self) -> bool {
        *self == Self::Dashboard
            || *self == Self::SystemMonitor
            || *self == Self::Export
            || *self == Self::Export
            || *self == Self::Settings
            || *self == Self::About
    }

    pub fn non_special_pages(&self) -> Vec<Self> {
        let mut pages = Vec::with_capacity(Self::ALL.len());
        for page in Self::ALL {
            if !page.is_special() {
                pages.push(*page);
            }
        }
        pages.shrink_to_fit();
        pages
    }

    pub fn title(&'a self) -> iced::widget::Column<'a, Message> {
        header_text(self.title_str())
    }

    pub fn page_num(&self) -> usize {
        match self {
            Self::Dashboard => 0,
            Self::SystemMonitor => 1,
            Self::Processors => 2,
            Self::CPUFrequency => 3,
            Self::CPUVulnerabilities => 4,
            Self::Memory => 5,
            Self::FileSystems => 6,
            Self::Network => 7,
            Self::NetStat => 8,
            Self::DMI => 9,
            Self::Battery => 10,
            Self::Screen => 11,
            Self::Sensors => 12,
            Self::Distro => 13,
            Self::Users => 14,
            Self::Groups => 15,
            Self::Environment => 16,
            Self::SystemManager => 17,
            Self::Software => 18,
            Self::Kernel => 19,
            Self::KModules => 20,
            Self::Firmware => 21,
            Self::SystemMisc => 22,
            Self::Settings => 23,
            Self::About => 24,
            _ => 0,
        }
    }

    pub fn next_page(&self) -> Self {
        let mut id = self.page_num() + 1;
        if id > Self::About.page_num() {
            id = 0;
        }
        Self::from(id)
    }

    pub fn prev_page(&self) -> Self {
        let cur_id = self.page_num();
        let next_id = if cur_id == 0 {
            Self::About.page_num()
        } else {
            cur_id - 1
        };
        Self::from(next_id)
    }

    pub fn scrolled_list_id(&self) -> Option<&'static str> {
        match self {
            Self::Processors => Some("proc-list"),
            _ => None,
        }
    }

    pub fn page_id(&self) -> &'static str {
        match self {
            Self::Dashboard => "dash",
            Self::Processors => cpu::ProcPage::PAGE_ID,
            Self::CPUFrequency => cpu_freq::ProcFreqPage::PAGE_ID,
            Self::CPUVulnerabilities => vulnerabilities::VulnPage::PAGE_ID,
            Self::SystemMonitor => sysmon::SysmonPage::PAGE_ID,
            Self::Memory => "mem",
            Self::FileSystems => "fs",
            Self::Network => "net",
            Self::NetStat => net::NetStatPage::PAGE_ID,
            Self::DMI => "dmi",
            Self::Battery => battery::BatPage::PAGE_ID,
            Self::Screen => drm::DRMPage::PAGE_ID,
            Self::Distro => distro::OsRelPage::PAGE_ID,
            Self::SystemMisc => "sys",
            Self::Session => session::SessionPage::PAGE_ID,
            Self::Users => users::UsersPage::PAGE_ID,
            Self::Groups => "grp",
            Self::SystemManager => "sysd",
            Self::Software => soft::SoftPage::PAGE_ID,
            Self::Environment => "env",
            Self::Sensors => "hwmon",
            Self::Kernel => "krn",
            Self::KModules => "kmds",
            Self::Firmware => firmware::FirmwarePage::PAGE_ID,
            Self::Development => "dev",
            Self::Settings => "set",
            Self::About => "about",
            Self::Export => "export",
            Self::Todo => "todo",
        }
    }

    pub fn title_str(&self) -> String {
        match self {
            Self::Dashboard => fl!("page-dashboard"),
            Self::Processors => fl!("page-procs"),
            Self::CPUFrequency => fl!("page-cpufreq"),
            Self::CPUVulnerabilities => fl!("page-vuln"),
            Self::SystemMonitor => fl!("page-sysmon"),
            Self::Memory => fl!("page-memory"),
            Self::FileSystems => fl!("page-fsystems"),
            Self::Network => fl!("page-net"),
            Self::NetStat => fl!("page-nstat"),
            Self::DMI => fl!("page-dmi"),
            Self::Battery => fl!("page-battery"),
            Self::Screen => fl!("page-screen"),
            Self::Distro => fl!("page-distro"),
            Self::Users => fl!("page-users"),
            Self::Groups => fl!("page-groups"),
            Self::SystemManager => fl!("page-sysmgr"),
            Self::Software => fl!("page-software"),
            Self::Environment => fl!("page-env"),
            Self::Sensors => fl!("page-sensors"),
            Self::Kernel => fl!("page-kernel"),
            Self::KModules => fl!("page-kmods"),
            Self::Firmware => fl!("page-frmwr"),
            Self::Development => fl!("page-dev"),
            Self::SystemMisc => fl!("page-sysmisc"),
            Self::Session => "Session".to_string(),
            Self::Settings => fl!("page-settings"),
            Self::About => fl!("page-about"),
            Self::Export => fl!("page-export"),
            Self::Todo => fl!("page-todo"),
        }
    }

    pub fn get_data_single(&'a self) -> Option<Task<Message>> {
        match self {
            Self::Processors | Self::Dashboard => {
                Some(cpu::ProcPage::get_data().map(Message::DataReceiver))
            }
            _ => None,
        }
    }

    pub fn page(&'a self, state: &'a Ferrix) -> Element<'a, Message> {
        let page = match self {
            Self::Dashboard => dashboard::dashboard(&state.data).into(),
            Self::SystemMonitor => {
                let sysmon = sysmon::SysmonPage::new(
                    &state.data.curr_proc_stat,
                    &state.data.prev_proc_stat,
                    &state.state,
                );
                sysmon.view()
            }
            Self::Processors => {
                let cpu_page = cpu::ProcPage::new(&state.data.proc_data, state.state.selected_proc);
                cpu_page.view()
            }
            Self::CPUFrequency => {
                let freq_page =
                    cpu_freq::ProcFreqPage::new(&state.data.cpu_freq, state.state.selected_freq);
                freq_page.view()
            }
            Self::CPUVulnerabilities => {
                let vuln_page = vulnerabilities::VulnPage::new(&state.data.cpu_vulnerabilities);
                vuln_page.view()
            }
            Self::Memory => ram::ram_page(&state.data.ram_data, &state.data.swap_data).into(),
            Self::FileSystems => storage::storage_page(&state.data.storages).into(),
            Self::Network => net::net_page(&state.data.networks).into(),
            Self::NetStat => {
                let nstat = net::NetStatPage::new(&state.data.networks);
                nstat.view()
            }
            Self::DMI => dmi::dmi_page(&state.data.dmi_data).into(),
            Self::Battery => {
                let battery = battery::BatPage::new(&state.data.bat_data);
                battery.view()
            }
            Self::Screen => {
                let screen = drm::DRMPage::new(&state.data.drm_data, state.state.selected_screen);
                screen.view()
            }
            Self::Distro => {
                let distro = distro::OsRelPage::new(&state.data.osrel_data);
                distro.view()
            }
            Self::Session => {
                let session = session::SessionPage::new(&state.data.session);
                session.view()
            }
            Self::Kernel => kernel::kernel_page(&state.data.kernel_data).into(),
            Self::KModules => kernel::kmods_page(&state.data.kmods_data).into(),
            Self::Firmware => {
                let firmware = firmware::FirmwarePage::new(&state.data.firmware_data);
                firmware.view()
            }
            Self::SystemMisc => system::system_page(&state.data.system).into(),
            Self::Users => {
                let usr = users::UsersPage::new(&state.data.users_list);
                usr.view()
            }
            Self::Groups => {
                let grp = groups::GroupsPage::new(&state.data.groups_list);
                grp.view()
            }
            Self::SystemManager => {
                systemd::services_page(&state.data.boot_time, &state.data.sysd_services_list).into()
            }
            Self::Software => {
                let soft = soft::SoftPage::new(&state.data.installed_pkgs_list);
                soft.view()
            }
            Self::Environment => env::env_page(&state.data.system).into(),
            Self::Settings => settings::settings_page(&state).into(),
            Self::Export => export::export_page(&state.export_manager).into(),
            Self::About => about::about_page(),
            _ => self.todo_page(),
        };

        column![self.title(), page,].spacing(5).into()
    }

    fn todo_page(&self) -> Element<'a, Message> {
        container(center(
            text(fl!("page-todo-msg")).size(16).style(text::secondary),
        ))
        .into()
    }
}

fn loading_page<'a>() -> container::Container<'a, Message> {
    container(center(
        text(fl!("ldr-page-tooltip"))
            .style(text::secondary)
            .size(14),
    ))
}

fn error_page<'a>(etext: &'a str) -> container::Container<'a, Message> {
    container(center(
        column![
            row![
                iced::widget::svg(Handle::from_memory(ERROR_ICON))
                    .width(20)
                    .height(20),
                text(fl!("err-page-tooltip")).size(20),
            ]
            .align_y(Center)
            .spacing(5),
            text(etext).style(text::secondary).size(14),
        ]
        .align_x(Center)
        .spacing(5),
    ))
}

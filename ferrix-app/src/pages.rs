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

use iced::{
    Alignment::{self, Center},
    Element,
    widget::{center, column, container, row, rule, svg::Handle, text},
};

use crate::{
    Message,
    ferrix::Ferrix,
    fl,
    icons::ERROR_ICON,
    widgets::{header_text, link_button},
};

mod battery;
mod cpu;
mod cpu_freq;
mod dashboard;
mod distro;
mod dmi;
mod drm;
mod env;
mod export;
mod groups;
mod kernel;
mod net;
mod ram;
mod settings;
mod soft;
mod storage;
mod sysmon;
mod system;
mod systemd;
mod users;
mod vulnerabilities;

pub use sysmon::*;

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
    DMI,
    Battery,
    Screen,

    /************************************
     *          Administration          *
     ************************************/
    Distro,
    SystemMisc,
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
            "sysmon" | "monitor" | "system" | "system-monitor" => Self::SystemMonitor,
            "proc" | "cpu" | "processors" => Self::Processors,
            "cpu-frequency" | "cpufreq" => Self::CPUFrequency,
            "cpu-vuln" | "vulnerabilities" => Self::CPUVulnerabilities,
            "memory" | "mem" | "ram" => Self::Memory,
            "fs" | "storage" => Self::FileSystems,
            "net" => Self::Network,
            "dmi" => Self::DMI,
            "battery" | "bat" => Self::Battery,
            "edid" | "screen" => Self::Screen,
            "distro" => Self::Distro,
            "users" => Self::Users,
            "groups" => Self::Groups,
            "misc" => Self::SystemMisc,
            "init" | "sysd" | "systemd" => Self::SystemManager,
            "software" | "soft" | "pkg" | "pkgs" => Self::Software,
            "env" => Self::Environment,
            "sensors" => Self::Sensors,
            "kernel" | "linux" => Self::Kernel,
            "kmods" | "mod" | "modules" => Self::KModules,
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
            8 => Self::DMI,
            9 => Self::Battery,
            10 => Self::Screen,
            11 => Self::Sensors,
            12 => Self::Distro,
            13 => Self::Users,
            14 => Self::Groups,
            15 => Self::Environment,
            16 => Self::SystemManager,
            17 => Self::Software,
            18 => Self::Kernel,
            19 => Self::KModules,
            20 => Self::SystemMisc,
            21 => Self::Settings,
            22 => Self::About,
            _ => Page::Dashboard,
        }
    }
}

impl<'a> Page {
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
            Self::DMI => 8,
            Self::Battery => 9,
            Self::Screen => 10,
            Self::Sensors => 11,
            Self::Distro => 12,
            Self::Users => 13,
            Self::Groups => 14,
            Self::Environment => 15,
            Self::SystemManager => 16,
            Self::Software => 17,
            Self::Kernel => 18,
            Self::KModules => 19,
            Self::SystemMisc => 20,
            Self::Settings => 21,
            Self::About => 22,
            _ => 0,
        }
    }

    pub fn next_page(&self) -> Self {
        let mut id = self.page_num() + 1;
        if id > 21 {
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
            Self::Processors => "proc",
            Self::CPUFrequency => "cpufreq",
            Self::CPUVulnerabilities => "cpuvuln",
            Self::SystemMonitor => "sysmon",
            Self::Memory => "mem",
            Self::FileSystems => "fs",
            Self::Network => "net",
            Self::DMI => "dmi",
            Self::Battery => "bat",
            Self::Screen => "drm",
            Self::Distro => "distro",
            Self::SystemMisc => "sys",
            Self::Users => "usr",
            Self::Groups => "grp",
            Self::SystemManager => "sysd",
            Self::Software => "pkg",
            Self::Environment => "env",
            Self::Sensors => "hwmon",
            Self::Kernel => "krn",
            Self::KModules => "kmds",
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
            Self::Development => fl!("page-dev"),
            Self::SystemMisc => fl!("page-sysmisc"),
            Self::Settings => fl!("page-settings"),
            Self::About => fl!("page-about"),
            Self::Export => fl!("page-export"),
            Self::Todo => fl!("page-todo"),
        }
    }

    pub fn page(&'a self, state: &'a Ferrix) -> Element<'a, Message> {
        let page = match self {
            Self::Dashboard => dashboard::dashboard(&state.data).into(),
            Self::SystemMonitor => sysmon::usage_charts_page(
                &state.data,
                &state.data.curr_proc_stat,
                &state.data.prev_proc_stat,
            )
            .into(), // TODO: cur_stat and proc_stat - ???
            Self::Processors => {
                cpu::proc_page(&state.data.proc_data, state.data.selected_proc).into()
            }
            Self::CPUFrequency => cpu_freq::cpu_freq_page(&state.data.cpu_freq).into(),
            Self::CPUVulnerabilities => {
                vulnerabilities::vulnerabilities_page(&state.data.cpu_vulnerabilities).into()
            }
            Self::Memory => ram::ram_page(&state.data.ram_data, &state.data.swap_data).into(),
            Self::FileSystems => storage::storage_page(&state.data.storages).into(),
            Self::Network => net::net_page(&state.data.networks).into(),
            Self::DMI => dmi::dmi_page(&state.data.dmi_data).into(),
            Self::Battery => battery::bat_page(&state.data.bat_data).into(),
            Self::Screen => drm::drm_page(&state.data.drm_data).into(),
            Self::Distro => distro::distro_page(&state.data.osrel_data).into(),
            Self::Kernel => kernel::kernel_page(&state.data.kernel_data).into(),
            Self::KModules => kernel::kmods_page(&state.data.kmods_data).into(),
            Self::SystemMisc => system::system_page(&state.data.system).into(),
            Self::Users => users::users_page(&state.data.users_list).into(),
            Self::Groups => groups::groups_page(&state.data.groups_list).into(),
            Self::SystemManager => systemd::services_page(
                &state.data.boot_time,
                &state.data.sysd_services_list,
            )
            .into(),
            Self::Software => soft::soft_page(&state.data.installed_pkgs_list).into(),
            Self::Environment => env::env_page(&state.data.system).into(),
            Self::Settings => settings::settings_page(&state).into(),
            Self::Export => export::export_page().into(),
            Self::About => self.about_page().into(),
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

    fn about_page(&'a self) -> container::Container<'a, Message> {
        let img = iced::widget::svg("/usr/share/Ferrix/com.mskrasnov.Ferrix.svg")
            .width(128)
            .height(128);
        let header = row![
            img,
            column![
                text(fl!("about-hdr")).size(24),
                text(format!(
                    "{}: {}, {}: {}",
                    fl!("about-ferrix"),
                    env!("CARGO_PKG_VERSION"),
                    fl!("about-flib"),
                    ferrix_lib::FX_LIB_VERSION,
                ))
                .size(14),
            ]
            .spacing(5),
        ]
        .align_y(Center)
        .spacing(5);

        let about_info = row![
            column![
                text(fl!("about-author-hdr")).style(text::secondary),
                text(fl!("about-feedback-hdr")).style(text::secondary),
                text(fl!("about-source-hdr")).style(text::secondary),
                text("crates.io:").style(text::secondary),
                text(fl!("about-blog")).style(text::secondary),
            ]
            .align_x(Alignment::End)
            .spacing(3),
            column![
                row![
                    text(fl!("about-author")),
                    link_button("(GitHub)", "https://github.com/mskrasnov"),
                ]
                .spacing(5),
                link_button("mskrasnov07 at ya dot ru", "mailto:mskrasnov07@ya.ru"),
                link_button("GitHub", "https://github.com/mskrasnov/Ferrix"),
                row![
                    link_button("ferrix-app", "https://crates.io/crates/ferrix-app"),
                    text(", "),
                    link_button("ferrix-lib", "https://crates.io/crates/ferrix-lib"),
                ],
                link_button("mskrasnov", "https://boosty.to/mskrasnov"),
            ]
            .spacing(3),
        ]
        .spacing(5);

        let donate = column![
            text(fl!("about-donate")),
            link_button(fl!("about-donate-lbl"), "https://boosty.to/mskrasnov"),
        ]
        .spacing(5);

        let contents = column![
            column![header, rule::horizontal(1)].spacing(2),
            about_info,
            row![
                text(fl!("about-support")).style(text::warning).size(16),
                rule::horizontal(1)
            ]
            .align_y(Center)
            .spacing(5),
            donate,
        ]
        .spacing(5);

        container(contents)
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

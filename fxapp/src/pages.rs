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

pub mod error_page;
pub mod loading_page;
pub mod todo_page;

pub mod battery;
pub mod mem;
pub mod proc;

pub use loading_page::loading_page;
pub use todo_page::todo;

use crate::{
    fl,
    message::{DataReceiver, Message},
};
use iced::{
    Alignment::Center,
    Element, Task,
    widget::{Id, column, row, rule, space, text},
};

pub trait PageView<'a> {
    fn page_id() -> &'static str;

    fn scrolled_page_id() -> Option<&'static str> {
        None
    }

    fn page_title() -> String;
    fn page_group() -> GroupVariant;

    fn page_title_controls(&'a self) -> Option<Element<'a, Message>> {
        None
    }

    fn page_title_view(&'a self) -> Element<'a, Message> {
        column![
            row![
                text(Self::page_title()).size(20),
                space::horizontal(),
                self.page_title_controls().unwrap_or(row![].into()),
            ]
            .align_y(Center)
            .spacing(5),
            rule::horizontal(1),
        ]
        .into()
    }

    fn page_contents_view(&'a self) -> Element<'a, Message>;

    fn view(&'a self) -> Element<'a, Message> {
        column![self.page_title_view(), self.page_contents_view()]
            .spacing(5)
            .into()
    }
}

pub trait PageData {
    fn get_data() -> Task<DataReceiver>;
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum PageVariant {
    #[default]
    SystemPassport,
    SystemMonitor,
    Processors,
    CPUFrequencies,
    CPUVulnerabilities,
    Memory,
    FileSystems,
    NetworkInterfaces,
    NetworkStatistics,
    DMITables,
    Battery,
    Screens,
    Sensors,
    Distro,
    Session,
    Users,
    Groups,
    Environment,
    SystemManager,
    Software,
    Kernel,
    KernelModules,
    FirmwareAttributes,
    SystemMisc,
    ExportData,
    ProgramSettings,
    ProgramAbout,
    Todo,
}

impl PageVariant {
    pub const ALL: &'static [Self] = &[
        // General
        Self::SystemPassport,
        Self::SystemMonitor,
        // Hardware
        Self::Processors,
        Self::CPUFrequencies,
        Self::CPUVulnerabilities,
        Self::FileSystems,
        Self::DMITables,
        Self::Battery,
        Self::Screens,
        Self::Sensors,
        // Network
        Self::NetworkInterfaces,
        Self::NetworkStatistics,
        // Admin
        Self::Distro,
        Self::Session,
        Self::Users,
        Self::Groups,
        Self::Environment,
        Self::SystemManager,
        Self::Software,
        // System
        Self::Memory,
        Self::Kernel,
        Self::KernelModules,
        Self::FirmwareAttributes,
        Self::SystemMisc,
    ];

    pub fn group(&self) -> GroupVariant {
        match self {
            Self::SystemPassport => GroupVariant::General,
            Self::SystemMonitor => GroupVariant::General,
            Self::Processors => proc::ProcPage::page_group(),
            Self::CPUFrequencies => GroupVariant::Hardware,
            Self::CPUVulnerabilities => GroupVariant::Hardware,
            Self::Memory => mem::MemoryPage::page_group(),
            Self::FileSystems => GroupVariant::Hardware,
            Self::DMITables => GroupVariant::Hardware,
            Self::Battery => battery::BatPage::page_group(),
            Self::Screens => GroupVariant::Hardware,
            Self::Sensors => GroupVariant::Hardware,
            Self::NetworkInterfaces => GroupVariant::Network,
            Self::NetworkStatistics => GroupVariant::Network,
            Self::Distro => GroupVariant::Admin,
            Self::Session => GroupVariant::Admin,
            Self::Users => GroupVariant::Admin,
            Self::Groups => GroupVariant::Admin,
            Self::Environment => GroupVariant::Admin,
            Self::SystemManager => GroupVariant::Admin,
            Self::Software => GroupVariant::Admin,
            Self::Kernel => GroupVariant::System,
            Self::KernelModules => GroupVariant::System,
            Self::FirmwareAttributes => GroupVariant::System,
            Self::SystemMisc => GroupVariant::System,
            _ => GroupVariant::Other,
        }
    }

    pub fn id(&self) -> Id {
        Id::new(match self {
            Self::Processors => proc::ProcPage::page_id(),
            Self::Memory => mem::MemoryPage::page_id(),
            Self::Battery => battery::BatPage::page_id(),
            _ => "",
        })
    }

    pub fn scrolled_id(&self) -> Option<Id> {
        match self {
            Self::Processors => proc::ProcPage::scrolled_page_id().and_then(|id| Some(Id::new(id))),
            _ => None,
        }
    }

    pub fn title(&self) -> String {
        match self {
            Self::Processors => proc::ProcPage::page_title(),
            Self::Memory => mem::MemoryPage::page_title(),
            Self::Battery => battery::BatPage::page_title(),
            _ => format!("[???] {:?}", self),
        }
    }

    pub fn view<'a>(&'a self, fx: &'a crate::Ferrix) -> Element<'a, Message> {
        match self {
            Self::Processors => fx.proc_page.view(),
            Self::Memory => fx.mem_page.view(),
            Self::Battery => fx.bat_page.view(),
            _ => todo_page::todo(),
        }
    }

    fn page_idx(&self) -> usize {
        Self::ALL.iter().position(|p| p == self).unwrap()
    }

    pub fn next_page(&self) -> Self {
        Self::ALL[(self.page_idx() + 1) % Self::ALL.len()]
    }

    pub fn prev_page(&self) -> Self {
        Self::ALL[(self.page_idx() + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GroupVariant {
    General,
    Hardware,
    Network,
    Admin,
    System,
    Other,
}

impl GroupVariant {
    pub const ALL: &'static [Self] = &[
        Self::General,
        Self::Hardware,
        Self::Network,
        Self::Admin,
        Self::System,
    ];

    pub fn title(&self) -> String {
        match self {
            Self::General => fl!("sidebar-basic"),
            Self::Hardware => fl!("sidebar-hardware"),
            Self::Network => fl!("sidebar-network"),
            Self::Admin => fl!("sidebar-admin"),
            Self::System => fl!("sidebar-system"),
            Self::Other => fl!("sidebar-manage"),
        }
        .to_string()
    }
}

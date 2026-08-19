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

use iced::{Element, Task};

use crate::messages::{DataReceiverMessage, Message};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Variant {
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

impl Variant {
    pub const ALL: &'static [Self] = &[
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
        Self::Settings,
        Self::About,
    ];

    pub fn next(&self) -> Self {
        let pos = Self::ALL.iter().position(|p| p == self).unwrap();
        Self::ALL[(pos + 1) % Self::ALL.len()].clone()
    }

    pub fn prev(&self) -> Self {
        let pos = Self::ALL.iter().position(|p| p == self).unwrap();
        Self::ALL[(pos + Self::ALL.len() - 1) % Self::ALL.len()].clone()
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Category {
    #[default]
    General,
    Hardware,
    Administration,
    System,
}

pub trait View<'a> {
    fn page_id() -> &'static str;
    fn page_title() -> String;
    fn page_title_controls(&'a self) -> Option<Element<'a, Message>>;
    fn page_title_view(&'a self) -> Element<'a, Message>;
    fn page_contents_view(&'a self) -> Element<'a, Message>;
}

pub trait DataProvider<'a> {
    fn get_data() -> Task<DataReceiverMessage>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SidebarItem {
    Category(Category),
    Page(Variant),
}

impl SidebarItem {
    pub const ALL: &'static [Self] = &[
        Self::Category(Category::General),
        Self::Page(Variant::Dashboard),
        Self::Page(Variant::SystemMonitor),
        Self::Category(Category::Hardware),
        Self::Page(Variant::Processors),
        Self::Page(Variant::CPUFrequency),
        Self::Page(Variant::CPUVulnerabilities),
        Self::Page(Variant::Memory),
        Self::Page(Variant::FileSystems),
        Self::Page(Variant::Network),
        Self::Page(Variant::NetStat),
        Self::Page(Variant::DMI),
        Self::Page(Variant::Battery),
        Self::Page(Variant::Screen),
        Self::Page(Variant::Sensors),
        Self::Category(Category::Administration),
        Self::Page(Variant::Distro),
        Self::Page(Variant::Users),
        Self::Page(Variant::Groups),
        Self::Page(Variant::Environment),
        Self::Page(Variant::SystemManager),
        Self::Page(Variant::Software),
        Self::Category(Category::System),
        Self::Page(Variant::Kernel),
        Self::Page(Variant::KModules),
        Self::Page(Variant::Firmware),
        Self::Page(Variant::SystemMisc),
    ];
}

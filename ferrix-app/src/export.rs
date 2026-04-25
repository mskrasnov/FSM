/* export.rs
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

//! Export manager

use crate::fl;
use ferrix_data::{FerrixData, System, dmi::DMIData, load_state::LoadState};
use ferrix_lib::{
    battery::BatInfo,
    cpu::Processors,
    cpu_freq::CpuFreq,
    drm::Video,
    init::{BootTimestamps, SystemdServices},
    net::Networks,
    parts::Mounts,
    ram::{RAM, Swaps},
    soft::InstalledPackages,
    sys::{Groups, KModules, Kernel, OsRelease, Users},
    traits::ToJson,
    vulnerabilities::Vulnerabilities,
};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum ExportStatus {
    #[default]
    Pending,
    LoadingData,
    ErrorLoadingData(String),
    SerializingStructure,
    ErrorSerializing(String),
    WritingData,
    ErrorWritingData(String),
    Complete,
}

impl Display for ExportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "{}", fl!("export-st-pending")),
            Self::LoadingData => write!(f, "{}", fl!("export-st-load")),
            Self::ErrorLoadingData(err) => write!(f, "{}", fl!("export-st-lerr", err = err)),
            Self::SerializingStructure => write!(f, "{}", fl!("export-st-ser")),
            Self::ErrorSerializing(err) => write!(f, "{}", fl!("export-st-serr", err = err)),
            Self::WritingData => write!(f, "{}", fl!("export-st-wr")),
            Self::ErrorWritingData(err) => write!(f, "{}", fl!("export-st-werr", err = err)),
            Self::Complete => write!(f, "OK"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum ExportFormat {
    #[default]
    CompressedJson,
    HumanJson,
    XML,
    PlainText,
}

impl ExportFormat {
    pub const ALL: &[Self] = &[
        Self::CompressedJson,
        Self::HumanJson,
        Self::XML,
        Self::PlainText,
    ];
}

impl Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CompressedJson => "Compressed JSON",
                Self::HumanJson => "Human-readable JSON",
                Self::XML => "XML",
                Self::PlainText => "Plain Text (*.txt)",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum ExportMode {
    AllData,
    #[default]
    Selected,
}

impl ExportMode {
    pub const ALL: &[Self] = &[Self::AllData, Self::Selected];
}

impl Display for ExportMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AllData => "All collected data",
                Self::Selected => "Selected data",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExportPages {
    pub proc: bool,
    pub cpu_freq: bool,
    pub cpu_vuln: bool,
    pub mem: bool,
    pub fs: bool,
    pub net: bool,
    pub dmi: bool,
    pub bat: bool,
    pub screen: bool,
    pub distro: bool,
    pub users: bool,
    pub groups: bool,
    pub env: bool,
    pub sys_mgr: bool,
    pub soft: bool,
    pub kernel: bool,
    pub kmods: bool,
    pub sysmisc: bool,
}

impl Default for ExportPages {
    fn default() -> Self {
        Self {
            proc: false,
            cpu_freq: false,
            cpu_vuln: false,
            mem: false,
            fs: false,
            net: false,
            dmi: false,
            bat: false,
            screen: false,
            distro: false,
            users: false,
            groups: false,
            env: false,
            sys_mgr: false,
            soft: false,
            kernel: false,
            kmods: false,
            sysmisc: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(untagged)]
pub enum ExportMember<T> {
    Data(Option<T>),
    Error(String),
    #[default]
    None,
}

impl<T> From<LoadState<T>> for ExportMember<T> {
    fn from(value: LoadState<T>) -> Self {
        match value {
            LoadState::Loaded(data) => Self::Data(Some(data)),
            LoadState::Loading => Self::Data(None),
            LoadState::Error(why) => Self::Error(why.clone()),
        }
    }
}

trait ToExportMember<T> {
    fn to_export_memb(self) -> ExportMember<T>;
}

impl<T> ToExportMember<T> for LoadState<T> {
    fn to_export_memb(self) -> ExportMember<T> {
        match self {
            Self::Loading => ExportMember::Data(None),
            Self::Loaded(data) => ExportMember::Data(Some(data)),
            Self::Error(why) => ExportMember::Error(why),
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ExportData {
    pub proc_data: ExportMember<Processors>,
    pub cpu_freq: ExportMember<CpuFreq>,
    pub cpu_vulnerabilities: ExportMember<Vulnerabilities>,

    pub ram_data: ExportMember<RAM>,
    pub swap_data: ExportMember<Swaps>,

    pub storages: ExportMember<Mounts>,
    pub networks: ExportMember<Networks>,
    pub dmi_data: ExportMember<DMIData>,
    pub bat_data: ExportMember<BatInfo>,
    pub drm_data: ExportMember<Video>,
    pub osrel_data: ExportMember<OsRelease>,

    pub kernel_data: ExportMember<Kernel>,
    pub kmods_data: ExportMember<KModules>,

    pub users_list: ExportMember<Users>,
    pub groups_list: ExportMember<Groups>,
    pub sysd_services_list: ExportMember<SystemdServices>,
    pub boot_time: ExportMember<BootTimestamps>,
    pub installed_pkgs_list: ExportMember<InstalledPackages>,
    pub system: ExportMember<System>,
}

impl ToJson for ExportData {}

// NOTE: SHITCODE!!!
impl<'a> From<&'a FerrixData> for ExportData {
    fn from(fx: &'a FerrixData) -> Self {
        Self {
            proc_data: fx.proc_data.clone().to_export_memb(),
            cpu_freq: fx.cpu_freq.clone().to_export_memb(),
            cpu_vulnerabilities: fx.cpu_vulnerabilities.clone().to_export_memb(),
            ram_data: fx.ram_data.clone().to_export_memb(),
            swap_data: fx.swap_data.clone().to_export_memb(),
            storages: fx.storages.clone().to_export_memb(),
            networks: fx.networks.clone().to_export_memb(),
            dmi_data: fx.dmi_data.clone().to_export_memb(),
            bat_data: fx.bat_data.clone().to_export_memb(),
            drm_data: fx.drm_data.clone().to_export_memb(),
            osrel_data: fx.osrel_data.clone().to_export_memb(),
            kernel_data: fx.kernel_data.clone().to_export_memb(),
            kmods_data: fx.kmods_data.clone().to_export_memb(),
            users_list: fx.users_list.clone().to_export_memb(),
            groups_list: fx.groups_list.clone().to_export_memb(),
            sysd_services_list: fx.sysd_services_list.clone().to_export_memb(),
            boot_time: fx.boot_time.clone().to_export_memb(),
            installed_pkgs_list: fx.installed_pkgs_list.clone().to_export_memb(),
            system: fx.system.clone().to_export_memb(),
        }
    }
}

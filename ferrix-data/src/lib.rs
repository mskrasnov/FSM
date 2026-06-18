/* lib.rs
 *
 * Copyright 2026 Michail Krasnov <mskrasnov07@ya.ru>
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

//! Data from `ferrix-lib`

pub mod dmi;
pub mod firmware;
pub mod kmods;
pub mod load_state;
pub mod polkit;

use anyhow::Result;
use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    cpu_freq::CpuFreq,
    drm::Video,
    init::{BootTimestamps, SystemdServices},
    net::Networks,
    parts::Mounts,
    ram::{RAM, Swaps},
    soft::InstalledPackages,
    sys::{
        Groups, Kernel, LoadAVG, OsRelease, Shell, Uptime, Users, current_user,
        get_current_desktop, get_env_vars, get_hostname, get_lang,
    },
    vulnerabilities::Vulnerabilities,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{dmi::DMIData, firmware::FResult, kmods::KResult, load_state::LoadState};

#[derive(Debug, Serialize)]
pub struct FerrixData {
    pub proc_data: LoadState<Processors>,
    pub prev_proc_stat: LoadState<Stat>,
    pub curr_proc_stat: LoadState<Stat>,
    pub cpu_freq: LoadState<CpuFreq>,
    pub cpu_vulnerabilities: LoadState<Vulnerabilities>,

    pub ram_data: LoadState<RAM>,
    pub swap_data: LoadState<Swaps>,

    pub storages: LoadState<Mounts>,
    pub networks: LoadState<Networks>,
    pub dmi_data: LoadState<DMIData>,
    pub bat_data: LoadState<BatInfo>,
    pub drm_data: LoadState<Video>,
    pub osrel_data: LoadState<OsRelease>,

    pub kernel_data: LoadState<Kernel>,
    pub kmods_data: LoadState<KResult>,
    pub firmware_data: LoadState<FResult>,

    pub users_list: LoadState<Users>,
    pub groups_list: LoadState<Groups>,
    pub sysd_services_list: LoadState<SystemdServices>,
    pub boot_time: LoadState<BootTimestamps>,
    pub installed_pkgs_list: LoadState<InstalledPackages>,
    pub system: LoadState<System>,
}

impl Default for FerrixData {
    fn default() -> Self {
        Self {
            proc_data: LoadState::default(),
            prev_proc_stat: LoadState::default(),
            curr_proc_stat: LoadState::default(),
            cpu_freq: LoadState::default(),
            cpu_vulnerabilities: LoadState::default(),

            ram_data: LoadState::default(),
            swap_data: LoadState::default(),
            storages: LoadState::default(),
            networks: LoadState::default(),
            dmi_data: LoadState::default(),
            bat_data: LoadState::default(),
            drm_data: LoadState::default(),
            osrel_data: LoadState::default(),

            kernel_data: LoadState::default(),
            kmods_data: LoadState::default(),
            firmware_data: LoadState::default(),

            users_list: LoadState::default(),
            groups_list: LoadState::default(),
            sysd_services_list: LoadState::default(),
            boot_time: LoadState::default(),
            installed_pkgs_list: LoadState::default(),
            system: LoadState::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct System {
    pub hostname: Option<String>,
    pub loadavg: Option<LoadAVG>,
    pub uptime: Option<Uptime>,
    pub desktop: Option<String>,
    pub language: Option<String>,
    pub env_vars: Vec<(String, String)>,
    pub current_user: Option<String>,
    pub shell: Option<Shell>,
}

impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hostname: get_hostname(),
            loadavg: Some(LoadAVG::new()?),
            uptime: Some(Uptime::new()?),
            desktop: get_current_desktop(),
            language: get_lang(),
            env_vars: get_env_vars(),
            current_user: current_user(),
            shell: Some(Shell::new()?),
        })
    }
}

impl Default for System {
    fn default() -> Self {
        Self {
            hostname: Some("unknown-host".to_string()),
            loadavg: None,
            uptime: None,
            desktop: Some("Unknown DE".to_string()),
            language: Some("Unknown locale".to_string()),
            env_vars: Vec::new(),
            current_user: None,
            shell: None,
        }
    }
}

trait FromJson {
    fn from_json(json: Value) -> Result<Self>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        Ok(serde_json::from_value(json)?)
    }
}

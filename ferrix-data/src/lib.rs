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

pub mod load_state;
pub mod dmi;
pub mod polkit;

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
    sys::{Groups, KModules, Kernel, OsRelease, Users},
    vulnerabilities::Vulnerabilities,
};
use std::collections::HashSet;
use load_state::LoadState;

#[derive(Debug)]
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
    pub kmods_data: LoadState<KModules>,

    pub users_list: LoadState<Users>,
    pub groups_list: LoadState<Groups>,
    pub sysd_services_list: LoadState<SystemdServices>,
    pub boot_time: LoadState<BootTimestamps>,
    pub installed_pkgs_list: LoadState<InstalledPackages>,
    pub system: LoadState<crate::System>,
}

impl Default for FerrixData {
    fn default() -> Self {
        Self {
            is_polkit: false,

            cpu_usage_chart: LineChart::new(),
            selected_proc: 0,
            show_cpus_chart: HashSet::new(),
            show_chart_elements: 100,
            ram_usage_chart: LineChart::new(),
            show_mem_chart: HashSet::new(),
            show_ram_chart: true,
            show_charts_legend: true,

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
            users_list: LoadState::default(),
            groups_list: LoadState::default(),
            sysd_services_list: LoadState::default(),
            boot_time: LoadState::default(),
            installed_pkgs_list: LoadState::default(),
            system: LoadState::default(),
        }
    }
}

impl FerrixData {
    pub fn new(settings: &FXSettings) -> Self {
        let style = &settings.style;
        let thickness = settings.chart_line_thickness;

        let mut cpu_usage_chart = LineChart::new();
        cpu_usage_chart.set_style(&style.to_theme());
        cpu_usage_chart.set_line_thickness(thickness.to_u32());

        let mut ram_usage_chart = LineChart::new();
        ram_usage_chart.set_style(&style.to_theme());
        ram_usage_chart.set_line_thickness(thickness.to_u32());

        Self {
            cpu_usage_chart,
            ram_usage_chart,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct ExportManager {
    pub output_path: String,
    pub format: ExportFormat,
    pub mode: ExportMode,
    pub selected_pages: ExportPages,
    pub export_data: ExportData,
    pub status: ExportStatus,
}

impl Default for ExportManager {
    fn default() -> Self {
        Self {
            output_path: "export.json".to_string(),
            format: ExportFormat::default(),
            mode: ExportMode::default(),
            selected_pages: ExportPages::default(),
            export_data: ExportData::default(),
            status: ExportStatus::default(),
        }
    }
}

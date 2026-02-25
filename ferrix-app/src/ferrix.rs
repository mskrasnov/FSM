/* ferrix.rs
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

//! Data from `ferrix-lib`

use crate::{
    SETTINGS_PATH, dmi::DMIData, load_state::LoadState, messages::Message, pages::Page,
    settings::FXSettings, sidebar::sidebar, utils::get_home, widgets::line_charts::LineChart,
};
use ferrix_lib::{
    battery::BatInfo,
    cpu::{Processors, Stat},
    cpu_freq::CpuFreq,
    drm::Video,
    init::SystemdServices,
    net::Networks,
    parts::Mounts,
    ram::{RAM, Swaps},
    soft::InstalledPackages,
    sys::{Groups, KModules, Kernel, OsRelease, Users},
    vulnerabilities::Vulnerabilities,
};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub settings: FXSettings,
    pub data: FerrixData,
    pub scrolled_area_id: Option<&'static str>,
}

impl Default for Ferrix {
    fn default() -> Self {
        let args = std::env::args().nth(1);
        let page = match &args {
            Some(a) => Page::from(a as &str),
            None => Page::default(),
        };
        let settings =
            FXSettings::read(get_home().join(".config").join(SETTINGS_PATH)).unwrap_or_default();

        Self {
            current_page: page,
            settings: settings.clone(),
            data: FerrixData::new(&settings),
            scrolled_area_id: None,
        }
    }
}

impl Ferrix {
    pub fn theme(&self) -> iced::Theme {
        self.settings.style.to_theme()
    }

    pub fn title(&self) -> String {
        "Ferrix System Monitor".to_string()
    }

    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        message.update(self)
    }

    pub fn view<'a>(&'a self) -> iced::Element<'a, Message> {
        iced::widget::row![sidebar(self.current_page), self.current_page.page(&self)]
            .spacing(5)
            .padding(5)
            .into()
    }
}

#[derive(Debug)]
pub struct FerrixData {
    pub is_polkit: bool,

    pub proc_data: LoadState<Processors>,
    pub selected_proc: usize,
    pub prev_proc_stat: LoadState<Stat>,
    pub curr_proc_stat: LoadState<Stat>,
    pub cpu_usage_chart: LineChart,
    pub show_cpus_chart: HashSet<usize>,
    pub show_chart_elements: usize,
    pub show_charts_legend: bool,
    pub cpu_freq: LoadState<CpuFreq>,
    pub cpu_vulnerabilities: LoadState<Vulnerabilities>,

    pub ram_data: LoadState<RAM>,
    pub swap_data: LoadState<Swaps>,
    pub show_mem_chart: HashSet<usize>,
    pub show_ram_chart: bool,
    pub ram_usage_chart: LineChart,

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
        cpu_usage_chart.set_line_thickness(thickness);

        let mut ram_usage_chart = LineChart::new();
        ram_usage_chart.set_style(&style.to_theme());
        ram_usage_chart.set_line_thickness(thickness);

        Self {
            cpu_usage_chart,
            ram_usage_chart,
            ..Default::default()
        }
    }
}

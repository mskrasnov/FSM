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

//! Data model and application state

use crate::{
    SETTINGS_PATH,
    export::{ExportData, ExportFormat, ExportMode, ExportPages, ExportStatus},
    messages::Message,
    pages::Page,
    settings::FXSettings,
    sidebar::sidebar,
    utils::get_home,
};
use ferrix_data::FerrixData;
use ferrix_widgets::line_charts::LineChart;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Ferrix {
    pub current_page: Page,
    pub scrolled_area_id: Option<&'static str>,
    pub settings: FXSettings,
    pub data: FerrixData,
    pub state: FerrixState,
    pub export_manager: ExportManager,
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
            scrolled_area_id: None,
            settings: settings.clone(),
            data: FerrixData::default(),
            state: FerrixState::new(&settings),
            export_manager: ExportManager::default(),
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
pub struct FerrixState {
    pub is_dmi_polkit: bool,
    pub is_kmods_polkit: bool,
    pub selected_proc: usize,
    pub cpu_usage_chart: LineChart,
    pub show_cpus_chart: HashSet<usize>,
    pub show_chart_elements: usize,
    pub show_charts_legend: bool,
    pub show_mem_chart: HashSet<usize>,
    pub show_ram_chart: bool,
    pub ram_usage_chart: LineChart,
}

impl Default for FerrixState {
    fn default() -> Self {
        Self {
            is_dmi_polkit: false,
            is_kmods_polkit: false,
            selected_proc: 0,
            cpu_usage_chart: LineChart::new(),
            show_cpus_chart: HashSet::new(),
            show_chart_elements: 100,
            show_charts_legend: true,
            show_mem_chart: HashSet::new(),
            show_ram_chart: true,
            ram_usage_chart: LineChart::new(),
        }
    }
}

impl FerrixState {
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

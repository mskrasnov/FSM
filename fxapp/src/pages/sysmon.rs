/* sysmon.rs
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

use std::collections::HashSet;

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::cpu::Stat;
use ferrix_widgets::container::glassy_container;
use iced::{
    Color, Element, Length, Task, color,
    widget::{Id, column, scrollable},
};

use super::{PageData, PageView};
use crate::{
    ferrix::Ferrix,
    fl,
    message::{DataReceiver, Message},
    widgets::line_chart::*,
};

#[derive(Debug, Clone)]
pub struct SysMonPage {
    pub prev_proc_stat: LoadState<Stat>,
    pub curr_proc_stat: LoadState<Stat>,
    pub cpu_cores_names: HashSet<usize>,

    pub y_axis_label_width: u32,
    pub max_elements: usize,
    pub show_bat_capacity_chart: bool,

    pub cpu_chart: LineChart,
    pub mem_chart: LineChart,
    pub bat_capacity_chart: LineChart,
}

const DEFAULT_CHART_HEIGHT: f32 = 192.;

impl SysMonPage {
    pub fn new() -> Self {
        Self {
            prev_proc_stat: LoadState::Loading,
            curr_proc_stat: LoadState::Loading,
            cpu_cores_names: HashSet::new(),

            max_elements: 100,
            y_axis_label_width: 35,
            show_bat_capacity_chart: false,

            cpu_chart: LineChart::new(CPU_CHARTS_COLORS.to_vec()),
            mem_chart: LineChart::new(CPU_CHARTS_COLORS.to_vec()),
            bat_capacity_chart: LineChart::new(vec![color!(128, 64, 32)]),
        }
    }

    fn get_cpu_chart_height(&self) -> Length {
        if self.cpu_chart.series_count() > 64 {
            Length::Fixed(288.)
        } else if self.cpu_chart.series_count() > 32 {
            Length::Fixed(224.)
        } else if self.cpu_chart.series_count() > 16 {
            Length::Fixed(208.)
        } else {
            Length::Fixed(DEFAULT_CHART_HEIGHT)
        }
    }
}

impl<'a> PageView<'a> for SysMonPage {
    fn page_id() -> &'static str {
        "sysmon"
    }

    fn page_title() -> String {
        fl!("page-sysmon")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::General
    }

    fn page_contents_view(&'a self) -> iced::Element<'a, Message> {
        let charts_height = Length::Fixed(DEFAULT_CHART_HEIGHT);

        let mut items: Vec<Element<'a, Message>> = vec![
            glassy_container(fl!("sysmon-cpu-hdr"), self.cpu_chart.view())
                .height(self.get_cpu_chart_height())
                .into(),
            glassy_container(fl!("sysmon-ram-hdr"), self.mem_chart.view())
                .height(charts_height)
                .into(),
        ];

        if self.show_bat_capacity_chart {
            let bat_capacity =
                glassy_container(fl!("page-battery"), self.bat_capacity_chart.view())
                    .height(charts_height)
                    .into();
            items.push(bat_capacity);
        }

        let mut col = column![].spacing(5);
        for item in items {
            col = col.push(item);
        }

        scrollable(col)
            .spacing(5)
            .id(Id::new(Self::page_id()))
            .into()
    }
}

impl PageData for SysMonPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { Stat::new().to_load_state() },
            DataReceiver::ProcStatReceived,
        )
    }
}

#[derive(Debug, Clone)]
pub enum SysMonPageMessage {
    AddCPUCoreLineSeries,
    AddMemoryLineSeries,
    AddTotalLineSeries,
}

impl SysMonPageMessage {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        let smp = &mut fx.sysmon_page;
        match self {
            Self::AddCPUCoreLineSeries => self.add_cpu_core_line_series(smp),
            Self::AddMemoryLineSeries => self.add_ram_line_series(fx),
            Self::AddTotalLineSeries => Task::batch([
                self.add_cpu_core_line_series(smp),
                self.add_ram_line_series(fx),
                self.add_bat_line_series(fx),
            ]),
        }
    }

    fn add_cpu_core_line_series<'a>(&'a self, smp: &'a mut SysMonPage) -> Task<Message> {
        let curr_stat = &smp.curr_proc_stat;
        let prev_stat = &smp.prev_proc_stat;

        if curr_stat.is_none() || prev_stat.is_none() {
            return Task::none();
        }

        let (curr_stat, prev_stat) = (
            curr_stat.to_option().unwrap(),
            prev_stat.to_option().unwrap(),
        );
        if curr_stat.cpus.len() != prev_stat.cpus.len() {
            return Task::none();
        }
        let len = curr_stat.cpus.len();

        smp.cpu_chart.set_y_axis_format(YAxisFormat::Percentage);
        smp.cpu_chart.set_y_label_area_size(smp.y_axis_label_width);
        smp.cpu_chart.set_max_values(smp.max_elements);

        for id in 0..len {
            let percent = curr_stat.cpus[id].usage_percentage(Some(prev_stat.cpus[id])) as f64;
            if smp.cpu_cores_names.get(&id).is_none() {
                smp.cpu_chart.add_series(format!("CPU #{id}"));
                smp.cpu_chart.push_value(percent, id);

                smp.cpu_cores_names.insert(id);
            } else {
                smp.cpu_chart.push_to(id, percent);
            }
        }
        Task::none()
    }

    fn add_ram_line_series<'a>(&'a self, fx: &'a mut Ferrix) -> Task<Message> {
        let ram = &fx.mem_page.ram_data;
        if ram.is_none() {
            return Task::none();
        }
        let ram = ram.to_option().unwrap();
        let ram_usage = ram.used_ram(2).get_bytes2().unwrap_or(0) as f64;
        let ram_total = ram.total.get_bytes2().unwrap_or(0) as f64;

        let smp = &mut fx.sysmon_page;
        smp.mem_chart.set_y_axis_format(YAxisFormat::Bytes);
        smp.mem_chart.set_y_max(ram_total);
        smp.mem_chart.set_y_label_area_size(smp.y_axis_label_width);

        if smp.mem_chart.series_count() == 0 {
            let mut ram_line =
                LineSeries::new("RAM".to_string(), color!(128, 64, 255), smp.max_elements);
            ram_line.push(ram_usage);
            smp.mem_chart.push_series(ram_line);
        } else {
            smp.mem_chart.push_to(0, ram_usage);
        }
        self.add_cache_line_series_helper(fx, ram.cached.get_bytes2().unwrap_or(0) as f64);
        self.add_swap_line_series_helper(fx);

        Task::none()
    }

    fn add_cache_line_series_helper<'a>(&'a self, fx: &'a mut Ferrix, cached: f64) {
        let smp = &mut fx.sysmon_page;
        if smp.mem_chart.series_count() == 1 {
            smp.mem_chart.add_series("Cached".to_string());
            smp.mem_chart.push_to(1, cached);
        } else {
            smp.mem_chart.push_to(1, cached);
        }
    }

    fn add_swap_line_series_helper<'a>(&'a self, fx: &'a mut Ferrix) {
        let swap = &fx.mem_page.swap_data;
        if swap.is_none() {
            return;
        }

        let swap = swap.unwrap();
        let smp = &mut fx.sysmon_page;

        let len = swap.swaps.len();

        for id in 0..len {
            let series_idx = id + 2; // 1 - RAM, 2 - cache
            let current_series_cnt = smp.mem_chart.series_count();

            let swap_usage = swap.swaps[id].used_swap(2).get_bytes2().unwrap_or(0) as f64;
            let swap_name = swap.swaps[id].filename.clone();

            // вся эта хуйня как-то работает только в таком виде. Если
            // использовать то, что было в более старых версиях FSM (v0.7.1
            // и ниже), то графики будут отставать один от другого (сначала
            // RAM, потом swap, потом проц)
            //
            // Мне лень разбираться в этом дерьме, честно.
            if series_idx >= current_series_cnt {
                smp.mem_chart.add_series(swap_name);
            }
            smp.mem_chart.push_to(series_idx, swap_usage);

            // let y_max = smp.mem_chart.get_y_max();
            // let series_max = swap.swaps[id].size.get_bytes2().unwrap_or(0) as f64;
            // if series_max < y_max {
            //     smp.mem_chart.set_y_max(series_max);
            // }
        }
    }

    fn add_bat_line_series<'a>(&'a self, fx: &'a mut Ferrix) -> Task<Message> {
        let bat = &fx.bat_page.bat_info;
        if bat.is_none() {
            return Task::none();
        }
        let bat = bat.to_option().unwrap();
        if bat.bats.is_empty() {
            return Task::none();
        }
        let len = bat.bats.len();
        fx.sysmon_page.show_bat_capacity_chart = true;

        let smp = &mut fx.sysmon_page;
        smp.bat_capacity_chart
            .set_y_axis_format(YAxisFormat::Percentage);
        smp.bat_capacity_chart.set_y_max(100.);
        smp.bat_capacity_chart
            .set_y_label_area_size(smp.y_axis_label_width);
        smp.bat_capacity_chart.set_displayed_y_labels_cnt(8);
        smp.bat_capacity_chart.set_max_values(256);

        for id in 0..len {
            let bat = &bat.bats[id];
            let name = bat.name.clone().unwrap_or(format!("unknown #{id}"));
            let capacity = bat.capacity.unwrap_or(0) as f64;

            let series_idx = id;
            let current_series_cnt = smp.bat_capacity_chart.series_count();

            if series_idx >= current_series_cnt {
                smp.bat_capacity_chart.add_series(name);
            }
            smp.bat_capacity_chart.push_to(series_idx, capacity);
        }

        Task::none()
    }
}

pub const CPU_CHARTS_COLORS: &'static [Color] = &[
    color!(0xe6194b),
    // color!(0xF58231),
    // color!(0xFFE119),
    // color!(0xBFEF45),
    // color!(0x3CB44B),
    // color!(0x42D4F4),
    // color!(0x4363D8),
    // color!(0x911EB4),
    // color!(0xff00e3),
    // color!(0xffb5ba),
    // color!(0x00a800),
    // color!(0xfdffc5),
];

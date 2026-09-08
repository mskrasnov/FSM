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
use iced::{Color, Task, color};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, Message},
    widgets::line_chart::*,
};

#[derive(Debug, Clone)]
pub struct SysMonPage {
    pub prev_proc_stat: LoadState<Stat>,
    pub curr_proc_stat: LoadState<Stat>,
    pub cpu_cores_names: HashSet<usize>,
    pub cpu_chart: LineChart,
}

impl SysMonPage {
    pub fn new() -> Self {
        Self {
            prev_proc_stat: LoadState::Loading,
            curr_proc_stat: LoadState::Loading,
            cpu_cores_names: HashSet::new(),
            cpu_chart: LineChart::new(CPU_CHARTS_COLORS.to_vec()),
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
        glassy_container(
            fl!("sysmon-cpu-hdr"),
            self.cpu_chart.view(), /*text("TEST")*/
        )
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
}

impl SysMonPageMessage {
    pub fn update<'a>(self, smp: &'a mut SysMonPage) -> Task<Message> {
        match self {
            Self::AddCPUCoreLineSeries => self.add_cpu_core_line_series(smp),
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
        smp.cpu_chart.set_max_values(100);

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
}

pub const CPU_CHARTS_COLORS: &'static [Color] = &[
    color!(0xe6194b),
    color!(0xF58231),
    color!(0xFFE119),
    color!(0xBFEF45),
    color!(0x3CB44B),
    color!(0x42D4F4),
    color!(0x4363D8),
    color!(0x911EB4),
    color!(0xff00e3),
    color!(0xffb5ba),
    color!(0x00a800),
    color!(0xfdffc5),
];

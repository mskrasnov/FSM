/* cpu_charts.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
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

//! CPU usage charts

use crate::{
    Message,
    ferrix::FerrixState,
    fl,
    messages::{ButtonsMessage, DataReceiverMessage},
};
use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::cpu::Stat;
use ferrix_widgets::container::glassy_container;
use iced::{
    Alignment::Center,
    Element, Task,
    widget::{column, container, row, slider, space, text, toggler},
};

#[derive(Debug, Clone)]
pub struct SysmonPage<'a> {
    pub current: &'a LoadState<Stat>,
    pub previous: &'a LoadState<Stat>,
    pub state: &'a FerrixState,
}

impl<'a> SysmonPage<'a> {
    pub const IS_SPECIAL: bool = false;

    pub fn new(
        current: &'a LoadState<Stat>,
        previous: &'a LoadState<Stat>,
        state: &'a FerrixState,
    ) -> Self {
        Self {
            current,
            previous,
            state,
        }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let stat = Stat::new();
                stat.to_load_state()
            },
            |val| DataReceiverMessage::ProcStatReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        usage_charts_page(&self.state, &self.current, &self.previous).into()
    }
}

fn usage_charts_page<'a>(
    fs: &'a FerrixState,
    cur_stat: &'a LoadState<Stat>,
    prev_stat: &'a LoadState<Stat>,
) -> container::Container<'a, Message> {
    if cur_stat.is_none() || prev_stat.is_none() {
        return container(text(fl!("sysmon-cpu-unk")).style(text::danger));
    }
    let cur_stat = cur_stat.to_option().unwrap();
    let prev_stat = prev_stat.to_option().unwrap();

    if cur_stat.cpus.len() != prev_stat.cpus.len() {
        return container(text(fl!("sysmon-cpu-brk")));
    }

    let mx = row![
        text(fl!("sysmon-x-axis")),
        slider(10.0..=120., fs.show_chart_elements as f64, |elems| {
            Message::DataReceiver(
                crate::messages::DataReceiverMessage::ChangeShowCPUChartElements(elems as usize),
            )
        })
        .width(200.),
        text(format!("{}", fs.show_chart_elements))
    ]
    .align_y(Center)
    .spacing(5);

    let line_widget = column![
        row![
            toggler(fs.show_charts_legend)
                .label(fl!("sysmon-toggle"))
                .on_toggle(|show| Message::Buttons(ButtonsMessage::ChangeLegendShow(show))),
            space::horizontal(),
            mx,
        ]
        .align_y(Center)
        .spacing(5),
        glassy_container(fl!("sysmon-cpu-hdr"), fs.cpu_usage_chart.view()),
        glassy_container(fl!("sysmon-ram-hdr"), fs.ram_usage_chart.view()),
    ]
    .spacing(5);

    container(line_widget)
}

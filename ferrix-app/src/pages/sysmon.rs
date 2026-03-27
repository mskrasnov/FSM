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

use crate::{DataLoadingState, Message, ferrix::FerrixData, fl, messages::ButtonsMessage};
use ferrix_lib::cpu::Stat;
use ferrix_widgets::container::glassy_container;
use iced::{
    Alignment::Center,
    widget::{column, container, row, slider, space, text, toggler},
};

pub fn usage_charts_page<'a>(
    fx: &'a FerrixData,
    cur_stat: &'a DataLoadingState<Stat>,
    prev_stat: &'a DataLoadingState<Stat>,
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
        slider(10.0..=120., fx.show_chart_elements as f64, |elems| {
            Message::DataReceiver(
                crate::messages::DataReceiverMessage::ChangeShowCPUChartElements(elems as usize),
            )
        })
        .width(200.),
        text(format!("{}", fx.show_chart_elements))
    ]
    .align_y(Center)
    .spacing(5);

    let line_widget = column![
        row![
            toggler(fx.show_charts_legend)
                .label(fl!("sysmon-toggle"))
                .on_toggle(|show| Message::Buttons(ButtonsMessage::ChangeLegendShow(show))),
            space::horizontal(),
            mx,
        ]
        .align_y(Center)
        .spacing(5),
        glassy_container(fl!("sysmon-cpu-hdr"), fx.cpu_usage_chart.view()),
        glassy_container(fl!("sysmon-ram-hdr"), fx.ram_usage_chart.view()),
    ]
    .spacing(5);

    container(line_widget)
}

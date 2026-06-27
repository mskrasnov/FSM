/* system.rs
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

//! Summary information about system

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    widgets::table::{InfoRow, kv_info_table},
};
use ferrix_data::System;
use ferrix_lib::sys::{LoadAVG, Uptime};
use iced::widget::{Id, container, scrollable};

pub fn system_page<'a>(system: &'a DataLoadingState<System>) -> container::Container<'a, Message> {
    match system {
        DataLoadingState::Loaded(sys) => {
            let rows = vec![
                InfoRow::new(fl!("misc-hostname"), sys.hostname.clone()),
                InfoRow::new(
                    fl!("misc-loadavg"),
                    Some(match &sys.loadavg {
                        Some(loadavg) => string_loadavg(loadavg),
                        None => format!("???"),
                    }),
                ),
                InfoRow::new(
                    fl!("misc-uptime"),
                    Some(match &sys.uptime {
                        Some(uptime) => string_uptime(uptime),
                        None => format!("???"),
                    }),
                ),
                InfoRow::new(fl!("misc-de"), sys.desktop.clone()),
                InfoRow::new(fl!("misc-lang"), sys.language.clone()),
                InfoRow::new(fl!("misc-user"), sys.current_user.clone()),
                InfoRow::new(
                    fl!("misc-shell"),
                    sys.shell.as_ref().map(|shell| format!("{shell}")),
                ),
                InfoRow::new("Machine ID".to_string(), sys.machine_id.clone()),
            ];

            let sys_table = container(kv_info_table(rows)).style(container::rounded_box);

            container(
                scrollable(sys_table)
                    .spacing(5)
                    .id(Id::new(super::Page::SystemMisc.page_id())),
            )
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

pub fn string_loadavg(lavg: &LoadAVG) -> String {
    format!("1min: {}\n5min: {}\n15min: {}", lavg.0, lavg.1, lavg.2)
}

fn string_uptime(uptime: &Uptime) -> String {
    fl!(
        "misc-uptime-val",
        up = string_time(uptime.0),
        down = string_time(uptime.1)
    )
}

pub fn string_time(time: f32) -> String {
    let hours = (time / 3600.) as u32;
    let remain_secs_after_hours = time % 3600.;
    let mins = (remain_secs_after_hours / 60.) as u32;
    let secs = (remain_secs_after_hours % 60.) as u32;

    format!(
        "{}{}:{}{}:{}{}",
        if hours < 10 { "0" } else { "" },
        hours,
        if mins < 10 { "0" } else { "" },
        mins,
        if secs < 10 { "0" } else { "" },
        secs,
    )
}

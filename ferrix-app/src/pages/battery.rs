/* battery.rs
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

//! Battery page

use crate::{
    Message, fl,
    messages::DataReceiverMessage,
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::battery::{BatInfo, Battery, Level, Status};

use iced::{
    Alignment::Center,
    Element, Length, Task,
    widget::{
        Id, center, column, container, progress_bar, row, scrollable, space::horizontal, text,
    },
};

#[derive(Debug, Clone)]
pub struct BatPage<'a> {
    pub bat_info: &'a LoadState<BatInfo>,
}

impl<'a> BatPage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "bat";

    pub fn new(bat_info: &'a LoadState<BatInfo>) -> Self {
        Self { bat_info }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let bat = BatInfo::new();
                bat.to_load_state()
            },
            |val| DataReceiverMessage::BatInfoReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match &self.bat_info {
            LoadState::Loaded(bat_info) => self.page(bat_info),
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }

    fn page(&self, bat_info: &'a BatInfo) -> Element<'a, Message> {
        let mut bat_list = column![].spacing(5);
        if bat_info.bats.is_empty() {
            bat_list = bat_list.push(center(
                column![text(fl!("bat-not-found")).style(text::secondary).size(16)].spacing(5),
            ));
            return container(bat_list).into();
        }

        for bat in &bat_info.bats {
            bat_list = bat_list.push(
                text(fl!(
                    "bat-header",
                    name = match &bat.name {
                        Some(name) => name.to_string(),
                        None => fl!("bat-unknown-name"),
                    }
                ))
                .style(text::warning),
            );
            let capacity = bat.capacity.unwrap_or(0);
            bat_list = bat_list.push(
                row![
                    text(fl!("bat-capacity")),
                    horizontal(),
                    progress_bar(0.0..=100., capacity as f32)
                        .length(Length::FillPortion(2))
                        .style(move |t: &iced::Theme| {
                            let p = t.palette();
                            progress_bar::Style {
                                bar: iced::Background::Color(match capacity {
                                    40..=100 => p.success,
                                    21..40 => p.warning,
                                    _ => p.danger,
                                }),
                                ..progress_bar::primary(t)
                            }
                        }),
                ]
                .spacing(5)
                .align_y(Center),
            );
            bat_list = bat_list.push(bat_table(bat));
        }
        container(scrollable(bat_list).spacing(5).id(Id::new(Self::PAGE_ID))).into()
    }
}

fn bat_table<'a>(bat: &'a Battery) -> container::Container<'a, Message> {
    let rows = vec![
        InfoRow::new(
            fl!("bat-status"),
            Some(match &bat.status {
                Some(Status::Full) => fl!("bat-status-ful"),
                Some(Status::Discharging) => fl!("bat-status-dis"),
                Some(Status::Charging) => fl!("bat-status-cha"),
                Some(Status::NotCharging) => fl!("bat-status-noc"),
                Some(Status::None) => fl!("bat-status-non"),
                Some(Status::Unknown(status)) => fl!("bat-status-unknown", status = status),
                None => fl!("bat-status-isnpresent"),
            }),
        ),
        InfoRow::new(
            fl!("bat-capacity"),
            Some(format!(
                "{} ({}%)",
                match &bat.capacity_level {
                    Some(Level::Full) => fl!("bat-lvl-ful"),
                    Some(Level::Normal) => fl!("bat-lvl-nor"),
                    Some(Level::High) => fl!("bat-lvl-hig"),
                    Some(Level::Low) => fl!("bat-lvl-low"),
                    Some(Level::Critical) => fl!("bat-lvl-cri"),
                    Some(Level::Unknown(lvl)) => fl!("bat-lvl-unk", lbl = lvl),
                    _ => fl!("bat-lvl-non"),
                },
                match bat.capacity {
                    Some(capacity) => format!("{capacity}"),
                    None => format!("none"),
                }
            )),
        ),
        InfoRow::new(fl!("bat-chtypes"), bat.charge_types.clone()),
        InfoRow::new(
            fl!("bat-estimated"),
            bat.estimated_time
                .and_then(|h| Some(format!("{h:.2} {}", fl!("bat-es-hours")))),
        ),
        InfoRow::new(
            fl!("bat-health"),
            bat.health.and_then(|h| Some(format!("{h:.2}%"))),
        ),
        InfoRow::new(fl!("bat-tech"), bat.technology.clone()),
        InfoRow::new(fl!("bat-cycle-cnt"), fmt_val(bat.cycle_count)),
        InfoRow::new(
            fl!("bat-volt-min-des"),
            bat.voltage_min_design.and_then(|v| Some(format!("{v} V"))),
        ),
        InfoRow::new(
            fl!("bat-volt-now"),
            bat.voltage_now.and_then(|v| Some(format!("{v} V"))),
        ),
        InfoRow::new(
            fl!("bat-power-now"),
            bat.power_now.and_then(|p| Some(format!("{p} W"))),
        ),
        InfoRow::new(
            fl!("bat-energy-full-des"),
            bat.energy_full_design.and_then(|e| Some(format!("{e} Wh"))),
        ),
        InfoRow::new(
            fl!("bat-energy-full"),
            bat.energy_full.and_then(|e| Some(format!("{e} Wh"))),
        ),
        InfoRow::new(
            fl!("bat-energy-now"),
            bat.energy_now.and_then(|e| Some(format!("{e} Wh"))),
        ),
        InfoRow::new(fl!("bat-model"), bat.model_name.clone()),
        InfoRow::new(fl!("bat-manufact"), bat.manufacturer.clone()),
        InfoRow::new(fl!("bat-serial"), bat.serial_number.clone()),
    ];
    container(kv_info_table(rows)).style(container::rounded_box)
}

/* firmware.rs
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

//! Firmware settings (attributes) page

use crate::{
    fl,
    load_state::LoadState,
    messages::{ButtonsMessage, DataReceiverMessage, Message},
    widgets::table::{InfoRow, hdr_name, kv_info_table},
};
use ferrix_data::firmware::FResult;
use ferrix_lib::firmware::Attribute;
use iced::{
    Element, Length, Task,
    widget::{button, column, container, scrollable, table, text},
};

#[derive(Debug, Clone)]
pub struct FirmwarePage<'a> {
    firmware: &'a LoadState<FResult>,
}

impl<'a> FirmwarePage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "uefi";

    pub fn new(firmware: &'a LoadState<FResult>) -> Self {
        Self { firmware }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move { ferrix_data::firmware::get_firmware_data().await },
            |val| DataReceiverMessage::FirmwareDataReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.firmware {
            LoadState::Loaded(firmware) => match firmware {
                FResult::Ok { data } => {
                    let rows = vec![InfoRow::new(
                        fl!("frmwr-drv"),
                        Some(data.driver_name.clone()),
                    )];
                    scrollable(
                        column![
                            text(fl!("frmwr-gen")).style(text::warning),
                            container(kv_info_table(rows)).style(container::rounded_box),
                            text(fl!("frmwr-params")).style(text::warning),
                            container(frmwr_table(&data.attributes)).style(container::rounded_box),
                        ]
                        .spacing(5),
                    )
                    .spacing(5)
                    .into()
                }
                FResult::Err { error } => super::error_page(error).into(),
            },
            LoadState::Loading => super::loading_page().into(),
            LoadState::Error(why) => super::error_page(why).into(),
        }
    }
}

fn frmwr_table<'a>(rows: &'a [Attribute]) -> table::Table<'a, Message> {
    let columns = [
        table::column(hdr_name(fl!("frmwr-name")), |row: &'a Attribute| {
            button(text(&row.display_name))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.display_name.clone(),
                )))
        }),
        table::column(hdr_name(fl!("frmwr-val")), |row: &'a Attribute| {
            button(
                text(&row.current_value)
                    .wrapping(text::Wrapping::WordOrGlyph)
                    .style(match &row.current_value as &str {
                        "Enable" => text::success,
                        "Disable" => text::danger,
                        _ => text::default,
                    }),
            )
            .style(button::text)
            .padding(0)
            .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                row.current_value.clone(),
            )))
        })
        .width(Length::FillPortion(1)),
        table::column(hdr_name(fl!("frmwr-pval")), |row: &'a Attribute| {
            button(text(&row.possible_values))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.possible_values.clone(),
                )))
        })
        .width(Length::FillPortion(1)),
        table::column(hdr_name(fl!("frmwr-type")), |row: &'a Attribute| {
            button(text(&row.param_type))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.param_type.clone(),
                )))
        }),
    ];
    table(columns, rows).padding(2).width(Length::Fill)
}

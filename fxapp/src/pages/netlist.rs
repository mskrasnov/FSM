/* netlist.rs
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

//! Pages with information about network interfaces

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::net::{Network, Networks};
use iced::{
    Element, Font, Length, Task,
    alignment::Horizontal,
    widget::{button, column, container, scrollable, table, text},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, KeyboardAndMouse, Message},
    pages::PageVariant,
    widgets::table::{InfoRow, fmt_val, hdr_name, kv_info_table},
};

#[derive(Debug, Clone)]
pub struct NetStatPage {
    pub net: LoadState<Networks>,
    pub page_type: PageVariant,
}

impl NetStatPage {
    pub fn new() -> Self {
        Self {
            net: LoadState::Loading,
            page_type: PageVariant::NetworkStatistics,
        }
    }

    fn net_stat_table<'a>(&'a self, rows: &'a [Network]) -> Element<'a, Message> {
        let columns = [
            table::column(hdr_name(fl!("net-int")), |row: &'a Network| {
                button(
                    text(&row.name)
                        .wrapping(text::Wrapping::WordOrGlyph)
                        .font(Font::MONOSPACE),
                )
                .style(button::text)
                .padding(0)
                .on_press(Message::KeyboardAndMouse(
                    KeyboardAndMouse::CopyButtonPressed(row.name.clone()),
                ))
            }),
            table::column(hdr_name("RX Bytes"), |row: &'a Network| {
                button(num_item(row.statistics.rx_bytes))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.rx_bytes.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("RX Pkt"), |row: &'a Network| {
                button(num_item(row.statistics.rx_packets))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.rx_packets.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("RX Err"), |row: &'a Network| {
                button(num_item(row.statistics.rx_errors))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.rx_errors.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("RX Drop"), |row: &'a Network| {
                button(num_item(row.statistics.rx_dropped))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.rx_dropped.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("TX Bytes"), |row: &'a Network| {
                button(num_item(row.statistics.tx_bytes))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.tx_bytes.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("TX Pkt"), |row: &'a Network| {
                button(num_item(row.statistics.tx_packets))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.tx_packets.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("TX Err"), |row: &'a Network| {
                button(num_item(row.statistics.tx_errors))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.tx_errors.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
            table::column(hdr_name("TX Drop"), |row: &'a Network| {
                button(num_item(row.statistics.tx_dropped))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.statistics.tx_dropped.to_string()),
                    ))
            })
            .align_x(Horizontal::Right),
        ];

        scrollable(
            container(table(columns, rows).padding(2).width(Length::Fill))
                .style(container::rounded_box),
        )
        .spacing(5)
        .id(Self::page_id())
        .into()
    }
}

fn num_item<'a>(num: u64) -> text::Text<'a> {
    text(num).font(Font::MONOSPACE)
}

impl<'a> PageView<'a> for NetStatPage {
    // WARN: only for NetworkStatistics page!!!
    fn page_id() -> &'static str {
        "netstat"
    }

    // WARN: only for NetworkStatistics page!!!
    fn page_title() -> String {
        fl!("page-nstat")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Network
    }

    fn page_contents_view(&'a self) -> Element<'a, crate::message::Message> {
        match &self.net {
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetNetworkData),
            LoadState::Loaded(net) => self.net_stat_table(&net.networks),
        }
    }
}

impl PageData for NetStatPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { Networks::new().to_load_state() },
            DataReceiver::NetworkDataReceived,
        )
    }
}

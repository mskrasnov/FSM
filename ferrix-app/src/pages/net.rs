/* net.rs
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

//! Pages with information about network

use crate::{
    fl,
    load_state::LoadState,
    messages::{ButtonsMessage, Message},
    widgets::table::{InfoRow, hdr_name, kv_info_table},
};
use ferrix_lib::net::{Network, Networks};

use iced::{
    Element, Font, Length,
    widget::{button, column, container, scrollable, table, text},
};

#[derive(Debug, Clone)]
pub struct NetStatPage<'a> {
    net: &'a LoadState<Networks>,
}

impl<'a> NetStatPage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "nstat";

    pub fn new(net: &'a LoadState<Networks>) -> Self {
        Self { net }
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.net {
            LoadState::Loaded(net) => {
                scrollable(container(net_stat_table(&net.networks)).style(container::rounded_box))
                    .spacing(5)
                    .id(Self::PAGE_ID)
                    .into()
            }
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }
}

fn net_stat_table<'a>(rows: &'a [Network]) -> table::Table<'a, Message> {
    let columns = [
        table::column(hdr_name(fl!("net-int")), |row: &'a Network| {
            button(
                text(&row.name)
                    .wrapping(text::Wrapping::WordOrGlyph)
                    .font(Font::MONOSPACE),
            )
            .style(button::text)
            .padding(0)
            .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                row.name.clone(),
            )))
        }),
        table::column(hdr_name("RX Bytes"), |row: &'a Network| {
            button(text(row.statistics.rx_bytes))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.rx_bytes.to_string(),
                )))
        }),
        table::column(hdr_name("RX Pkt"), |row: &'a Network| {
            button(text(row.statistics.rx_packets))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.rx_packets.to_string(),
                )))
        }),
        table::column(hdr_name("RX Err"), |row: &'a Network| {
            button(text(row.statistics.rx_errors))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.rx_errors.to_string(),
                )))
        }),
        table::column(hdr_name("RX Drop"), |row: &'a Network| {
            button(text(row.statistics.rx_dropped))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.rx_dropped.to_string(),
                )))
        }),
        table::column(hdr_name("TX Bytes"), |row: &'a Network| {
            button(text(row.statistics.tx_bytes))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.tx_bytes.to_string(),
                )))
        }),
        table::column(hdr_name("TX Pkt"), |row: &'a Network| {
            button(text(row.statistics.tx_packets))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.tx_packets.to_string(),
                )))
        }),
        table::column(hdr_name("TX Err"), |row: &'a Network| {
            button(text(row.statistics.tx_errors))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.tx_errors.to_string(),
                )))
        }),
        table::column(hdr_name("TX Drop"), |row: &'a Network| {
            button(text(row.statistics.tx_dropped))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    row.statistics.tx_dropped.to_string(),
                )))
        }),
    ];
    table(columns, rows).padding(2).width(Length::Fill)
}

pub fn net_page<'a>(net: &'a LoadState<Networks>) -> container::Container<'a, Message> {
    match net {
        LoadState::Loaded(net) => {
            let mut net_data = column![].spacing(5);
            for net in &net.networks {
                let header = text(fl!("net-adp", adp = net.name.clone())).style(text::warning);
                let rows = vec![
                    InfoRow::new(fl!("net-os"), Some(net.operstate.to_string())),
                    InfoRow::new(fl!("net-addr"), Some(net.address.clone())),
                    InfoRow::new(fl!("net-bcast"), Some(net.broadcast.clone())),
                    InfoRow::new(fl!("net-mtu"), Some(net.mtu.to_string())),
                ];
                net_data = net_data.push(header);
                net_data =
                    net_data.push(container(kv_info_table(rows)).style(container::rounded_box));
            }

            container(
                scrollable(net_data)
                    .spacing(5)
                    .id(super::Page::Network.page_id()),
            )
        }
        LoadState::Loading => super::loading_page(),
        LoadState::Error(why) => super::error_page(why),
    }
}

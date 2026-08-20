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
use ferrix_lib::{
    net::{Network, Networks},
    utils::Size,
};
use ferrix_widgets::tooltip::icon_tooltip;
use iced::{
    Alignment::Center,
    Element, Font, Length, Task,
    alignment::Horizontal,
    widget::{button, column, container, row, rule, scrollable, space, table, text},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, KeyboardAndMouse, Message},
    pages::PageVariant,
    widgets::table::{InfoRow, hdr_name, kv_info_table},
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
                button(num_bytes(row.statistics.rx_bytes))
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
                button(num_bytes(row.statistics.tx_bytes))
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

    fn net_list_tables<'a>(&'a self, net: &'a [Network]) -> Element<'a, Message> {
        let mut net_data = column![].spacing(5);
        for n in net {
            let header = text(fl!("net-adp", adp = n.name.clone())).style(text::warning);
            let rows = vec![
                InfoRow::new(fl!("net-os"), Some(n.operstate.to_string())),
                InfoRow::new(fl!("net-addr"), Some(n.address.clone())),
                InfoRow::new(fl!("net-bcast"), Some(n.broadcast.clone())),
                InfoRow::new(fl!("net-mtu"), Some(n.mtu.to_string())),
            ];
            net_data = net_data.push(header);
            net_data = net_data.push(container(kv_info_table(rows)).style(container::rounded_box));
        }

        container(scrollable(net_data).spacing(5).id(Self::page_id2())).into()
    }

    pub fn page_id2() -> &'static str {
        "netlist"
    }

    pub fn page_title2() -> String {
        fl!("page-net")
    }
}

fn num_bytes<'a>(num: u64) -> Element<'a, Message> {
    let size = Size::B(num).round(2).unwrap_or_default();
    let btn = button(icon_tooltip("about", format!("{size}")))
        .style(button::text)
        .padding(0)
        .on_press(Message::KeyboardAndMouse(
            KeyboardAndMouse::CopyButtonPressed(format!("{size}")),
        ));
    row![text(num).font(Font::MONOSPACE), btn]
        .spacing(5)
        .align_y(Center)
        .into()
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
            LoadState::Loaded(net) => match self.page_type {
                PageVariant::NetworkInterfaces => self.net_list_tables(&net.networks),
                PageVariant::NetworkStatistics => self.net_stat_table(&net.networks),
                _ => panic!("Unknown page type: {:?}", self.page_type),
            },
        }
    }

    fn view(&'a self) -> Element<'a, Message> {
        let title_str = match &self.page_type {
            PageVariant::NetworkInterfaces => Self::page_title2(),
            PageVariant::NetworkStatistics => Self::page_title(),
            _ => "Unknown page".to_string(),
        };
        let title = column![
            row![text(title_str).size(20), space::horizontal(),]
                .align_y(Center)
                .spacing(5),
            rule::horizontal(1),
        ]
        .spacing(2);

        column![title, self.page_contents_view()].spacing(5).into()
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

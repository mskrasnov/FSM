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
    messages::Message,
    widgets::table::{InfoRow, kv_info_table},
};
use ferrix_lib::net::Networks;

use iced::widget::{column, container, scrollable, text};

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

/* env.rs
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

//! Environment variables list

use crate::{
    Message,
    load_state::DataLoadingState,
    widgets::table::{InfoRow, kv_info_table},
};
use ferrix_data::System;
use iced::widget::{Id, container, scrollable};

pub fn env_page<'a>(system: &'a DataLoadingState<System>) -> container::Container<'a, Message> {
    match system {
        DataLoadingState::Loaded(sys) => {
            let mut rows = Vec::with_capacity(sys.env_vars.len());
            for var in &sys.env_vars {
                rows.push(InfoRow::new(&var.0, Some(var.1.to_string())));
            }
            let table = container(kv_info_table(rows)).style(container::rounded_box);
            container(
                scrollable(table)
                    .spacing(5)
                    .id(Id::new(super::Page::Environment.page_id())),
            )
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

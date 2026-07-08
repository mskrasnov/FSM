/* mem.rs
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

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::ram::RAM;
use iced::{
    Task,
    widget::{button, text},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, Message},
};

#[derive(Debug, Clone)]
pub struct MemoryPage {
    pub ram_data: LoadState<RAM>,
}

impl MemoryPage {
    pub fn new() -> Self {
        Self {
            ram_data: LoadState::Loading,
        }
    }
}

impl<'a> PageView<'a> for MemoryPage {
    fn page_id() -> &'static str {
        "mem"
    }

    fn page_title() -> String {
        fl!("page-memory")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::System
    }

    fn page_title_controls(&'a self) -> Option<iced::Element<'a, Message>> {
        Some(
            button("Update")
                .on_press(Message::DataReceiver(DataReceiver::GetRAMData))
                .style(button::subtle)
                .padding(2)
                .into(),
        )
    }

    fn page_contents_view(&'a self) -> iced::Element<'a, Message> {
        text(format!("{:#?}", &self.ram_data)).into()
    }
}

impl PageData for MemoryPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { RAM::new().to_load_state() },
            DataReceiver::RAMDataReceived,
        )
    }
}

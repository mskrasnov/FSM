/* session.rs
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

//! Session info (desktop environment, window manager, etc.)

use crate::{
    messages::{DataReceiverMessage, Message},
    widgets::table::{InfoRow, kv_info_table},
};
use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::desktop::SessionInfo;
use iced::{Element, Task, widget::container};

#[derive(Debug, Clone)]
pub struct SessionPage<'a> {
    session: &'a LoadState<SessionInfo>,
}

impl<'a> SessionPage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "de";

    pub fn new(session: &'a LoadState<SessionInfo>) -> Self {
        Self { session }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move { SessionInfo::new().to_load_state() },
            DataReceiverMessage::SessionDataReceived,
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.session {
            LoadState::Loaded(session) => self.session_view(session),
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }

    fn session_view(&self, session: &'a SessionInfo) -> Element<'a, Message> {
        let rows = vec![
            InfoRow::new("Desktop", session.desktop.clone()),
            InfoRow::new("Version", session.desktop_ver.clone()),
            InfoRow::new("Window manager", session.window_manager.clone()),
            InfoRow::new("Session type", None),
            InfoRow::new("WM Theme", None),
            InfoRow::new("GTK2 Theme", None),
            InfoRow::new("GTK3 Theme", None),
            InfoRow::new("Icons", None),
            InfoRow::new("Font", None),
            InfoRow::new(
                "Wallpaper",
                Some("Click to open in standard viewer".to_string()),
            ),
        ];
        container(kv_info_table(rows))
            .style(container::rounded_box)
            .into()
    }
}

/* groups.rs
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

//! Groups list page

use crate::{
    Message, fl,
    messages::DataReceiverMessage,
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_data::load_state::LoadState;
use ferrix_lib::sys::Groups;

use iced::{
    Element, Task,
    widget::{Id, column, container, scrollable, text},
};

#[derive(Debug, Clone)]
pub struct GroupsPage<'a> {
    groups: &'a LoadState<Groups>,
}

impl<'a> GroupsPage<'a> {
    pub const PAGE_ID: &'static str = "grp";
    pub const IS_SPECIAL: bool = false;

    pub fn new(groups: &'a LoadState<Groups>) -> Self {
        Self { groups }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let grp = Groups::new();
                match grp {
                    Ok(mut grp) => {
                        grp.groups.sort_by_key(|grp| grp.gid);
                        LoadState::Loaded(grp)
                    }
                    Err(why) => LoadState::Error(why.to_string()),
                }
            },
            |val| DataReceiverMessage::GroupsDataReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match &self.groups {
            LoadState::Loaded(grp) => groups_page(grp).into(),
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }
}

fn groups_page<'a>(groups: &'a Groups) -> container::Container<'a, Message> {
    let mut groups_list = column![].spacing(5);
    for grp in &groups.groups {
        let rows = vec![
            InfoRow::new(fl!("groups-name"), Some(grp.name.clone())),
            InfoRow::new(fl!("groups-id"), fmt_val(Some(grp.gid))),
            InfoRow::new(fl!("groups-members"), Some(format!("{:?}", &grp.users))),
        ];
        let grp_view = column![
            text(fl!("groups-group", group_no = grp.gid)).style(text::warning),
            container(kv_info_table(rows)).style(container::rounded_box),
        ]
        .spacing(5);
        groups_list = groups_list.push(grp_view);
    }
    container(
        scrollable(groups_list)
            .spacing(5)
            .id(Id::new(super::Page::Groups.page_id())),
    )
}

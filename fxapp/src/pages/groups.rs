/* groups.rs
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

//! Groups list page

use ferrix_data::load_state::LoadState;
use ferrix_lib::sys::Groups;

use crate::{
    fl,
    message::{DataReceiver, Message},
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};
use iced::{
    Element, Task,
    widget::{Id, column, container, scrollable, text},
};

use super::{PageData, PageView};

#[derive(Debug, Clone)]
pub struct GroupsPage {
    pub groups: LoadState<Groups>,
}

impl GroupsPage {
    pub fn new() -> Self {
        Self {
            groups: LoadState::default(),
        }
    }

    fn groups_page<'a>(&'a self, groups: &'a Groups) -> Element<'a, Message> {
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
                .id(Id::new(Self::page_id())),
        )
        .into()
    }
}

impl<'a> PageView<'a> for GroupsPage {
    fn page_id() -> &'static str {
        "groups"
    }

    fn page_title() -> String {
        fl!("page-groups")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Admin
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.groups {
            LoadState::Loaded(groups) => self.groups_page(groups),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetGroupsData),
            LoadState::Loading => super::loading_page(),
        }
    }
}

impl PageData for GroupsPage {
    fn get_data() -> Task<DataReceiver> {
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
            |val| DataReceiver::GroupsDataReceived(val),
        )
    }
}

/* users.rs
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

//! Users list page

use ferrix_data::load_state::LoadState;
use ferrix_lib::sys::Users;

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
pub struct UsersPage {
    pub users: LoadState<Users>,
}

impl UsersPage {
    pub fn new() -> Self {
        Self {
            users: LoadState::default(),
        }
    }

    fn users_page<'a>(&'a self, users: &'a Users) -> Element<'a, Message> {
        let mut users_list = column![].spacing(5);
        for usr in &users.users {
            let rows = vec![
                InfoRow::new(fl!("users-name"), Some(usr.name.clone())),
                InfoRow::new(fl!("users-id"), fmt_val(Some(usr.uid))),
                InfoRow::new(fl!("users-gid"), fmt_val(Some(usr.gid))),
                InfoRow::new(fl!("users-gecos"), usr.gecos.clone()),
                InfoRow::new(fl!("users-home"), Some(usr.home_dir.clone())),
                InfoRow::new(fl!("users-shell"), Some(usr.login_shell.clone())),
            ];
            let usr_view = column![
                text(fl!("users-hdr", id = usr.uid)).style(text::warning),
                container(kv_info_table(rows)).style(container::rounded_box),
            ]
            .spacing(5);
            users_list = users_list.push(usr_view);
        }
        container(
            scrollable(users_list)
                .spacing(5)
                .id(Id::new(Self::page_id())),
        )
        .into()
    }
}

impl<'a> PageView<'a> for UsersPage {
    fn page_id() -> &'static str {
        "users"
    }

    fn page_title() -> String {
        fl!("page-users")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Admin
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.users {
            LoadState::Loaded(users) => self.users_page(users),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetUsersData),
            LoadState::Loading => super::loading_page(),
        }
    }
}

impl PageData for UsersPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move {
                let usr = Users::new();
                match usr {
                    Ok(mut usr) => {
                        usr.users.sort_by_key(|usr| usr.uid);
                        LoadState::Loaded(usr)
                    }
                    Err(why) => LoadState::Error(why.to_string()),
                }
            },
            |val| DataReceiver::UsersDataReceived(val),
        )
    }
}

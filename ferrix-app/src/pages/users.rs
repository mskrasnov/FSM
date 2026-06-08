/* users.rs
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

//! Users list page

use crate::{
    Message, fl,
    messages::DataReceiverMessage,
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_data::load_state::LoadState;
use ferrix_lib::sys::Users;

use iced::{
    Element, Task,
    widget::{Id, column, container, scrollable, text},
};

#[derive(Debug, Clone)]
pub struct UsersPage<'a> {
    users: &'a LoadState<Users>,
}

impl<'a> UsersPage<'a> {
    pub const PAGE_ID: &'static str = "usr";
    pub const IS_SPECIAL: bool = false;

    pub fn new(users: &'a LoadState<Users>) -> Self {
        Self { users }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
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
            |val| DataReceiverMessage::UsersDataReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match &self.users {
            LoadState::Loaded(users) => users_page(users).into(),
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }
}

fn users_page<'a>(users: &'a Users) -> container::Container<'a, Message> {
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
            .id(Id::new(UsersPage::PAGE_ID)),
    )
}

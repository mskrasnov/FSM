/* distro.rs
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

//! Page with information about installed Linux distro

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::sys::OsRelease;
use iced::{
    Element, Task,
    widget::{Id, column, container, scrollable},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, Message},
    pages::GroupVariant,
    widgets::table::{InfoRow, kv_info_table},
};

#[derive(Debug, Clone)]
pub struct OsRelPage {
    pub osrel: LoadState<OsRelease>,
}

impl OsRelPage {
    pub fn new() -> Self {
        Self {
            osrel: LoadState::Loading,
        }
    }

    fn distro_table<'a>(&'a self, osrel: &'a OsRelease) -> Element<'a, Message> {
        let mut os_data = column![].spacing(5);
        let rows = vec![
            InfoRow::new(fl!("distro-name"), Some(osrel.name.clone())),
            InfoRow::new(fl!("distro-id"), osrel.id.clone()),
            InfoRow::new(fl!("distro-like"), osrel.id_like.clone()),
            InfoRow::new(fl!("distro-cpe"), osrel.cpe_name.clone()),
            InfoRow::new(fl!("distro-variant"), osrel.variant.clone()),
            InfoRow::new(fl!("distro-version"), osrel.version.clone()),
            InfoRow::new(fl!("distro-codename"), osrel.version_codename.clone()),
            InfoRow::new(fl!("distro-build-id"), osrel.build_id.clone()),
            InfoRow::new(fl!("distro-image-id"), osrel.image_id.clone()),
            InfoRow::new(fl!("distro-image-ver"), osrel.image_version.clone()),
            InfoRow::new(fl!("distro-homepage"), osrel.home_url.clone()),
            InfoRow::new(fl!("distro-docs"), osrel.documentation_url.clone()),
            InfoRow::new(fl!("distro-support"), osrel.support_url.clone()),
            InfoRow::new(fl!("distro-bugtracker"), osrel.bug_report_url.clone()),
            InfoRow::new(
                fl!("distro-privacy-policy"),
                osrel.privacy_policy_url.clone(),
            ),
            InfoRow::new(fl!("distro-logo"), osrel.logo.clone()),
            InfoRow::new(fl!("distro-def-host"), osrel.default_hostname.clone()),
            InfoRow::new(fl!("distro-sysext-lvl"), osrel.sysext_level.clone()),
        ];

        os_data = os_data.push(container(kv_info_table(rows)).style(container::rounded_box));
        container(scrollable(os_data).spacing(5).id(Id::new(Self::page_id()))).into()
    }
}

impl<'a> PageView<'a> for OsRelPage {
    fn page_id() -> &'static str {
        "osrel"
    }

    fn page_title() -> String {
        fl!("page-distro")
    }

    fn page_group() -> super::GroupVariant {
        GroupVariant::Admin
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.osrel {
            LoadState::Loaded(osrel) => self.distro_table(&osrel),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetOsRelData),
            LoadState::Loading => super::loading_page(),
        }
    }
}

impl PageData for OsRelPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { OsRelease::new().to_load_state() },
            DataReceiver::OsRelDataReceived,
        )
    }
}

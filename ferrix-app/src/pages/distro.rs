/* distro.rs
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

//! Page with information about installed Linux distro

use crate::{
    Message, fl, log,
    log::log_path,
    messages::DataReceiverMessage,
    widgets::table::{InfoRow, kv_info_table},
};
use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::sys::OsRelease;

use iced::{
    Element, Task,
    widget::{Id, column, container, scrollable},
};

#[derive(Debug, Clone)]
pub struct OsRelPage<'a> {
    osrel: &'a LoadState<OsRelease>,
}

impl<'a> OsRelPage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "osrel";

    pub fn new(osrel: &'a LoadState<OsRelease>) -> Self {
        Self { osrel }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let osrel = OsRelease::new().to_load_state();
                if let LoadState::Loaded(os) = &osrel {
                    heh(&os.name);
                }
                osrel
            },
            DataReceiverMessage::OsReleaseDataReceived,
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.osrel {
            LoadState::Loaded(osrel) => self.distro_table(osrel),
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }

    fn distro_table(&self, osrel: &OsRelease) -> Element<'a, Message> {
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
        container(
            scrollable(os_data)
                .spacing(5)
                .id(Id::new(super::Page::Distro.page_id())),
        )
        .into()
    }
}

fn heh(os_name: &str) {
    let os_name = os_name.to_lowercase();
    if os_name.contains("astra") {
        log!(
            log_path(),
            "Astra Linux? И где вы только берёте эту гадость..."
        );
    } else if os_name.contains("arch") {
        log!(log_path(), "Странно, что этот ваш Archlinux всё ещё жив");
    } else if os_name.contains("fedora") {
        log!(log_path(), "Ммм, моя любимая система");
    }
}

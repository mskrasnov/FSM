/* passport.rs
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

//! System Passport page

use ferrix_lib::{battery, utils::Size};
use iced::{
    Alignment::Center,
    Element, Task,
    widget::{Id, column, container, row, rule, scrollable, space, text},
};

use super::{PageData, PageView};
use crate::{
    Ferrix, fl,
    message::Message,
    widgets::table::{InfoRow, kv_info_table},
};

/*                                NOTE                                       */
/* "Virtual" page without standard view method (Passport::view throws a panic
 * if used in code). This page doesn't contain its own data; instead, it pulls
 * data from other pages registered in ::ferrix::Ferrix.
 */
#[derive(Debug, Clone)]
pub struct Passport;

impl Passport {
    pub fn new() -> Self {
        Self
    }
}

impl<'a> PageView<'a> for Passport {
    fn page_id() -> &'static str {
        "passport"
    }

    fn page_title() -> String {
        fl!("page-dashboard")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::General
    }

    /* NOTE: We can't retrieve data from other pages using this method,
     * so there's no need to use it. Nevertheless, implementing this
     * type is necessary to simplify the integration of the Passport
     * page into the FSM UI. */
    fn page_contents_view(&'a self) -> Element<'a, Message> {
        panic!("Instead of this method, use the `Ferrix::system_passport_view()` method.")
    }
}

impl PageData for Passport {
    fn get_data() -> Task<crate::message::DataReceiver> {
        Task::batch([
            super::proc::ProcPage::get_data(),
            super::freq::CpuFreqPage::get_data(),
            super::mem::MemoryPage::get_data(),
            super::fs::FSPage::get_data(),
            super::battery::BatPage::get_data(),
            super::drm::DRMPage::get_data(),
        ])
    }
}

impl Ferrix {
    pub fn system_passport_view<'a>(&'a self) -> Element<'a, Message> {
        let title = column![
            row![text(Passport::page_title()).size(20), space::horizontal(),]
                .align_y(Center)
                .spacing(5),
            rule::horizontal(1),
        ]
        .spacing(2);

        column![title, self.system_passport_table()]
            .spacing(5)
            .into()
    }

    fn system_passport_table<'a>(&'a self) -> Element<'a, Message> {
        let rows = vec![
            InfoRow::new(fl!("dash-sys"), self.system_name()),
            InfoRow::new(fl!("dash-host"), None),
            InfoRow::new("Kernel version", None),
            InfoRow::new("Up/downtime", None),
            InfoRow::new("Desktop environment", None),
            InfoRow::new("Command shell", None),
            InfoRow::new(fl!("dash-proc"), self.cpu_name()),
            InfoRow::new("GPU", None),
            InfoRow::new(fl!("dash-mem"), self.memory()),
            InfoRow::new(fl!("dash-swap"), self.swap()),
            InfoRow::new(fl!("dash-root-part"), self.disk("/")),
            InfoRow::new(fl!("dash-home-part"), self.disk("/home")),
            InfoRow::new(fl!("dash-bat"), self.battery()),
            InfoRow::new(fl!("page-screen"), self.screen()),
            InfoRow::new("Current user", None),
            InfoRow::new("Locale", None),
            InfoRow::new("Load average", None),
        ];

        scrollable(container(kv_info_table(rows)).style(container::rounded_box))
            .spacing(5)
            .id(Id::new(Passport::page_id()))
            .into()
    }

    fn system_name(&self) -> Option<String> {
        None
    }

    fn format_cpu_name(&self, name: &str) -> String {
        let seps = ["w/", "with"];

        for sep in seps {
            if name.contains(sep) {
                let name = name.split(sep).next().unwrap_or(name);
                return name.to_string();
            }
        }

        name.to_string()
    }

    fn cpu_name(&self) -> Option<String> {
        let name = self.proc_page.proc_data.to_option().map(|proc| {
            let len = proc.entries.len();
            let model = &proc.entries[0].model_name;
            let name = match model {
                Some(name) => self.format_cpu_name(name),
                None => "N/A".to_string(),
            };

            format!("{name} ({len})")
        });
        let max_frequency = self.freq_page.freqs.to_option().map(|freq| {
            let freq = freq.policy[0].cpu_max_freq.unwrap_or_default() as f32 / 1_000_000.;
            format!("{freq:.3} GHz")
        });

        match (name, max_frequency) {
            (Some(name), Some(freq)) => Some(format!("{name} @ {freq}")),
            (Some(name), None) => Some(name),
            _ => None,
        }
    }

    fn disk(&self, mpoint: &str) -> Option<String> {
        self.fs_page.mounts.to_option().map(|mounts| {
            mounts
                .mounts
                .iter()
                .find(|item| &item.mount_point == mpoint)
                .map(|item| {
                    item.fstats
                        .map(|fstats| {
                            let total = fstats.total_size().round(2).unwrap();
                            let free = fstats.free_size().round(2).unwrap();

                            let total_bytes = fstats.total_bytes();
                            let free_bytes = fstats.free_bytes();
                            let free_percent =
                                (free_bytes as f32 / total_bytes as f32 * 100.) as u64;

                            format!("{free} / {total} ({free_percent}%)")
                        })
                        .unwrap_or("No data is available".to_string())
                })
                .unwrap_or("No data is available".to_string())
        })
    }

    fn memory(&self) -> Option<String> {
        self.mem_page.ram_data.to_option().map(|ram| {
            let total = ram.total.round(2).unwrap();
            let used = ram.used_ram(2).round(2).unwrap();
            let percentage = ram.usage_percentage().unwrap_or_default();

            format!("{used} / {total} ({percentage:.0}%)")
        })
    }

    fn swap(&self) -> Option<String> {
        self.mem_page.swap_data.to_option().map(|swaps| {
            let mut total = 0;
            let mut used = 0;
            for swap in &swaps.swaps {
                total += swap.size.get_bytes2().unwrap_or_default();
                used += swap.used.get_bytes2().unwrap_or_default();
            }
            let percentage = (used as f32 / total as f32 * 100.) as u64;

            format!(
                "{} / {} ({percentage}%)",
                Size::B(used).round(2).unwrap(),
                Size::B(total).round(2).unwrap(),
            )
        })
    }

    fn battery(&self) -> Option<String> {
        self.bat_page.bat_info.to_option().map(|bat| {
            bat.bats
                .get(0)
                .map(|bat| {
                    let name = &bat.model_name;
                    let name = match name {
                        Some(name) => name.to_string(),
                        None => "Unknown battery".to_string(),
                    };
                    let cap = bat.capacity.unwrap_or(0);
                    let status = match bat.status.clone().unwrap_or(battery::Status::None) {
                        battery::Status::Full => fl!("bat-status-ful"),
                        battery::Status::Discharging => fl!("bat-status-dis"),
                        battery::Status::Charging => fl!("bat-status-cha"),
                        battery::Status::NotCharging => fl!("bat-status-noc"),
                        _ => "?".to_string(),
                    };

                    format!("{name}: {cap}% ({status})")
                })
                .unwrap_or("Unknown battery".to_string())
        })
    }

    fn screen(&self) -> Option<String> {
        self.drm_page.drm.to_option().map(|drm| {
            let mut screens = String::new();
            let scr = drm
                .devices
                .iter()
                .filter(|screen| screen.enabled && !screen.is_empty_info())
                .collect::<Vec<_>>();
            for screen in scr {
                screens += screen
                    .edid
                    .clone()
                    .map(|edid| {
                        format!("{} (serial: {})\n", &edid.manufacturer, edid.serial_number)
                    })
                    .unwrap_or("".to_string())
                    .as_str();
            }
            screens.trim_end().to_string()
        })
    }
}

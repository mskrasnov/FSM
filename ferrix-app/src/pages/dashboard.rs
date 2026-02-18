/* dashboard.rs
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

//! Dashboard page

use crate::{Message, Page, ferrix::FerrixData, fl, load_state::LoadState, widgets::card::Card};
use ferrix_lib::{battery::Status, utils::Size};
use iced::widget::{Id, column, container, grid, progress_bar, scrollable, text};

#[derive(Debug, Clone, Copy)]
struct SwapUsage<'a> {
    name: &'a str,
    size: Size,
    used: Size,
    size_b: f32,
    used_b: f32,
}

pub fn dashboard<'a>(fx: &'a FerrixData) -> container::Container<'a, Message> {
    let (proc_name, proc_threads) = {
        match fx.proc_data.to_option() {
            Some(proc) => {
                let model = &proc.entries[0].model_name;
                let vendor = match model {
                    Some(model) => model,
                    None => "N/A",
                };
                let threads = proc.entries.len();

                (vendor, threads)
            }
            None => ("N/A", 0),
        }
    };
    let (total_ram, avail_ram) = {
        match fx.ram_data.to_option() {
            Some(ram) => (
                ram.total.round(2).unwrap_or(Size::None),
                ram.available
                    .round(2)
                    .unwrap_or(ferrix_lib::utils::Size::None),
            ),
            None => (Size::None, Size::None),
        }
    };
    let total_ram_bytes = total_ram.get_bytes2().unwrap_or(0) as f32;
    let avail_ram_bytes = avail_ram.get_bytes2().unwrap_or(0) as f32;

    let used_ram_bytes = total_ram_bytes - avail_ram_bytes;
    let used_ram = Size::B(used_ram_bytes as u64)
        .round(2)
        .unwrap_or(Size::B(used_ram_bytes as u64));

    let swaps_usage = match fx.swap_data.to_option() {
        Some(swaps) => {
            let mut usage = Vec::with_capacity(swaps.swaps.len());
            for swap in &swaps.swaps {
                let size_b = swap.size.get_bytes2().unwrap_or(0) as f32;
                let used_b = swap.used.get_bytes2().unwrap_or(0) as f32;
                usage.push(SwapUsage {
                    name: &swap.filename,
                    size: swap.size,
                    used: swap.used,
                    size_b,
                    used_b,
                });
            }
            usage
        }
        None => vec![],
    };

    let os_name = {
        match fx.osrel_data.to_option() {
            Some(osr) => match &osr.pretty_name {
                Some(pname) => pname,
                None => &osr.name,
            },
            None => "Generic Linux",
        }
    };
    let hostname = match fx.system.to_option() {
        Some(system) => match &system.hostname {
            Some(hostname) => hostname as &str,
            None => "Unknown hostname",
        },
        None => "Unknown hostname",
    };
    let uptime = match fx.system.to_option() {
        Some(system) => match &system.uptime {
            Some(uptime) => super::system::string_time(uptime.0),
            None => "Unknown".to_string(),
        },
        None => "Unknown".to_string(),
    };
    let lavg = match fx.system.to_option() {
        Some(system) => match &system.loadavg {
            Some(lavg) => super::system::string_loadavg(lavg),
            None => "Unknown".to_string(),
        },
        None => "Unknown".to_string(),
    };
    let locale = match fx.system.to_option() {
        Some(system) => match &system.language {
            Some(lang) => lang as &str,
            None => "Unknown",
        },
        None => "Unknown",
    };
    let de = match fx.system.to_option() {
        Some(system) => match &system.desktop {
            Some(de) => de as &str,
            None => "Unknown desktop",
        },
        None => "Unknown desktop",
    };
    let (prev_stat, cur_stat) = (fx.prev_proc_stat.to_option(), fx.curr_proc_stat.to_option());
    let cpu_usage = if prev_stat.is_none() || cur_stat.is_none() {
        0.0
    } else {
        let prev_stat = prev_stat.unwrap();
        let cur_stat = cur_stat.unwrap();

        match &cur_stat.cpu {
            Some(cpu) => cpu.usage_percentage(prev_stat.cpu),
            None => 0.0,
        }
    };

    let battery = match fx.bat_data.to_option() {
        Some(bat) => {
            let mut bats = Vec::with_capacity(bat.bats.len());
            for b in &bat.bats {
                let status = match &b.status {
                    Some(status) => match status {
                        Status::Charging => "⚡️",
                        Status::Discharging => "🔋️",
                        Status::NotCharging => "🚫️",
                        Status::Full => "🟢️",
                        _ => "❔️",
                    },
                    None => "❔️",
                };

                let name = match &b.name {
                    Some(name) => format!("{} {}", status, name),
                    None => fl!("dash-unk-bat"),
                };
                bats.push((name, b.capacity.unwrap_or(0)));
            }
            bats
        }
        None => vec![],
    };

    let mut items = vec![
        Card::new(fl!("dash-proc"), Message::SelectPage(Page::Processors)).widget(text(fl!(
            "dash-proc-info",
            name = proc_name,
            threads = proc_threads
        ))),
        Card::new(
            fl!("dash-proc-usage"),
            Message::SelectPage(Page::SystemMonitor),
        )
        .widget(
            column![
                text(fl!(
                    "dash-proc-usg_label",
                    usage = format!("{cpu_usage:.2}")
                )),
                progress_bar(0.0..=100., cpu_usage),
            ]
            .spacing(5),
        ),
        Card::new(fl!("dash-mem"), Message::SelectPage(Page::Memory)).widget(
            column![
                column![
                    text(fl!("dash-mem-used", used = used_ram.to_string())),
                    text(fl!("dash-mem-total", total = total_ram.to_string())),
                ],
                progress_bar(0.0..=total_ram_bytes, used_ram_bytes),
            ]
            .spacing(5),
        ),
    ];

    /* 0 - CPU,
     * 1 - CPU Usage,
     * 2 - RAM Usage,
     * 3...n - Swap usage
     */
    let mut offset = 3;
    for swap in swaps_usage {
        items.insert(
            offset,
            Card::new(fl!("dash-swap"), Message::SelectPage(Page::Memory)).widget(
                column![
                    column![
                        text(swap.name),
                        text(fl!(
                            "dash-mem-used",
                            used = swap.used.round(2).unwrap_or_default().to_string()
                        )),
                        text(fl!(
                            "dash-mem-total",
                            total = swap.size.round(2).unwrap_or_default().to_string()
                        )),
                    ],
                    progress_bar(0.0..=swap.size_b, swap.used_b),
                ]
                .spacing(5),
            ),
        );
        offset += 1;
    }

    for bat in battery {
        items.push(
            Card::new(fl!("dash-bat"), Message::SelectPage(Page::Battery)).widget(
                column![
                    text(format!("{}: {}%", bat.0, bat.1)),
                    progress_bar(0.0..=100., bat.1 as f32)
                ]
                .spacing(5),
            ),
        );
    }

    if let LoadState::Loaded(storages) = &fx.storages {
        let storages = &storages.mounts;
        for storage in storages {
            if &storage.mount_point == "/" || &storage.mount_point == "/home" {
                let (usage_percent, used, total) = match &storage.fstats {
                    Some(fstats) => (
                        fstats.usage_percent() as f32,
                        fstats.used_size().round(2).unwrap_or_default(),
                        fstats.total_size().round(2).unwrap_or_default(),
                    ),
                    None => (0., Size::None, Size::None),
                };

                items.push(
                    Card::new(
                        match &storage.mount_point as &str {
                            "/" => fl!("dash-root-part"),
                            "/home" => fl!("dash-home-part"),
                            _ => fl!("dash-unk-part"),
                        },
                        Message::SelectPage(Page::FileSystems),
                    )
                    .widget(
                        column![
                            column![
                                text(fl!("dash-mem-used", used = used.to_string())),
                                text(fl!("dash-mem-total", total = total.to_string()))
                            ],
                            progress_bar(0.0..=100., usage_percent),
                        ]
                        .spacing(5),
                    ),
                );
            }
        }
    }

    let cards = [
        Card::new(fl!("dash-sys"), Message::SelectPage(Page::Distro)).widget(text(os_name)),
        Card::new(fl!("dash-host"), Message::SelectPage(Page::SystemMisc)).widget(text(hostname)),
        Card::new(fl!("misc-de"), Message::SelectPage(Page::SystemMisc)).widget(text(de)),
        Card::new(fl!("misc-uptime"), Message::SelectPage(Page::SystemMisc)).widget(text(uptime)),
        Card::new(fl!("misc-loadavg"), Message::SelectPage(Page::SystemMisc)).widget(text(lavg)),
        Card::new(fl!("misc-lang"), Message::SelectPage(Page::SystemMisc)).widget(text(locale)),
    ];
    for card in cards {
        items.push(card);
    }
    items.shrink_to_fit();

    let mut gr = grid([]).spacing(5).fluid(185.);
    for item in items {
        gr = gr.push(item);
    }

    container(
        scrollable(gr)
            .spacing(5)
            .id(Id::new(super::Page::Dashboard.page_id())),
    )
}

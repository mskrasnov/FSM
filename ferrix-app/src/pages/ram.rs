/* ram.rs
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

//! RAM page

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};
use ferrix_lib::ram::{RAM, Swaps};
use ferrix_widgets::headers::header;
use iced::widget::{column, container, scrollable, text};

pub fn ram_page<'a>(
    ram: &'a DataLoadingState<RAM>,
    swaps: &'a DataLoadingState<Swaps>,
) -> container::Container<'a, Message> {
    container(
        scrollable(column![ram_table(ram), swap_table(swaps),].spacing(5))
            .spacing(5)
            .id(super::Page::Memory.page_id()),
    )
}

fn ram_table<'a>(ram: &'a DataLoadingState<RAM>) -> container::Container<'a, Message> {
    match ram {
        DataLoadingState::Loaded(ram) => {
            let mut ram_data = column![header(fl!("ram-hdr"))].spacing(5);
            let rows = vec![
                InfoRow::new(fl!("ram-total"), fmt_val(ram.total.round(2))),
                InfoRow::new(fl!("ram-free"), fmt_val(ram.free.round(2))),
                InfoRow::new(fl!("ram-available"), fmt_val(ram.available.round(2))),
                InfoRow::new(fl!("ram-buffers"), fmt_val(ram.buffers.round(2))),
                InfoRow::new(fl!("ram-cached"), fmt_val(ram.cached.round(2))),
                InfoRow::new(fl!("ram-swap-cached"), fmt_val(ram.swap_cached.round(2))),
                InfoRow::new(fl!("ram-active"), fmt_val(ram.active.round(2))),
                InfoRow::new(fl!("ram-inactive"), fmt_val(ram.inactive.round(2))),
                InfoRow::new(fl!("ram-active-anon"), fmt_val(ram.active_anon.round(2))),
                InfoRow::new(
                    fl!("ram-inactive-anon"),
                    fmt_val(ram.inactive_anon.round(2)),
                ),
                InfoRow::new(fl!("ram-active-file"), fmt_val(ram.active_file.round(2))),
                InfoRow::new(
                    fl!("ram-inactive-file"),
                    fmt_val(ram.inactive_file.round(2)),
                ),
                InfoRow::new(fl!("ram-unevictable"), fmt_val(ram.unevictable.round(2))),
                InfoRow::new(fl!("ram-locked"), fmt_val(ram.mlocked.round(2))),
                InfoRow::new(fl!("ram-swap-total"), fmt_val(ram.swap_total.round(2))),
                InfoRow::new(fl!("ram-swap-free"), fmt_val(ram.swap_free.round(2))),
                InfoRow::new(fl!("ram-zswap"), fmt_val(ram.zswap.round(2))),
                InfoRow::new(fl!("ram-zswapped"), fmt_val(ram.zswapped.round(2))),
                InfoRow::new(fl!("ram-dirty"), fmt_val(ram.dirty.round(2))),
                InfoRow::new(fl!("ram-writeback"), fmt_val(ram.writeback.round(2))),
                InfoRow::new(fl!("ram-anon-pages"), fmt_val(ram.anon_pages.round(2))),
                InfoRow::new(fl!("ram-mapped"), fmt_val(ram.mapped.round(2))),
                InfoRow::new(fl!("ram-shmem"), fmt_val(ram.shmem.round(2))),
                InfoRow::new(fl!("ram-kreclaimable"), fmt_val(ram.kreclaimable.round(2))),
                InfoRow::new(fl!("ram-slab"), fmt_val(ram.slab.round(2))),
                InfoRow::new(fl!("ram-sreclaimable"), fmt_val(ram.sreclaimable.round(2))),
                InfoRow::new(fl!("ram-sunreclaim"), fmt_val(ram.sunreclaim.round(2))),
                InfoRow::new(fl!("ram-kernel-stack"), fmt_val(ram.kernel_stack.round(2))),
                InfoRow::new(fl!("ram-page-tables"), fmt_val(ram.page_tables.round(2))),
                InfoRow::new(
                    fl!("ram-sec-page-tables"),
                    fmt_val(ram.sec_page_tables.round(2)),
                ),
                InfoRow::new(fl!("ram-nfs-unstable"), fmt_val(ram.nfs_unstable.round(2))),
                InfoRow::new(fl!("ram-bounce"), fmt_val(ram.bounce.round(2))),
                InfoRow::new(
                    fl!("ram-writeback-tmp"),
                    fmt_val(ram.writeback_tmp.round(2)),
                ),
                InfoRow::new(fl!("ram-commit-limit"), fmt_val(ram.commit_limit.round(2))),
            ];

            ram_data = ram_data.push(container(kv_info_table(rows)).style(container::rounded_box));
            container(ram_data)
        }
        DataLoadingState::Error(why) => {
            container(text(format!("Failed to get RAM data!\n{why}")).style(text::danger))
        }
        DataLoadingState::Loading => container(text("Loading data...").style(text::warning)),
    }
}

fn swap_table<'a>(swaps: &'a DataLoadingState<Swaps>) -> container::Container<'a, Message> {
    match swaps {
        DataLoadingState::Loaded(swaps) => {
            let mut swap_data = column![header(fl!("ram-swp-hdr"))].spacing(5);

            if swaps.swaps.is_empty() {
                swap_data = swap_data.push(text(fl!("ram-swp-not-found")).style(text::danger));
                return container(swap_data);
            }

            for swap in &swaps.swaps {
                let rows = vec![
                    InfoRow::new(fl!("ram-swp-size"), fmt_val(swap.size.round(2))),
                    InfoRow::new(fl!("ram-swp-used"), fmt_val(swap.used.round(2))),
                    InfoRow::new(fl!("ram-swp-prior"), fmt_val(Some(swap.priority))),
                ];
                swap_data = swap_data.push(
                    column![
                        text(fl!("ram-swp", name = swap.filename.to_string())).style(text::warning),
                        container(kv_info_table(rows)).style(container::rounded_box),
                    ]
                    .spacing(5),
                );
            }

            container(swap_data)
        }
        DataLoadingState::Error(why) => {
            container(text(format!("Failed to get RAM data!\n{why}")).style(text::danger))
        }
        DataLoadingState::Loading => container(text("Loading data...").style(text::warning)),
    }
}

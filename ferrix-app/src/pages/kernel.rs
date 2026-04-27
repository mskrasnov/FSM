/* kernel.rs
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

//! Kernel page

use crate::{
    Message, fl,
    load_state::DataLoadingState,
    widgets::table::{InfoRow, fmt_val, hdr_name, kv_info_table, text_fmt_val},
};
use ferrix_data::kmods::KResult;
use ferrix_lib::sys::{KModules, Kernel, Module};

use iced::{
    Length,
    widget::{Id, center, container, scrollable, table, text},
};

pub fn kernel_page<'a>(
    kernel_data: &'a DataLoadingState<Kernel>,
) -> container::Container<'a, Message> {
    match kernel_data {
        DataLoadingState::Loaded(kern) => {
            let rows = vec![
                InfoRow::new(fl!("kernel-summary"), kern.uname.clone()),
                InfoRow::new(fl!("kernel-cmdline"), kern.cmdline.clone()),
                InfoRow::new(fl!("kernel-arch"), kern.arch.clone()),
                InfoRow::new(fl!("kernel-version"), kern.version.clone()),
                InfoRow::new(fl!("kernel-build"), kern.build_info.clone()),
                InfoRow::new(fl!("kernel-pid-max"), fmt_val(Some(kern.pid_max))),
                InfoRow::new(fl!("kernel-threads-max"), fmt_val(Some(kern.threads_max))),
                InfoRow::new(fl!("kernel-user-evs"), fmt_val(kern.user_events_max)),
                InfoRow::new(fl!("kernel-avail-enthropy"), fmt_val(kern.enthropy_avail)),
            ];

            container(
                scrollable(container(kv_info_table(rows)).style(container::rounded_box))
                    .spacing(5)
                    .id(Id::new(super::Page::Kernel.page_id())),
            )
        }
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

pub fn kmods_page<'a>(kmods: &'a DataLoadingState<KResult>) -> container::Container<'a, Message> {
    match kmods {
        DataLoadingState::Loaded(kmods) => match kmods {
            KResult::Ok { data } => kmods_view(data),
            KResult::Err { error } => super::error_page(error),
        },
        DataLoadingState::Error(why) => super::error_page(why),
        DataLoadingState::Loading => super::loading_page(),
    }
}

fn kmods_view<'a>(kmods: &'a KModules) -> container::Container<'a, Message> {
    if kmods.modules.is_empty() {
        container(center(
            text(fl!("kernel-mods-is-empty"))
                .size(16)
                .style(text::secondary),
        ))
    } else {
        let table = container(modules_table(&kmods.modules)).style(container::rounded_box);
        container(
            scrollable(table)
                .spacing(5)
                .id(Id::new(super::Page::KModules.page_id())),
        )
    }
}

fn modules_table<'a>(rows: &'a [Module]) -> table::Table<'a, Message> {
    let columns = [
        table::column(hdr_name(fl!("kmod-name")), |row: &'a Module| {
            text(&row.name).wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(1)),
        table::column(hdr_name(fl!("kmod-size")), |row: &'a Module| {
            text_fmt_val(row.size.round(2))
        }),
        table::column(hdr_name(fl!("kmod-instances")), |row: &'a Module| {
            text(row.instances)
        }),
        table::column(hdr_name(fl!("kmod-depends")), |row: &'a Module| {
            text(if &row.dependencies == "-" {
                ""
            } else {
                &row.dependencies
            })
            .wrapping(text::Wrapping::WordOrGlyph)
        })
        .width(Length::FillPortion(3)),
        table::column(hdr_name(fl!("kmod-state")), |row: &'a Module| {
            text(&row.state).style(if &row.state == "Live" {
                text::success
            } else {
                text::default
            })
        }),
        table::column(hdr_name(fl!("kmod-addrs")), |row: &'a Module| {
            text(&row.memory_addrs)
        }),
    ];

    table(columns, rows).padding(2).width(Length::Fill)
}

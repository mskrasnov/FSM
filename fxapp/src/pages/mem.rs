/* mem.rs
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

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::ram::{RAM, Swaps};
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Element, Length, Task,
    widget::{Column, button, column, container, text},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, Message, PageMessage},
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};

#[derive(Debug, Clone)]
pub struct MemoryPage {
    pub ram_data: LoadState<RAM>,
    pub swap_data: LoadState<Swaps>,
    pub selected_data: MemoryData,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MemoryData {
    Summary,
    Swaps,
}

impl MemoryPage {
    pub fn new() -> Self {
        Self {
            ram_data: LoadState::Loading,
            swap_data: LoadState::Loading,
            selected_data: MemoryData::Summary,
        }
    }

    fn ram_table<'a>(&'a self, ram: &'a RAM) -> Element<'a, Message> {
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

        container(kv_info_table(rows))
            .style(container::rounded_box)
            .into()
    }

    fn swap_table<'a>(&self, swaps: &'a LoadState<Swaps>) -> Element<'a, Message> {
        match swaps {
            LoadState::Loaded(swaps) => {
                let mut swap_data = column![].spacing(5);
                if swaps.swaps.is_empty() {
                    swap_data = swap_data.push(text(fl!("ram-swp-not-found")).style(text::danger));
                    return swap_data.into();
                }

                for swap in &swaps.swaps {
                    let rows = vec![
                        InfoRow::new(fl!("ram-swp-size"), fmt_val(swap.size.round(2))),
                        InfoRow::new(fl!("ram-swp-used"), fmt_val(swap.used.round(2))),
                        InfoRow::new(fl!("ram-swp-prior"), fmt_val(Some(swap.priority))),
                    ];
                    swap_data = swap_data.push(
                        column![
                            text(fl!("ram-swp", name = swap.filename.to_string()))
                                .style(text::warning),
                            container(kv_info_table(rows)).style(container::rounded_box),
                        ]
                        .spacing(5),
                    );
                }

                swap_data.into()
            }
            LoadState::Loading => text("Swaps: loading data...").style(text::warning).into(),
            LoadState::Error(why) => text(format!("Swaps: loading error: {why}"))
                .style(text::danger)
                .into(),
        }
    }

    fn data_selector<'a>(&'a self) -> Vec<Element<'a, Message>> {
        let buttons = vec![
            button(text(fl!("ram-hdr")))
                .on_press(Message::PageMessage(PageMessage::MemPage(
                    MemoryPageMessage::DataSelected(MemoryData::Summary),
                )))
                .style(if MemoryData::Summary == self.selected_data {
                    button::subtle
                } else {
                    button::text
                })
                .height(Length::Fill)
                .padding(2)
                .into(),
            button(text(fl!("ram-swp-hdr")))
                .on_press(Message::PageMessage(PageMessage::MemPage(
                    MemoryPageMessage::DataSelected(MemoryData::Swaps),
                )))
                .style(if MemoryData::Swaps == self.selected_data {
                    button::subtle
                } else {
                    button::text
                })
                .height(Length::Fill)
                .padding(2)
                .into(),
        ];
        buttons
    }

    fn memory_view<'a>(
        &'a self,
        ram: &'a RAM,
        swaps: &'a LoadState<Swaps>,
    ) -> Element<'a, Message> {
        let first_panel = container(Column::from_vec(self.data_selector()))
            .style(container::rounded_box)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(2);
        let second_panel = container(match self.selected_data {
            MemoryData::Summary => self.ram_table(ram),
            MemoryData::Swaps => self.swap_table(swaps),
        });

        let view = SeparatedView::new(first_panel, second_panel)
            .set_fpane_id("aa")
            .set_spane_id(Self::page_id())
            .set_fpane_max_height(Length::Shrink)
            .set_spane_max_height(Length::Fill);
        view.view().into()
    }
}

impl<'a> PageView<'a> for MemoryPage {
    fn page_id() -> &'static str {
        "mem"
    }

    fn page_title() -> String {
        fl!("page-memory")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_contents_view(&'a self) -> iced::Element<'a, Message> {
        match &self.ram_data {
            LoadState::Loaded(ram) => self.memory_view(ram, &self.swap_data),
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetRAMData),
        }
    }
}

impl PageData for MemoryPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { (RAM::new().to_load_state(), Swaps::new().to_load_state()) },
            DataReceiver::RAMDataReceived,
        )
    }
}

#[derive(Debug, Clone)]
pub enum MemoryPageMessage {
    DataSelected(MemoryData),
}

impl MemoryPageMessage {
    pub fn update<'a>(self, page: &'a mut MemoryPage) -> Task<Message> {
        match self {
            Self::DataSelected(data) => {
                page.selected_data = data;
                Task::none()
            }
        }
    }
}

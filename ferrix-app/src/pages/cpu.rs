/* cpu.rs
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

//! CPU page

use crate::{
    fl,
    load_state::LoadState,
    messages::{ButtonsMessage, DataReceiverMessage, Message},
    widgets::table::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};
use ferrix_data::load_state::ToLoadState;
use ferrix_lib::cpu::Processors;
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Element, Length, Task,
    widget::{Column, button, column, container, text},
};

#[derive(Debug, Clone)]
pub struct ProcPage<'a> {
    pub processors: &'a LoadState<Processors>,
    pub id: usize,
}

impl<'a> ProcPage<'a> {
    pub const IS_SPECIAL: bool = false;

    pub fn new(processors: &'a LoadState<Processors>, id: usize) -> Self {
        Self { processors, id }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let proc = Processors::new();
                proc.to_load_state()
            },
            |val| DataReceiverMessage::CPUDataReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.processors {
            LoadState::Loaded(proc) => {
                let proc_names = get_proc_names(proc);
                let proc_list = {
                    let mut elements = Vec::with_capacity(proc.entries.len());
                    for p in proc_names {
                        let b = button(text(p.1))
                            .on_press(Message::Buttons(ButtonsMessage::ProcessorSelected(p.0)))
                            .style(if p.0 == self.id {
                                button::subtle
                            } else {
                                button::text
                            })
                            .height(Length::Fill)
                            .padding(2)
                            .into();
                        elements.push(b);
                    }
                    elements
                };
                let first_panel = container(
                    column![
                        text(fl!("page-procs")).style(text::secondary),
                        Column::from_vec(proc_list),
                    ]
                    .spacing(5),
                )
                .style(container::rounded_box)
                .width(Length::Fill)
                .padding(2);
                let second_panel = proc_info(proc, self.id).style(container::rounded_box);

                let view = SeparatedView::new(first_panel, second_panel)
                    .set_fpane_id(super::Page::Processors.scrolled_list_id().unwrap_or(""))
                    .set_spane_id(super::Page::Processors.page_id());
                container(view.view()).into()
            }
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }
}

#[deprecated]
pub fn proc_page<'a>(
    processors: &'a LoadState<Processors>,
    id: usize,
) -> container::Container<'a, Message> {
    match processors {
        LoadState::Loaded(proc) => {
            let proc_names = get_proc_names(proc);
            let proc_list = {
                let mut elements = Vec::with_capacity(proc.entries.len());
                for p in proc_names {
                    let b = button(text(p.1))
                        .on_press(Message::Buttons(ButtonsMessage::ProcessorSelected(p.0)))
                        .style(if p.0 == id {
                            button::subtle
                        } else {
                            button::text
                        })
                        .height(Length::Fill)
                        .padding(2)
                        .into();
                    elements.push(b);
                }
                elements
            };
            let first_panel = container(
                column![
                    text(fl!("page-procs")).style(text::secondary),
                    Column::from_vec(proc_list),
                ]
                .spacing(5),
            )
            .style(container::rounded_box)
            .width(Length::Fill)
            .padding(2);
            let second_panel = proc_info(proc, id).style(container::rounded_box);

            let view = SeparatedView::new(first_panel, second_panel)
                .set_fpane_id(super::Page::Processors.scrolled_list_id().unwrap_or(""))
                .set_spane_id(super::Page::Processors.page_id());
            container(view.view())
        }
        LoadState::Error(why) => super::error_page(why),
        LoadState::Loading => super::loading_page(),
    }
}

fn get_proc_names<'a>(proc: &'a Processors) -> Vec<(usize, String)> {
    let mut i = 0;
    let mut v = Vec::with_capacity(proc.entries.len());

    for p in &proc.entries {
        v.push((
            i,
            match &p.model_name {
                Some(m) => format!("#{i}: {m}"),
                None => format!("#{i}: Unknown processor"),
            },
        ));
        i += 1;
    }
    v
}

#[cfg(not(target_arch = "aarch64"))]
fn proc_info<'a>(proc: &'a Processors, id: usize) -> container::Container<'a, Message> {
    let proc = &proc.entries[id];
    let rows = vec![
        InfoRow::new(fl!("cpu-model"), proc.model_name.clone()),
        InfoRow::new(fl!("cpu-vendor"), proc.vendor_id.clone()),
        InfoRow::new(fl!("cpu-physical-id"), fmt_val(proc.physical_id)),
        InfoRow::new(fl!("cpu-core-id"), fmt_val(proc.core_id)),
        InfoRow::new(fl!("cpu-apicid"), fmt_val(proc.apicid)),
        InfoRow::new(fl!("cpu-iapicid"), fmt_val(proc.initial_apicid)),
        InfoRow::new(fl!("cpu-family"), fmt_val(proc.cpu_family)),
        InfoRow::new(fl!("cpu-stepping"), fmt_val(proc.stepping)),
        InfoRow::new(fl!("cpu-microcode"), proc.microcode.clone()),
        InfoRow::new(fl!("cpu-freq"), Some(fl!("cpu-see-freq"))),
        InfoRow::new(fl!("cpu-cache"), fmt_val(proc.cache_size)),
        InfoRow::new(fl!("cpu-siblings"), fmt_val(proc.siblings)),
        InfoRow::new(fl!("cpu-cpu-cores"), fmt_val(proc.cpu_cores)),
        InfoRow::new(fl!("cpu-fpu"), fmt_bool(proc.fpu)),
        InfoRow::new(fl!("cpu-fpu-e"), fmt_bool(proc.fpu_exception)),
        InfoRow::new(fl!("cpu-cpuid-lvl"), fmt_val(proc.cpuid_level)),
        InfoRow::new(fl!("cpu-wp"), fmt_bool(proc.wp)),
        InfoRow::new(fl!("cpu-flags"), fmt_vec(&proc.flags)),
        InfoRow::new(fl!("cpu-bugs"), fmt_vec(&proc.bugs)),
        InfoRow::new(fl!("cpu-bogomips"), fmt_val(proc.bogomips)),
        InfoRow::new(fl!("cpu-clflush"), fmt_val(proc.clflush_size)),
        InfoRow::new(fl!("cpu-cache-align"), fmt_val(proc.cache_alignment)),
        InfoRow::new(fl!("cpu-address-size"), proc.address_sizes.clone()),
        InfoRow::new(fl!("cpu-power"), proc.power_management.clone()),
    ];
    container(kv_info_table(rows))
}

#[cfg(target_arch = "aarch64")]
fn proc_info<'a>(proc: &'a Processors, id: usize) -> container::Container<'a, Message> {
    let proc = &proc.entries[id];
    let rows = vec![
        InfoRow::new(fl!("cpu-impl"), proc.cpu_implementer.clone()),
        InfoRow::new(fl!("cpu-arch"), fmt_val(proc.cpu_architecture)),
        InfoRow::new(fl!("cpu-var"), proc.cpu_variant.clone()),
        InfoRow::new(fl!("cpu-part"), proc.cpu_part.clone()),
        InfoRow::new(fl!("cpu-rev"), fmt_val(proc.cpu_revision)),
    ];
    container(kv_info_table(rows))
}

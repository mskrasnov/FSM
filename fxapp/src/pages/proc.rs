/* proc.rs
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
use ferrix_lib::cpu::Processors;
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Length, Task,
    widget::{Column, button, column, container, text},
};

use crate::{
    fl,
    message::{DataReceiver, Message, PageMessage},
    widgets::table::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};

use super::{PageData, PageView};

#[derive(Debug, Clone)]
pub struct ProcPage {
    pub proc_data: LoadState<Processors>,
    pub id: usize,
}

impl ProcPage {
    pub fn new() -> Self {
        Self {
            proc_data: LoadState::Loading,
            id: 0,
        }
    }

    fn get_proc_list<'a>(
        &'a self,
        proc: &'a Processors,
        names: Vec<(usize, String)>,
    ) -> Vec<iced::Element<'a, Message>> {
        let mut elements = Vec::with_capacity(proc.entries.len());
        for p in names {
            let b = button(text(p.1))
                .on_press(Message::PageMessage(PageMessage::ProcPage(
                    ProcPageMessage::ProcSelected(p.0),
                )))
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
    }

    fn loaded_view<'a>(&'a self, proc: &'a Processors) -> iced::Element<'a, Message> {
        let proc_names = get_proc_names(proc);
        let proc_list = self.get_proc_list(proc, proc_names);
        let first_panel = container(
            column![
                text(fl!("page-procs")).style(text::secondary),
                Column::from_vec(proc_list),
            ]
            .spacing(5),
        )
        .style(container::rounded_box)
        .width(Length::Fill)
        .height(Length::Shrink)
        .padding(2);
        let second_panel = container(self.proc_info(proc, self.id)).style(container::rounded_box);

        let view = SeparatedView::new(first_panel, second_panel)
            .set_fpane_id(Self::scrolled_page_id().unwrap_or(""))
            .set_spane_id(Self::page_id())
            .set_fpane_max_height(Length::Fixed(170.))
            .set_spane_max_height(Length::Fill);

        view.view().into()
    }

    #[cfg(not(target_arch = "aarch64"))]
    fn proc_info<'a>(&'a self, proc: &'a Processors, id: usize) -> iced::Element<'a, Message> {
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
        kv_info_table(rows).into()
    }

    #[cfg(target_arch = "aarch64")]
    fn proc_info<'a>(&'a self, proc: &'a Processors, id: usize) -> iced::Element<'a, Message> {
        let proc = &proc.entries[id];
        let rows = vec![
            InfoRow::new(fl!("cpu-impl"), proc.cpu_implementer.clone()),
            InfoRow::new(fl!("cpu-arch"), fmt_val(proc.cpu_architecture)),
            InfoRow::new(fl!("cpu-var"), proc.cpu_variant.clone()),
            InfoRow::new(fl!("cpu-part"), proc.cpu_part.clone()),
            InfoRow::new(fl!("cpu-rev"), fmt_val(proc.cpu_revision)),
        ];
        kv_info_table(rows).into()
    }
}

impl<'a> PageView<'a> for ProcPage {
    fn page_id() -> &'static str {
        "proc"
    }

    fn scrolled_page_id() -> Option<&'static str> {
        Some("proc-scrolled")
    }

    fn page_title() -> String {
        fl!("page-procs")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_contents_view(&'a self) -> iced::Element<'a, Message> {
        match &self.proc_data {
            LoadState::Loaded(proc) => self.loaded_view(proc),
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetProcData),
        }
    }
}

impl PageData for ProcPage {
    fn get_data() -> iced::Task<DataReceiver> {
        Task::perform(
            async move { Processors::new().to_load_state() },
            DataReceiver::ProcDataReceived,
        )
    }
}

#[derive(Debug, Clone)]
pub enum ProcPageMessage {
    ProcSelected(usize),
}

impl ProcPageMessage {
    pub fn update<'a>(self, page: &'a mut ProcPage) -> Task<Message> {
        match self {
            Self::ProcSelected(id) => {
                page.id = id;
                Task::none()
            }
        }
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

/* freq.rs
 *
 * Copyright 2025, 2026 Michail Krasnov <mskrasnov07@ya.ru>
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

//! CPU Frequency page

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::cpu_freq::CpuFreq;
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Element, Length, Task,
    widget::{Column, button, column, container, text},
};

use crate::{
    fl,
    message::{DataReceiver, Message, PageMessage},
    widgets::table::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};

use super::{PageData, PageView};

#[derive(Debug, Clone)]
pub struct CpuFreqPage {
    pub freqs: LoadState<CpuFreq>,
    pub id: usize,
}

impl CpuFreqPage {
    pub fn new() -> Self {
        Self {
            freqs: LoadState::Loading,
            id: 0,
        }
    }

    fn cpu_freq_list<'a>(&'a self, cpu_freq: &'a CpuFreq) -> Element<'a, Message> {
        if cpu_freq.policy.is_empty() {
            return container(text(fl!("cpufreq-notfound")).style(text::danger)).into();
        }
        let proc_names = {
            let mut names = Vec::with_capacity(cpu_freq.policy.len());
            let mut i = 0;
            while i < cpu_freq.policy.len() {
                names.push((
                    i,
                    format!(
                        "{}: {}",
                        fl!("cpufreq-sum", cpu = i),
                        fmt_freq(*(&cpu_freq.policy[i].scaling_cur_freq))
                            .unwrap_or("N/A MHz".to_string()),
                    ),
                ));
                i += 1;
            }
            names
        };

        let freq_list = {
            let mut elements = Vec::with_capacity(cpu_freq.policy.len());
            for f in proc_names {
                let b = button(text(f.1))
                    .on_press(Message::PageMessage(PageMessage::CpuFreqMessage(
                        ProcFreqMessage::ProcSelected(f.0),
                    )))
                    .style(if f.0 == self.id {
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
                text(fl!("cpufreq-flist")).style(text::secondary),
                Column::from_vec(freq_list),
            ]
            .spacing(5),
        )
        .style(container::rounded_box)
        .width(Length::Fill)
        .padding(2);

        let second_panel = freq_view(self.id, &cpu_freq);
        let cpu_freq_view = SeparatedView::new(first_panel, second_panel)
            .set_fpane_id(Self::page_id())
            .set_spane_id(Self::scrolled_page_id().unwrap_or(""))
            .set_fpane_max_height(210.);
        cpu_freq_view.view()
    }
}

impl<'a> PageView<'a> for CpuFreqPage {
    fn page_id() -> &'static str {
        "freq"
    }

    fn scrolled_page_id() -> Option<&'static str> {
        Some("freq")
    }

    fn page_title() -> String {
        fl!("page-cpufreq")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.freqs {
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetCpuFreqData),
            LoadState::Loaded(freqs) => self.cpu_freq_list(freqs),
        }
    }
}

fn freq_view<'a>(id: usize, freq: &'a CpuFreq) -> Element<'a, Message> {
    let policy = &freq.policy[id];
    let rows = vec![
        InfoRow::new(fl!("cpufreq-bios-limit"), fmt_freq(policy.bios_limit)),
        InfoRow::new(fl!("cpufreq-cpb"), fmt_bool(policy.cpb)),
        InfoRow::new(fl!("cpufreq-cpu_max_freq"), fmt_freq(policy.cpu_max_freq)),
        InfoRow::new(fl!("cpufreq-cpu_min_freq"), fmt_freq(policy.cpu_min_freq)),
        InfoRow::new(
            fl!("cpufreq-scaling_min"),
            fmt_freq(policy.scaling_min_freq),
        ),
        InfoRow::new(
            fl!("cpufreq-scaling_max"),
            fmt_freq(policy.scaling_max_freq),
        ),
        InfoRow::new(
            fl!("cpufreq-scaling_cur"),
            fmt_freq(policy.scaling_cur_freq),
        ),
        InfoRow::new(fl!("cpufreq-scaling_gov"), policy.scaling_governor.clone()),
        InfoRow::new(
            fl!("cpufreq-avail_gov"),
            fmt_vec(&policy.scaling_available_governors),
        ),
        InfoRow::new(fl!("cpufreq-scaling_drv"), policy.scaling_driver.clone()),
        InfoRow::new(
            fl!("cpufreq-avail_freq"),
            fmt_vec_freq(&policy.scaling_available_frequencies),
        ),
        InfoRow::new(
            fl!("cpufreq-trans_lat"),
            fmt_val(policy.cpuinfo_transition_latency),
        ),
        InfoRow::new(fl!("cpufreq-set_speed"), policy.scaling_setspeed.clone()),
    ];
    container(kv_info_table(rows))
        .style(container::rounded_box)
        .into()
}

fn fmt_freq(f: Option<u32>) -> Option<String> {
    f.and_then(|f| {
        let (freq, suf) = if f >= 1_000_000 {
            (f as f32 / 1_000_000., "GHz")
        } else if f >= 1_000 {
            (f as f32 / 1_000., "MHz")
        } else {
            (f as f32, "kHz")
        };
        Some(format!("{freq:.3} {suf}"))
    })
}

fn fmt_vec_freq(f: &Option<Vec<u32>>) -> Option<String> {
    f.as_ref().and_then(|f| {
        let mut s = String::new();
        for freq in f {
            s += &format!("{}; ", fmt_freq(Some(*freq)).unwrap());
        }
        Some(s.trim().strip_suffix(';').unwrap_or("").to_string())
    })
}

impl PageData for CpuFreqPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { CpuFreq::new().to_load_state() },
            DataReceiver::CpuFreqDataReceived,
        )
    }
}

#[derive(Debug, Clone)]
pub enum ProcFreqMessage {
    ProcSelected(usize),
}

impl ProcFreqMessage {
    pub fn update<'a>(self, page: &'a mut CpuFreqPage) -> Task<Message> {
        match self {
            Self::ProcSelected(id) => {
                page.id = id;
                Task::none()
            }
        }
    }
}

/* cpu_freq.rs
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

//! CPU Frequency page

use crate::{
    Message, fl,
    load_state::LoadState,
    messages::{ButtonsMessage, DataReceiverMessage},
    widgets::table::{InfoRow, fmt_bool, fmt_val, fmt_vec, kv_info_table},
};
use ferrix_data::load_state::ToLoadState;
use ferrix_lib::cpu_freq::CpuFreq;
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Element, Length, Task,
    widget::{Column, button, column, container, scrollable, text},
};

#[derive(Debug, Clone)]
pub struct ProcFreqPage<'a> {
    pub cpu_freq: &'a LoadState<CpuFreq>,
    pub id: usize,
}

impl<'a> ProcFreqPage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "cpufreq";

    pub fn new(freq: &'a LoadState<CpuFreq>, id: usize) -> Self {
        Self { cpu_freq: freq, id }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let cpu_freq = CpuFreq::new();
                cpu_freq.to_load_state()
            },
            |val| DataReceiverMessage::CPUFrequencyReceived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.cpu_freq {
            LoadState::Loaded(cpu_freq) => self.cpu_freq_page(cpu_freq).into(),
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
    }

    fn cpu_freq_page(&self, cpu_freq: &'a CpuFreq) -> container::Container<'a, Message> {
        let mut policy_list = column![].spacing(5);
        let rows = vec![InfoRow::new(
            fl!("cpufreq-tboost"),
            fmt_bool(cpu_freq.boost),
        )];
        policy_list = policy_list.push(
            column![container(kv_info_table(rows)).style(container::rounded_box),].spacing(5),
        );

        if cpu_freq.policy.is_empty() {
            policy_list = policy_list.push(text(fl!("cpufreq-notfound")).style(text::danger));
            return container(scrollable(policy_list));
        }
        policy_list = policy_list.push(container(self.cpu_freq_list(cpu_freq)));
        container(policy_list)
    }

    fn cpu_freq_list(&self, cpu_freq: &'a CpuFreq) -> Element<'a, Message> {
        if cpu_freq.policy.is_empty() {
            return container(text("cpufreq-notfound").style(text::danger)).into();
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
                    .on_press(Message::Buttons(ButtonsMessage::FrequencySelected(f.0)))
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
            .set_fpane_id(super::Page::CPUFrequency.scrolled_list_id().unwrap_or(""))
            .set_spane_id(ProcFreqPage::PAGE_ID)
            .set_fpane_max_height(210.);
        cpu_freq_view.view()
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

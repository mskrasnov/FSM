/* vuln.rs
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

//! CPU Vulnerabilities page

use std::fmt::Display;

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::vulnerabilities::Vulnerabilities;
use iced::{
    Element, Length, Task,
    widget::{Id, button, container, scrollable, table, text},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, KeyboardAndMouse, Message},
    widgets::table::hdr_name,
};

#[derive(Debug, Clone)]
pub struct VulnPage {
    pub vulns: LoadState<Vulnerabilities>,
}

impl VulnPage {
    pub fn new() -> Self {
        Self {
            vulns: LoadState::Loading,
        }
    }

    fn vuln_table<'a>(&'a self, rows: &'a [(String, String)]) -> table::Table<'a, Message> {
        let columns = [
            table::column(
                hdr_name(fl!("vuln-hdr-name")),
                |row: &'a (String, String)| text(row.0.trim()).wrapping(text::Wrapping::Word),
            )
            .width(Length::FillPortion(1)),
            table::column(
                hdr_name(fl!("vuln-hdr-descr")),
                |row: &'a (String, String)| {
                    let s = row.1.trim();
                    let vuln_type = VulnType::detect(s);
                    let vuln_str = s.to_string();

                    button(
                        text(format!("{vuln_type} {vuln_str}"))
                            .wrapping(text::Wrapping::WordOrGlyph)
                            .style(move |t: &iced::Theme| {
                                let p = t.palette();
                                text::Style {
                                    color: Some(match vuln_type {
                                        VulnType::Safe => p.success,
                                        VulnType::Warning => p.warning,
                                        VulnType::Danger => p.danger,
                                        VulnType::Unknown => p.text,
                                    }),
                                }
                            }),
                    )
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(format!("{}: {}", &row.0, &row.1)),
                    ))
                },
            )
            .width(Length::FillPortion(3)),
        ];
        table(columns, rows).padding(2).width(Length::Fill)
    }
}

impl<'a> PageView<'a> for VulnPage {
    fn page_id() -> &'static str {
        "vulns"
    }

    fn page_title() -> String {
        fl!("page-vuln")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.vulns {
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetCpuVulnsData),
            LoadState::Loaded(vulns) => {
                let vulns = &vulns.list;
                let table = container(self.vuln_table(&vulns)).style(container::rounded_box);

                scrollable(table)
                    .spacing(5)
                    .id(Id::new(Self::page_id()))
                    .into()
            }
        }
    }
}

impl PageData for VulnPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { Vulnerabilities::new().to_load_state() },
            DataReceiver::CpuVulnsDataReceived,
        )
    }
}

enum VulnType {
    Safe,
    Warning,
    Danger,
    Unknown,
}

impl VulnType {
    fn detect(descr: &str) -> Self {
        let descr = descr.to_lowercase();
        if descr.contains("vulnerable") {
            Self::Danger
        } else if descr.contains("mitigation") {
            Self::Warning
        } else if descr.contains("not affected") {
            Self::Safe
        } else {
            Self::Unknown
        }
    }
}

impl Display for VulnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Safe => '\u{1F7E2}',
                Self::Warning => '\u{1F7E1}',
                Self::Danger => '\u{1F534}',
                Self::Unknown => '\u{26AA}',
            }
        )
    }
}

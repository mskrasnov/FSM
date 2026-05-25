/* vulnerabilities.rs
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

//! CPU Vulnerabilities page

use crate::{
    Message, fl,
    messages::{ButtonsMessage, DataReceiverMessage},
    widgets::table::hdr_name,
};
use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::vulnerabilities::Vulnerabilities;

use iced::{
    Element, Length, Task,
    widget::{Id, button, container, scrollable, table, text},
};

#[derive(Debug, Clone)]
pub struct VulnPage<'a> {
    pub vulnerabilities: &'a LoadState<Vulnerabilities>,
}

impl<'a> VulnPage<'a> {
    pub const IS_SPECIAL: bool = false;
    pub const PAGE_ID: &'static str = "vulns";

    pub fn new(vulnerabilities: &'a LoadState<Vulnerabilities>) -> Self {
        Self { vulnerabilities }
    }

    pub fn get_data() -> Task<DataReceiverMessage> {
        Task::perform(
            async move {
                let vuln = Vulnerabilities::new();
                vuln.to_load_state()
            },
            |val| DataReceiverMessage::CPUVulnerabilitiesReveived(val),
        )
    }

    pub fn view(&self) -> Element<'a, Message> {
        match self.vulnerabilities {
            LoadState::Loaded(vulns) => {
                let vulns = &vulns.list;
                let table = container(vuln_table(vulns)).style(container::rounded_box);
                container(scrollable(table).spacing(5).id(Id::new(Self::PAGE_ID))).into()
            }
            LoadState::Error(why) => super::error_page(why).into(),
            LoadState::Loading => super::loading_page().into(),
        }
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

fn vuln_table<'a>(rows: &'a [(String, String)]) -> table::Table<'a, Message> {
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
                    text(vuln_str.clone())
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
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    format!("{}: {}", &row.0, &row.1),
                )))
            },
        )
        .width(Length::FillPortion(3)),
    ];
    table(columns, rows).padding(2).width(Length::Fill)
}

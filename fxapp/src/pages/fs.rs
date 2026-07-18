/* fs.rs
 *
 * Copyright 2026 Michail Krasnov <mskrasnov07@ya.ru>
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

//! Storage statistics page

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, KeyboardAndMouse, Message},
    widgets::table::hdr_name,
};
use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::{
    parts::{MountEntry, Mounts},
    utils::Size,
};
use ferrix_widgets::tooltip::icon_tooltip;
use iced::{
    Alignment::Center,
    Color, Element, Font, Length, Task,
    alignment::Horizontal::Right,
    widget::{button, center, container, progress_bar, row, scrollable, stack, table, text},
};

#[derive(Debug, Clone)]
pub struct FSPage {
    pub mounts: LoadState<Mounts>,
}

impl FSPage {
    pub fn new() -> Self {
        Self {
            mounts: LoadState::Loading,
        }
    }
}

impl<'a> PageView<'a> for FSPage {
    fn page_id() -> &'static str {
        "fs"
    }

    fn page_title() -> String {
        fl!("page-fsystems")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.mounts {
            LoadState::Loading => super::loading_page(),
            LoadState::Error(why) => {
                super::error_page::error(why, DataReceiver::GetFilesystemsData)
            }
            LoadState::Loaded(storage) => {
                let mut rows = Vec::with_capacity(storage.mounts.len());
                for part in &storage.mounts {
                    rows.push(TableRow::from(part));
                }
                rows.sort_by(|r1, r2| {
                    let s1 = r1.total_size.get_bytes2().unwrap_or(0);
                    let s2 = r2.total_size.get_bytes2().unwrap_or(0);
                    s2.cmp(&s1)
                });

                scrollable(container(storage_table(rows)).style(container::rounded_box))
                    .id(Self::page_id())
                    .spacing(5)
                    .into()
            }
        }
    }
}

impl PageData for FSPage {
    fn get_data() -> iced::Task<DataReceiver> {
        Task::perform(
            async move { Mounts::new().to_load_state() },
            DataReceiver::FilesystemsDataReceived,
        )
    }
}

#[derive(Debug, Clone)]
struct TableRow<'a> {
    pub device: &'a str,
    pub mount_point: &'a str,
    pub filesystem: &'a str,
    pub options: &'a str,
    pub total_size: Size,
    pub free_size: Size,
    pub used_size: Size,
    pub usage_percent: f32,
}

impl<'a> From<&'a MountEntry> for TableRow<'a> {
    fn from(value: &'a MountEntry) -> Self {
        let fstats = value.fstats;
        let total_size = match fstats {
            Some(fstats) => fstats.total_size(),
            None => Size::None,
        }
        .round(2)
        .unwrap_or_default();
        let free_size = match fstats {
            Some(fstats) => fstats.free_size(),
            None => Size::None,
        }
        .round(2)
        .unwrap_or_default();
        let used_size = match fstats {
            Some(fstats) => fstats.used_size(),
            None => Size::None,
        }
        .round(2)
        .unwrap_or_default();
        let usage_percent = match fstats {
            Some(fstats) => fstats.usage_percent() as f32,
            None => 0.,
        };

        Self {
            device: &value.device,
            mount_point: &value.mount_point,
            filesystem: &value.filesystem,
            options: &value.options,
            total_size,
            free_size,
            used_size,
            usage_percent,
        }
    }
}

fn storage_table<'a>(rows: Vec<TableRow<'a>>) -> Element<'a, Message> {
    let columns = [
        table::column(hdr_name(fl!("storage-dev")), |row: TableRow| {
            row![
                button(text(row.device).font(Font::MONOSPACE))
                    .style(button::text)
                    .padding(0)
                    .on_press(Message::KeyboardAndMouse(
                        KeyboardAndMouse::CopyButtonPressed(row.device.to_string()),
                    )),
                button(icon_tooltip(
                    "about",
                    format!("{}\n{}", row.mount_point, row.options)
                ))
                .style(button::text)
                .padding(0)
                .on_press(Message::KeyboardAndMouse(
                    KeyboardAndMouse::CopyButtonPressed(format!(
                        "{}\n{}",
                        row.mount_point, row.options
                    ))
                )),
            ]
            .spacing(5)
            .align_y(Center)
        }),
        table::column(hdr_name(fl!("storage-fs")), |row: TableRow| {
            button(text(row.filesystem).font(Font::MONOSPACE))
                .style(button::text)
                .padding(0)
                .on_press(Message::KeyboardAndMouse(
                    KeyboardAndMouse::CopyButtonPressed(row.filesystem.to_string()),
                ))
        }),
        table::column(hdr_name(fl!("storage-total")), |row: TableRow| {
            button(text(row.total_size.to_string()))
                .style(button::text)
                .padding(0)
                .on_press(Message::KeyboardAndMouse(
                    KeyboardAndMouse::CopyButtonPressed(row.total_size.to_string()),
                ))
        })
        .align_x(Right),
        table::column(hdr_name(fl!("storage-free")), |row: TableRow| {
            button(text(row.free_size.to_string()))
                .style(button::text)
                .padding(0)
                .on_press(Message::KeyboardAndMouse(
                    KeyboardAndMouse::CopyButtonPressed(row.free_size.to_string()),
                ))
        })
        .align_x(Right),
        table::column(hdr_name(fl!("storage-used")), |row: TableRow| {
            button(text(row.used_size.to_string()))
                .style(button::text)
                .padding(0)
                .on_press(Message::KeyboardAndMouse(
                    KeyboardAndMouse::CopyButtonPressed(row.used_size.to_string()),
                ))
        })
        .align_x(Right),
        table::column(hdr_name(fl!("storage-usage")), |row: TableRow| {
            stack![
                progress_bar(0.0..=100., row.usage_percent)
                    .length(Length::FillPortion(2))
                    .girth(Length::Fixed(15.)), // NOTE: Some fonts may display incorrectly
                center(
                    text(format!("{:.3}%", row.usage_percent)).style(move |s: &iced::Theme| {
                        let color = if row.usage_percent >= 40. {
                            Color::BLACK
                        } else {
                            s.palette().text
                        };

                        text::Style { color: Some(color) }
                    })
                ),
            ]
        })
        .width(Length::FillPortion(1)),
    ];
    table(columns, rows).padding(2).width(Length::Fill).into()
}

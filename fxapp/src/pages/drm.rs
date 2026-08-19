/* drm.rs
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

//! DRM Page

use ferrix_data::load_state::{LoadState, ToLoadState};
use ferrix_lib::drm::{DRM, EDID, Video, VideoInputParams};
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Element, Length, Task,
    widget::{Column, button, center, column, container, text},
};

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, Message, PageMessage},
    widgets::table::{InfoRow, fmt_val, kv_info_table},
};

#[derive(Debug, Clone)]
pub struct DRMPage {
    pub drm: LoadState<Video>,
    pub id: usize,
}

impl DRMPage {
    pub fn new() -> Self {
        Self {
            drm: LoadState::Loading,
            id: 0,
        }
    }

    fn drm_page<'a>(&self, drm: &'a Video) -> Element<'a, Message> {
        let vid_names = get_screens_names(drm);

        if vid_names.is_empty() {
            return container(center(text(fl!("drm-is-empty")).size(16))).into();
        }

        let vid_list = {
            let mut elements = Vec::with_capacity(vid_names.len());
            for v in vid_names {
                let b = button(text(v.1))
                    .on_press(Message::PageMessage(PageMessage::DRMPage(
                        DRMPageMessage::ScreenSelected(v.0),
                    )))
                    .style(if v.0 == self.id {
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
                text(fl!("drm-fpanel")).style(text::secondary),
                Column::from_vec(vid_list)
            ]
            .spacing(5),
        )
        .style(container::rounded_box)
        .width(Length::Fill)
        .padding(2);
        let second_panel = screen_subpage(&drm.devices, self.id);

        let view = SeparatedView::new(first_panel, second_panel)
            .set_fpane_id(Self::scrolled_page_id().unwrap_or(""))
            .set_spane_id(Self::page_id())
            .set_fpane_max_height(Length::Shrink)
            .set_spane_max_height(Length::Fill);
        container(view.view()).into()
    }
}

impl<'a> PageView<'a> for DRMPage {
    fn page_id() -> &'static str {
        "drm"
    }

    fn scrolled_page_id() -> Option<&'static str> {
        Some("drm_contents")
    }

    fn page_group() -> super::GroupVariant {
        super::GroupVariant::Hardware
    }

    fn page_title() -> String {
        fl!("page-screen")
    }

    fn page_title_controls(&'a self) -> Option<Element<'a, Message>> {
        let update_button = button(text(fl!("err-page-update")))
            .on_press(Message::DataReceiver(DataReceiver::GetDRMData))
            .padding(3);
        Some(update_button.into())
    }

    fn page_contents_view(&'a self) -> Element<'a, Message> {
        match &self.drm {
            LoadState::Loaded(drm) => self.drm_page(drm),
            LoadState::Error(why) => super::error_page::error(why, DataReceiver::GetDRMData),
            LoadState::Loading => super::loading_page(),
        }
    }
}

impl PageData for DRMPage {
    fn get_data() -> Task<DataReceiver> {
        Task::perform(
            async move { Video::new().to_load_state() },
            DataReceiver::DRMDataReceived,
        )
    }
}

fn get_screens_names<'a>(video: &'a Video) -> Vec<(usize, String)> {
    let mut i = 0;
    let j = video.devices.len();
    let mut v = Vec::with_capacity(j);
    while i < j {
        v.push((
            i,
            format!(
                "{}: {}",
                fl!("drm-title", idx = i),
                get_screen_name(&video.devices[i]),
            ),
        ));
        i += 1;
    }
    v
}

fn get_screen_name<'a>(screen: &'a ferrix_lib::drm::DRM) -> String {
    if !screen.enabled {
        fl!("drm-disabled")
    } else {
        match &screen.edid {
            Some(edid) => format!("{} {:0x}", &edid.manufacturer, edid.product_code),
            None => fl!("drm-unknown"),
        }
    }
}

pub fn get_first_active_screen<'a>(screens: &'a LoadState<Video>) -> Option<usize> {
    match screens {
        LoadState::Loaded(screens) => {
            let mut i = 0;
            let j = screens.devices.len();
            while i < j {
                if screens.devices[i].enabled {
                    return Some(i);
                }
                i += 1;
            }
            None
        }
        _ => None,
    }
}

fn screen_subpage<'a>(drm: &'a [DRM], idx: usize) -> Element<'a, Message> {
    if drm.is_empty() {
        return container(center(
            text(fl!("drm-is-empty")).size(16).style(text::secondary),
        ))
        .height(Length::Fill)
        .into();
    }

    let drm = &drm[idx];

    let mut layout = column![].spacing(5);

    layout = if drm.enabled {
        match &drm.edid {
            Some(edid) => layout.push(
                column![
                    text(fl!("drm-summary")).style(text::warning),
                    edid_summary_table(edid),
                    text(fl!("drm-vparams")).style(text::warning),
                    edid_video_params_table(edid),
                ]
                .spacing(5),
            ),
            None => layout.push(text(fl!("drm-edid-not-found", idx = idx))),
        }
    } else {
        layout.push(text(fl!("drm-not-enabled", idx = idx)).style(text::danger))
    };

    if drm.enabled {
        layout = layout.push(text(fl!("drm-modes")).style(text::warning));
        layout = layout.push(support_modes_table(&drm.modes));
    }

    container(layout).height(Length::Fill).into()
}

fn support_modes_table<'a>(modes: &'a [String]) -> Element<'a, Message> {
    let mut rows = Vec::with_capacity(modes.len());
    for mode in modes {
        rows.push(InfoRow::new(fl!("drm-mode"), fmt_val(Some(mode))));
    }
    container(kv_info_table(rows))
        .style(container::rounded_box)
        .into()
}

fn edid_summary_table<'a>(edid: &'a EDID) -> Element<'a, Message> {
    let rows = vec![
        InfoRow::new(fl!("drm-manufacturer"), Some(edid.manufacturer.clone())),
        InfoRow::new(fl!("drm-pcode"), fmt_val(Some(edid.product_code))),
        InfoRow::new(fl!("drm-snum"), Some(format!("{:X}", edid.serial_number))),
        InfoRow::new(
            fl!("drm-date"),
            Some(format!("{}/{}", edid.week, edid.year)),
        ),
        InfoRow::new(fl!("drm-edid-ver"), fmt_val(Some(edid.edid_version))),
        InfoRow::new(fl!("drm-edid-rev"), fmt_val(Some(edid.edid_revision))),
        InfoRow::new(
            fl!("drm-size"),
            Some(format!("{}x{}", edid.hscreen_size, edid.vscreen_size)),
        ),
        InfoRow::new(fl!("drm-gamma"), fmt_val(Some(edid.display_gamma))),
    ];
    container(kv_info_table(rows))
        .style(container::rounded_box)
        .into()
}

fn edid_video_params_table<'a>(edid: &'a EDID) -> Element<'a, Message> {
    let rows = match &edid.video_input {
        VideoInputParams::Digital(val) => vec![
            InfoRow::new(fl!("drm-signal"), Some(fl!("drm-digital"))),
            InfoRow::new(fl!("drm-bit-depth"), Some(format!("{}", val.bit_depth))),
            InfoRow::new(
                fl!("drm-interface"),
                Some(format!("{}", val.video_interface)),
            ),
        ],
        VideoInputParams::Analog(val) => vec![
            InfoRow::new(fl!("drm-signal"), Some(fl!("drm-analog"))),
            InfoRow::new("White sync levels", fmt_val(Some(val.white_sync_levels))),
            InfoRow::new(
                "Blank to black setup",
                fmt_val(Some(val.blank_to_black_setup)),
            ),
            InfoRow::new(
                "Separate sync supported",
                fmt_val(Some(val.separate_sync_supported)),
            ),
            InfoRow::new(
                "Composite sync supported",
                fmt_val(Some(val.composite_sync_supported)),
            ),
            InfoRow::new(
                "Sync on green supported",
                fmt_val(Some(val.sync_on_green_supported)),
            ),
            InfoRow::new(
                "Sync on green issued",
                fmt_val(Some(val.sync_on_green_isused)),
            ),
        ],
    };
    container(kv_info_table(rows))
        .style(container::rounded_box)
        .into()
}

#[derive(Debug, Clone)]
pub enum DRMPageMessage {
    ScreenSelected(usize),
}

impl DRMPageMessage {
    pub fn update<'a>(self, page: &'a mut DRMPage) -> Task<Message> {
        match self {
            Self::ScreenSelected(id) => {
                page.id = id;
                Task::none()
            }
        }
    }
}

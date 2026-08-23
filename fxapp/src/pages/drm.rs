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
use ferrix_lib::drm::{DRM, DetailedTiming, EDID, RangeLimits, Video, VideoInputParams};
use ferrix_widgets::separated_view::SeparatedView;
use iced::{
    Element, Font, Length, Task,
    widget::{Column, button, center, column, container, text},
};
use std::fmt::Write;

use super::{PageData, PageView};
use crate::{
    fl,
    message::{DataReceiver, Message, PageMessage},
    widgets::table::{InfoRow, fmt_bool, fmt_val, kv_info_table},
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
            Some(edid) => format!("{} {}", &edid.manufacturer, &edid.model,),
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
                    edid_detailed_timings_blocks_table(edid),
                    edid_range_limits_table(edid),
                    edid_raw_table(edid),
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
        InfoRow::new(
            fl!("drm-manufacturer"),
            Some(format!(
                "{} ({})",
                &edid.manufacturer,
                edid.description.clone().unwrap_or("unknown".to_string())
            )),
        ),
        InfoRow::new(fl!("drm-pcode"), fmt_val(Some(edid.product_code))),
        InfoRow::new(
            fl!("drm-snum"),
            Some(match &edid.serial {
                Some(serial) => format!("{} ({})", edid.serial_number, serial),
                None => format!("{}", edid.serial_number),
            }),
        ),
        InfoRow::new(fl!("drm-model"), Some(edid.model.to_string())),
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
        InfoRow::new(
            fl!("drm-diag"),
            edid.diagonal_inches.and_then(|d| Some(format!("{d:.1}\""))),
        ),
        InfoRow::new(fl!("drm-resol"), fmt_resolution(edid)),
        InfoRow::new(fl!("drm-aspratio"), edid.aspect_ratio.clone()),
        InfoRow::new(
            fl!("drm-pixclck"),
            edid.pixel_clock_mhz.and_then(|p| Some(format!("{p} MHz"))),
        ),
        InfoRow::new(fl!("drm-extblcks"), Some(edid.extension_blocks.to_string())),
        InfoRow::new(fl!("drm-cksum"), Some(edid.checksum.to_string())),
    ];
    container(kv_info_table(rows))
        .style(container::rounded_box)
        .into()
}

fn fmt_resolution<'a>(edid: &'a EDID) -> Option<String> {
    match (edid.resolution_width, edid.resolution_height) {
        (Some(w), Some(h)) => Some(format!("{w}x{h}")),
        _ => None,
    }
}

fn edid_raw_table<'a>(edid: &'a EDID) -> Element<'a, Message> {
    let raw_edid_hex = edid_raw_row(edid).trim().to_string();
    let raw_value = container(
        button(text(raw_edid_hex.clone()).font(Font::MONOSPACE))
            .style(button::text)
            .padding(0)
            .on_press(Message::KeyboardAndMouse(
                crate::message::KeyboardAndMouse::CopyButtonPressed(raw_edid_hex),
            )),
    )
    .style(container::rounded_box)
    .padding(2)
    .width(Length::Fill);

    column![text(fl!("drm-edid-raw")).style(text::warning), raw_value,]
        .spacing(5)
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

fn edid_detailed_timings_blocks_table<'a>(edid: &'a EDID) -> Element<'a, Message> {
    let len = edid.detailed_timings.len();
    if len == 0 {
        return text(fl!("drm-no-dtb")).style(text::danger).into();
    }

    let mut table = Column::with_capacity(len).spacing(5);
    for dt in edid.detailed_timings.iter().enumerate() {
        table = table.push(edid_detailed_timings_table_single(dt.0, dt.1));
    }
    table.into()
}

fn edid_detailed_timings_table_single<'a>(
    idx: usize,
    dt: &'a DetailedTiming,
) -> Element<'a, Message> {
    let rows = vec![
        InfoRow::new(
            fl!("drm-pixclck"),
            Some(format!("{} Hz", dt.pixel_clock_hz)),
        ),
        InfoRow::new(fl!("drm-aspratio"), Some(dt.aspect_ratio.clone())),
        InfoRow::new(fl!("drm-h-active"), Some(format!("{} px", dt.h_active))),
        InfoRow::new(fl!("drm-v-active"), Some(format!("{} lines", dt.v_active))),
        InfoRow::new(fl!("drm-h-blanking"), Some(format!("{} px", dt.h_blanking))),
        InfoRow::new(
            fl!("drm-v-blanking"),
            Some(format!("{} lines", dt.v_blanking)),
        ),
        InfoRow::new(
            fl!("drm-h-front-porch"),
            Some(format!("{} px", dt.h_front_porch)),
        ),
        InfoRow::new(
            fl!("drm-h-sync-pulse"),
            Some(format!("{} px", dt.h_sync_pulse)),
        ),
        InfoRow::new(
            fl!("drm-v-front-porch"),
            Some(format!("{} lines", dt.v_front_porch)),
        ),
        InfoRow::new(
            fl!("drm-v-sync-pulse"),
            Some(format!("{} lines", dt.v_sync_pulse)),
        ),
        InfoRow::new(
            fl!("drm-h-back-porch"),
            Some(format!("{} px", dt.h_back_porch)),
        ),
        InfoRow::new(
            fl!("drm-v-back-porch"),
            Some(format!("{} lines", dt.v_back_porch)),
        ),
        InfoRow::new(fl!("drm-h-sync-pos"), fmt_bool(Some(dt.h_sync_positive))),
        InfoRow::new(fl!("drm-v-sync-pos"), fmt_bool(Some(dt.v_sync_positive))),
    ];

    column![
        text(fl!("drm-dtdb", idx = idx)).style(text::warning),
        container(kv_info_table(rows)).style(container::rounded_box),
    ]
    .spacing(5)
    .into()
}

fn edid_range_limits_table<'a>(edid: &'a EDID) -> Element<'a, Message> {
    match &edid.range_limits {
        Some(rl) => column![
            text(fl!("drm-rl")).style(text::warning),
            range_limits_table(rl)
        ]
        .spacing(5)
        .into(),
        None => text(fl!("drm-no-rl")).style(text::danger).into(),
    }
}

fn range_limits_table<'a>(rl: &'a RangeLimits) -> Element<'a, Message> {
    let rows = vec![
        InfoRow::new(
            fl!("drm-min-v-freq"),
            Some(format!("{} Hz", rl.min_v_freq_hz)),
        ),
        InfoRow::new(
            fl!("drm-max-v-freq"),
            Some(format!("{} Hz", rl.max_v_freq_hz)),
        ),
        InfoRow::new(
            fl!("drm-min-h-freq"),
            Some(format!("{} kHz", rl.min_h_freq_khz)),
        ),
        InfoRow::new(
            fl!("drm-max-h-freq"),
            Some(format!("{} kHz", rl.max_h_freq_khz)),
        ),
        InfoRow::new(
            fl!("drm-max-pixclck"),
            Some(format!("{} MHz", rl.max_pixel_clock_mhz)),
        ),
    ];

    container(kv_info_table(rows))
        .style(container::rounded_box)
        .into()
}

fn edid_raw_row<'a>(edid: &'a EDID) -> String {
    // 16 по 2
    let data = &edid.raw;
    let mut hex_string = String::new();
    hex_string.reserve(data.len() * 3 + data.len() / 16 + 1);

    let (mut k, j) = (0, 8);
    for chunk in data.chunks(16) {
        for (i, &byte) in chunk.iter().enumerate() {
            if i > 0 {
                hex_string.push(' ');
            }
            write!(hex_string, "{byte:02X}").unwrap();
        }
        hex_string.push('\n');
        if k >= j {
            hex_string.push('\n');
            k = 0;
        }
        k += 1;
    }
    hex_string
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

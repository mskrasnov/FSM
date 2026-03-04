/* about.rs
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

//! About this program page

use crate::{fl, messages::Message, widgets::link_button};
use iced::{
    Alignment::{self, Center},
    widget::{column, container, row, rule, svg, text},
};

pub fn about_page<'a>() -> container::Container<'a, Message> {
    let img = svg("/usr/share/Ferrix/com.mskrasnov.Ferrix.svg")
        .width(128)
        .height(128);
    let header = row![
        img,
        column![
            text(fl!("about-hdr")).size(24),
            text(format!(
                "{}: {}, {}: {}",
                fl!("about-ferrix"),
                env!("CARGO_PKG_VERSION"),
                fl!("about-flib"),
                ferrix_lib::FX_LIB_VERSION,
            ))
            .size(14),
        ]
        .spacing(5),
    ]
    .align_y(Center)
    .spacing(5);

    let about_info = row![
        column![
            text(fl!("about-author-hdr")).style(text::secondary),
            text(fl!("about-feedback-hdr")).style(text::secondary),
            text(fl!("about-source-hdr")).style(text::secondary),
            text("crates.io:").style(text::secondary),
            text(fl!("about-blog")).style(text::secondary),
        ]
        .align_x(Alignment::End)
        .spacing(3),
        column![
            row![
                text(fl!("about-author")),
                link_button("(GitHub)", "https://github.com/mskrasnov"),
            ]
            .spacing(5),
            link_button("mskrasnov07 at ya dot ru", "mailto:mskrasnov07@ya.ru"),
            link_button("GitHub", "https://github.com/mskrasnov/Ferrix"),
            row![
                link_button("ferrix-app", "https://crates.io/crates/ferrix-app"),
                text(", "),
                link_button("ferrix-lib", "https://crates.io/crates/ferrix-lib"),
            ],
            link_button("mskrasnov", "https://boosty.to/mskrasnov"),
        ]
        .spacing(3),
    ]
    .spacing(5);

    let donate = column![
        text(fl!("about-donate")),
        link_button(fl!("about-donate-lbl"), "https://boosty.to/mskrasnov"),
    ]
    .spacing(5);

    let contents = column![
        column![header, rule::horizontal(1)].spacing(2),
        about_info,
        row![
            text(fl!("about-support")).style(text::warning).size(16),
            rule::horizontal(1)
        ]
        .align_y(Center)
        .spacing(5),
        donate,
    ]
    .spacing(5);

    container(contents)
}

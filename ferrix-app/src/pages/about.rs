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
use ferrix_widgets::items_list::*;
use iced::{
    Alignment::Center,
    Element,
    widget::{column, row, svg, text},
};

pub fn about_page<'a>() -> Element<'a, Message> {
    let img = svg("/usr/share/Ferrix/com.mskrasnov.Ferrix.svg")
        .width(96)
        .height(96);

    let about_list = items_group(
        column![
            list_item(fl!("about-ferrix"), env!("CARGO_PKG_VERSION"),),
            list_item(fl!("about-flib"), ferrix_lib::FX_LIB_VERSION,),
            list_item(
                fl!("about-author-hdr"),
                row![
                    text(fl!("about-author")),
                    link_button("(GitHub)", "https://github.com/mskrasnov")
                ]
                .spacing(5)
            ),
            list_item(
                fl!("about-feedback-hdr"),
                link_button("mskrasnov07 at ya dot ru", "mailto:mskrasnov07@ya.ru")
            ),
            list_item(
                fl!("about-source-hdr"),
                link_button("GitHub", "https://github.com/mskrasnov/FSM"),
            ),
            list_item(
                "crates.io:",
                row![
                    link_button("ferrix-app", "https://crates.io/crates/ferrix-app"),
                    text(", "),
                    link_button("ferrix-lib", "https://crates.io/crates/ferrix-lib"),
                ],
            ),
            list_item(
                fl!("about-blog"),
                link_button("mskrasnov", "https://boosty.to/mskrasnov"),
            ),
        ]
        .spacing(5),
    );

    let donate_list = items_group(
        column![
            list_item(
                "Boosty",
                link_button(fl!("about-donate-lbl"), "https://boosty.to/mskrasnov"),
            ),
            list_item("Card", text("2202 2062 5233 5406 (Sberbank, Russia)"),),
        ]
        .spacing(5),
    );

    items_list_container(
        column![
            img,
            text(fl!("about-hdr")).size(17),
            simple_list_header(fl!("about-sum")),
            about_list,
            simple_list_header(fl!("about-support")),
            donate_list,
        ]
        .align_x(Center)
        .spacing(5),
    )
}

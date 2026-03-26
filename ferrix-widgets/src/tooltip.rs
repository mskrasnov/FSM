/* tooltip.rs
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

//! Tooltip widget

use crate::icons;
use iced::{
    Color, Element,
    widget::{
        container, text,
        text::IntoFragment,
        tooltip::{self, Position},
    },
};

pub fn tooltip_txt<'a, T>(txt: T) -> text::Text<'a>
where
    T: IntoFragment<'a>,
{
    text(txt).size(11).style(|s: &iced::Theme| text::Style {
        color: Some(if s.extended_palette().is_dark {
            s.palette().text
        } else {
            Color::WHITE
        }),
    })
}

pub fn icon_tooltip<'a, T, Message>(icon_name: &'a str, txt: T) -> container::Container<'a, Message>
where
    T: IntoFragment<'a>,
    Message: 'a + Clone,
{
    let icon = icons::icon(icon_name).width(16).height(16);
    container(tooltip(icon, tooltip_txt(txt)))
        .width(16)
        .height(16)
}

pub fn tooltip<'a, C, T, Message>(
    main_contents: C,
    tooltip_contents: T,
) -> tooltip::Tooltip<'a, Message>
where
    C: Into<Element<'a, Message>>,
    T: Into<Element<'a, Message>>,
    Message: 'a + Clone,
{
    iced::widget::tooltip(
        main_contents,
        container(tooltip_contents)
            .padding(2)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
                border: iced::Border {
                    radius: iced::border::Radius::from(2),
                    ..iced::Border::default()
                },
                ..Default::default()
            }),
        Position::Bottom,
    )
}

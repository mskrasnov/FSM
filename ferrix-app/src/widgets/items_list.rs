/* items_list.rs
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

//! Items list widget

use iced::{
    Alignment::Center,
    Element, Pixels,
    widget::{container, row, rule, space, text},
};

use crate::{messages::Message, widgets::icon_tooltip};

pub fn items_list_container<'a, Message: Clone + 'a>(
    contents: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![
        space::horizontal(),
        container(contents.into())
            .width(450)
            .max_width(Pixels(550.)),
        space::horizontal(),
    ]
    .align_y(Center)
    .into()
}

pub fn items_container<'a, Message: Clone + 'a>(
    contents: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![
        space::horizontal(),
        container(contents.into())
            .width(450)
            .max_width(Pixels(550.)),
        space::horizontal(),
    ]
    .align_y(Center)
    .into()
}

pub fn items_group<'a, Message: Clone + 'a>(
    contents: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(contents.into())
        .style(container::bordered_box)
        .padding(5)
        .into()
}

pub fn list_item<'a, T, C, Message>(header: T, contents: C) -> Element<'a, Message>
where
    T: text::IntoFragment<'a>,
    C: Into<Element<'a, Message>>,
    Message: Clone + 'a,
{
    row![text(header), space::horizontal(), contents.into()]
        .align_y(Center)
        .into()
}

pub fn list_header<'a, T>(header: T, tooltip: T) -> row::Row<'a, Message>
where
    T: text::IntoFragment<'a>,
{
    row![
        text(header).size(16),
        icon_tooltip("about", tooltip),
        rule::horizontal(1.)
    ]
    .spacing(5)
    .align_y(Center)
}

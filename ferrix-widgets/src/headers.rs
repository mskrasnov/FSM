/* headers.rs
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

//! Interface headers

use iced::{
    Alignment::Center,
    Theme,
    widget::{Column, column, row, rule, text, text::IntoFragment},
};

pub fn header<'a, T, Message>(txt: T) -> row::Row<'a, Message>
where
    T: 'a + IntoFragment<'a>,
    Message: 'a + Clone,
{
    row![text(txt).size(16), rule::horizontal(1),]
        .spacing(5)
        .align_y(Center)
}

pub fn header_text<'a, Message>(txt: String) -> Column<'a, Message>
where
    Message: 'a + Clone,
{
    column![text(txt).size(22), rule::horizontal(1)].spacing(2)
}

pub fn category_header<'a, T>(txt: T) -> text::Text<'a>
where
    T: IntoFragment<'a> + 'a,
{
    text(txt).size(14).style(|t: &Theme| {
        let palette = t.palette();
        let text_color = palette.text.scale_alpha(0.7);

        let mut style = text::Style::default();
        style.color = Some(text_color);

        style
    })
}

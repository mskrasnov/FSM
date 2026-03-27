/* container.rs
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

//! Custom container widgets

use crate::headers::category_header;
use iced::{
    Border, Element,
    widget::{column, container, text::IntoFragment},
};

pub fn glassy_container<'a, T, C, Message>(
    header: T,
    content: C,
) -> container::Container<'a, Message>
where
    T: 'a + IntoFragment<'a>,
    C: 'a + Into<Element<'a, Message>>,
    Message: 'a + Clone,
{
    container(column![category_header(header), content.into()].spacing(5))
        .padding(5)
        .style(|theme: &iced::Theme| {
            let is_dark = theme.extended_palette().is_dark;
            let text_color = theme.palette().text;

            let base_color = match is_dark {
                true => text_color,
                false => theme.extended_palette().background.strong.color,
            };
            let background_color = base_color.scale_alpha(match is_dark {
                true => 0.03,
                false => 0.7,
            });
            let border_color = match is_dark {
                true => base_color,
                false => iced::Color::BLACK,
            }
            .scale_alpha(0.08);

            container::Style::default()
                .background(background_color)
                .border(Border {
                    color: border_color,
                    width: 1.,
                    radius: 5.0.into(),
                })
        })
}

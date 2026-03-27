/* widgets.rs
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

//! Custom widgets for UI

use crate::{
    messages::{ButtonsMessage, Message},
    pages::Page,
};
use ferrix_widgets::{
    headers::category_header,
    tooltip::{tooltip, tooltip_txt},
};
use iced::{
    Border, Element,
    widget::{Tooltip, button, column, container, text, text::IntoFragment},
};

pub mod card;
pub mod items_list;
pub mod line_charts;
pub mod table;

pub fn sidebar_button<'a>(page: Page, cur_page: Page) -> button::Button<'a, Message> {
    button(text(page.title_str()))
        .style(if page != cur_page {
            button::subtle
        } else {
            button::secondary
        })
        .on_press(Message::SelectPage(page))
}

pub fn link_button<'a, P, L>(placeholder: P, link: L) -> Tooltip<'a, Message>
where
    P: IntoFragment<'a>,
    L: ToString + IntoFragment<'a> + 'a,
{
    tooltip(
        button(text(placeholder))
            .style(super::styles::link_button)
            .padding(0)
            .on_press(Message::Buttons(ButtonsMessage::LinkButtonPressed(
                link.to_string(),
            ))),
        tooltip_txt(link),
    )
}

pub fn glassy_container<'a, T, C>(header: T, content: C) -> container::Container<'a, Message>
where
    T: IntoFragment<'a> + 'a,
    C: Into<Element<'a, Message>> + 'a,
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

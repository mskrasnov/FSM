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
use ferrix_widgets::tooltip::{tooltip, tooltip_txt};
use iced::widget::{Tooltip, button, text, text::IntoFragment};

pub mod card;
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

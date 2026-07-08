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

pub mod table;

use crate::message::{KeyboardAndMouse, Message};
use ferrix_widgets::tooltip::{tooltip, tooltip_txt};
use iced::widget::{Tooltip, button, text, text::IntoFragment};

pub fn link_button<'a, N, L>(name: N, link: L) -> Tooltip<'a, Message>
where
    N: IntoFragment<'a>,
    L: ToString + IntoFragment<'a> + 'a,
{
    let btn = button(text(name))
        .style(button::danger)
        .padding(0)
        .on_press(Message::KeyboardAndMouse(
            KeyboardAndMouse::LinkButtonPressed(link.to_string()),
        ));
    tooltip(btn, tooltip_txt(link))
}

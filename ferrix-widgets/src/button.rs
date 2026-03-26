/* button.rs
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

//! Custom `Button` widget

use crate::{
    icons,
    tooltip::{tooltip, tooltip_txt},
};
use iced::widget::{button, text::IntoFragment};

/// Button with an icon instead of text label
pub fn icon_button<'a, T, Message>(icon_name: &'a str, txt: T) -> button::Button<'a, Message>
where
    T: IntoFragment<'a>,
    Message: 'a + Clone,
{
    let icon = icons::icon(icon_name).width(16).height(16);
    button(tooltip(icon, tooltip_txt(txt)))
        .style(button::subtle)
        .padding(2)
}

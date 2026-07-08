/* error_page.rs
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

use crate::message::DataReceiver;
use iced::widget::{button, center, column, text};

pub fn error<'a>(etext: &'a str, message: DataReceiver) -> iced::Element<'a, DataReceiver> {
    let update_btn = button("Update").on_press(message).style(button::danger);

    center(column![text("Error!").size(26), text(etext), update_btn,].spacing(5)).into()
}

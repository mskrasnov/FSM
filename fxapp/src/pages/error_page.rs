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

use crate::{
    fl,
    message::{DataReceiver, KeyboardAndMouse, Message},
};
use ferrix_widgets::icons::ERROR_ICON;
use iced::{
    Alignment::Center,
    Font,
    widget::{button, center, column, container, row, scrollable, svg, text},
};

pub fn error<'a>(etext: &'a str, message: DataReceiver) -> iced::Element<'a, Message> {
    let update_btn = button(text(fl!("err-page-update")))
        .on_press(Message::DataReceiver(message))
        .style(button::danger);
    let err_icon = svg(svg::Handle::from_memory(ERROR_ICON))
        .width(20)
        .height(20);

    let err_text = format!("{}\n\n{etext}", fl!("err-page-backend"),);
    let err_body = container(scrollable(
        button(text(err_text.clone()).font(Font::MONOSPACE))
            .style(button::text)
            .padding(0)
            .on_press(Message::KeyboardAndMouse(
                KeyboardAndMouse::CopyButtonPressed(etext.to_string()),
            )),
    ))
    .max_width(440)
    .max_height(320)
    .width(440)
    .height(320)
    .padding(5)
    .style(container::rounded_box);

    let err_header = row![err_icon, text(fl!("err-page-tooltip")).size(20),]
        .spacing(5)
        .align_y(Center);

    center(
        column![err_header, err_body, update_btn]
            .spacing(5)
            .align_x(Center),
    )
    .into()
}

/* navigation.rs
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

use iced::{
    Element, Length,
    widget::{button, column, container, scrollable, text},
};

use crate::{
    message::Message,
    pages::{GroupVariant, PageVariant},
};

pub fn sidebar<'a>(current: PageVariant) -> Element<'a, Message> {
    let mut col = column![].spacing(5);
    let mut last_i = 0;
    let j = PageVariant::ALL.len();

    'grp: for group in GroupVariant::ALL {
        col = col.push(text(group.title()).style(text::secondary));
        let mut i = last_i;
        while i < j {
            let page = PageVariant::ALL[i];
            if &page.group() != group {
                last_i = i;
                continue 'grp;
            }
            col = col.push(
                button(text(page.title()))
                    .on_press(Message::SelectPage(page))
                    .style(if current == page {
                        button::primary
                    } else {
                        button::subtle
                    }),
            );
            last_i = i;
            i += 1;
        }
    }

    container(scrollable(col).spacing(5).id("sidebar"))
        .padding(5)
        .height(Length::Fill)
        .style(container::bordered_box)
        .into()
}

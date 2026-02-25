/* card.rs
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

//! Card widget for dashboard and OSD

use crate::messages::Message;
use iced::{
    Element, Font, Length, Vector, color,
    font::Weight,
    never,
    widget::{button, column, container, rich_text, space, span},
};

pub struct Card {
    header: String,
    on_press: Message,
    is_transparent: bool,
}

impl Card {
    pub fn new(header: String, on_press: Message) -> Self {
        Self {
            is_transparent: false,
            on_press,
            header,
        }
    }

    pub fn set_transparent(mut self, trans: bool) -> Self {
        self.is_transparent = trans;
        self
    }

    pub fn widget<'a, C>(&self, contents: C) -> Element<'a, Message>
    where
        C: Into<Element<'a, Message>>,
    {
        let is_transparent = self.is_transparent;
        let cont = container(column![
            rich_text![
                span(self.header.clone())
                    .font(Font {
                        weight: Weight::Bold,
                        ..Default::default()
                    })
                    .size(16)
            ]
            .on_link_click(never),
            space().width(Length::Fill).height(Length::Fill),
            contents.into(),
        ])
        .padding(5)
        .width(135)
        .height(135)
        .max_width(135)
        .max_height(135)
        .style(move |theme| default(theme, is_transparent));

        button(cont)
            .padding(0)
            .style(button::text)
            .on_press(self.on_press.clone())
            .into()
    }
}

pub fn default(theme: &iced::Theme, is_transparent: bool) -> container::Style {
    let is_dark = theme.extended_palette().is_dark;
    let shadow = match is_transparent {
        true => iced::Shadow::default(),
        false => iced::Shadow {
            color: match is_dark {
                true => color!(0x1d2021),
                false => color!(0xebdbb2),
            },
            offset: Vector::new(2., 2.),
            blur_radius: 2.,
        },
    };
    let mut style = container::rounded_box(theme);
    style.shadow = shadow;
    if is_transparent {
        style.background = style.background.and_then(|b| Some(b.scale_alpha(0.6)));
    }
    style
}

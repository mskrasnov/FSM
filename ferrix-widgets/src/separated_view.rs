/* separated_view.rs
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

//! Two-pane view

use iced::{
    Element, Length, Pixels,
    widget::{Id, column, container, scrollable},
};

pub struct SeparatedView<'a, Message> {
    pub first_pane: Element<'a, Message>,
    pub second_pane: Element<'a, Message>,
    pub first_pane_id: Option<&'static str>,
    pub second_pane_id: Option<&'static str>,
    pub fpane_max_height: f32,
}

impl<'a, Message: 'a + Clone> SeparatedView<'a, Message> {
    pub fn new(f: impl Into<Element<'a, Message>>, s: impl Into<Element<'a, Message>>) -> Self {
        Self {
            first_pane: f.into(),
            second_pane: s.into(),
            first_pane_id: None,
            second_pane_id: None,
            fpane_max_height: 170.,
        }
    }

    pub fn set_fpane_id(mut self, id: &'static str) -> Self {
        self.first_pane_id = Some(id);
        self
    }

    pub fn set_spane_id(mut self, id: &'static str) -> Self {
        self.second_pane_id = Some(id);
        self
    }

    pub fn set_fpane_max_height(mut self, height: f32) -> Self {
        self.fpane_max_height = height;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let f = container(
            scrollable(self.first_pane)
                .width(Length::Fill)
                .spacing(5)
                .id(match self.first_pane_id {
                    Some(id) => Id::new(id),
                    None => Id::unique(),
                }),
        );
        let s = container(
            scrollable(self.second_pane)
                .width(Length::Fill)
                .spacing(5)
                .id(match self.second_pane_id {
                    Some(id) => Id::new(id),
                    None => Id::unique(),
                }),
        );

        container(
            column![
                f.height(Length::Shrink)
                    .max_height(Pixels(self.fpane_max_height)),
                s,
            ]
            .spacing(5),
        )
        .into()
    }
}

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

use iced::{
    Alignment::Center,
    Color, Element, Theme, Border,
    widget::{
        Column, button, column, container, row, rule, svg, svg::Handle, text, text::IntoFragment,
        tooltip, tooltip::Position,
    },
};

pub mod card;
pub mod line_charts;
pub mod table;
pub mod separated_view;
pub mod items_list;

use crate::{
    icons::{ABOUT_ICON, ERROR_ICON, EXPORT_ICON, SETTINGS_ICON},
    messages::{ButtonsMessage, Message},
    pages::Page,
};

pub fn icon_button<'a>(icon_name: &'a str, tooltip: String) -> button::Button<'a, Message> {
    let svg_bytes = match icon_name {
        "about" => ABOUT_ICON,
        "error" => ERROR_ICON,
        "export" => EXPORT_ICON,
        "settings" => SETTINGS_ICON,
        _ => &[],
    };
    let icon = svg(Handle::from_memory(svg_bytes)).style(|theme: &iced::Theme, _| svg::Style {
        color: Some(theme.palette().text),
    });

    button(iced::widget::tooltip(
        icon.width(16).height(16),
        container(text(tooltip).size(11).style(|s: &iced::Theme| text::Style {
            color: Some(if s.extended_palette().is_dark {
                s.palette().text
            } else {
                Color::WHITE
            }),
        }))
        .padding(2)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
            border: iced::Border {
                radius: iced::border::Radius::from(2),
                ..iced::Border::default()
            },
            ..Default::default()
        }),
        Position::Bottom,
    ))
    .style(button::subtle)
    .padding(2)
}

pub fn sidebar_button<'a>(page: Page, cur_page: Page) -> button::Button<'a, Message> {
    button(text(page.title_str()))
        .style(if page != cur_page {
            button::subtle
        } else {
            button::secondary
        })
        .on_press(Message::SelectPage(page))
}

pub fn link_button<'a, P, L>(placeholder: P, link: L) -> tooltip::Tooltip<'a, Message>
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
        container(text(link).size(11).style(|s: &iced::Theme| text::Style {
            color: Some(if s.extended_palette().is_dark {
                s.palette().text
            } else {
                Color::WHITE
            }),
        }))
        .padding(2)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 0.71))),
            border: iced::Border {
                radius: iced::border::Radius::from(2),
                ..iced::Border::default()
            },
            ..Default::default()
        }),
        Position::Bottom,
    )
}

pub fn header<'a, T>(txt: T) -> row::Row<'a, Message>
where
    T: IntoFragment<'a> + 'a,
{
    row![text(txt).size(16), rule::horizontal(1),]
        .spacing(5)
        .align_y(Center)
}

pub fn header_text<'a>(txt: String) -> Column<'a, Message> {
    column![text(txt).size(22), rule::horizontal(1)].spacing(2)
}

pub fn category_header<'a, T>(txt: T) -> text::Text<'a>
where
    T: IntoFragment<'a> + 'a,
{
    text(txt).size(14).style(|t: &Theme| {
        let palette = t.palette();
        let text_color = palette.text.scale_alpha(0.7);

        let mut style = text::Style::default();
        style.color = Some(text_color);

        style
    })
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
            }.scale_alpha(0.08);

            container::Style::default().background(background_color).border(Border {
                color: border_color,
                width: 1.,
                radius: 5.0.into(),
            })
        })
}

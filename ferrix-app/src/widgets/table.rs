/* table.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
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

//! Custom table widget

use iced::widget::button;
use iced::widget::text::IntoFragment;
use iced::{
    Element, Length,
    widget::{table, text},
};

use crate::messages::ButtonsMessage;
use crate::{Message, fl, widgets::link_button};

#[derive(Debug, Clone)]
pub struct InfoRow<V> {
    pub param_header: String,
    pub value: Option<V>,
}

impl<V> InfoRow<V> {
    pub fn new<P>(param: P, value: Option<V>) -> Self
    where
        P: Into<String>,
        V: ToString,
    {
        Self {
            param_header: param.into(),
            value,
        }
    }
}

pub fn kv_info_table<'a, V>(rows: Vec<InfoRow<V>>) -> Element<'a, Message>
where
    V: ToString + Clone + 'a,
{
    let columns = [
        table::column(hdr_name(fl!("hdr-param")), |row: InfoRow<V>| {
            text(row.param_header)
        }),
        table::column(hdr_name(fl!("hdr-value")), |row: InfoRow<V>| {
            text_fmt_val(row.value)
        })
        .width(Length::Fill),
    ];

    table(columns, rows).padding(2).width(Length::Fill).into()
}

pub fn text_fmt_val<'a, V>(val: Option<V>) -> Element<'a, Message>
where
    V: ToString + 'a,
{
    match val {
        Some(val) if !val.to_string().is_empty() && !val.to_string().starts_with("http") => {
            button(text(val.to_string()))
                .style(button::text)
                .padding(0)
                .on_press(Message::Buttons(ButtonsMessage::CopyButtonPressed(
                    val.to_string(),
                )))
                .into()
        }
        Some(val) if !val.to_string().is_empty() && val.to_string().contains("http") => {
            link_button(val.to_string(), val.to_string()).into()
        }
        Some(_) => text("N/A").into(),
        None => text("").into(),
    }
}

pub fn hdr_name<'a, S: IntoFragment<'a>>(s: S) -> text::Text<'a> {
    text(s).style(text::secondary)
}

pub fn fmt_val<T>(val: Option<T>) -> Option<String>
where
    T: ToString + Copy,
{
    match val {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

pub fn fmt_vec<T>(val: &Option<Vec<T>>) -> Option<String>
where
    T: ToString + Clone,
{
    match val {
        Some(val) => {
            let mut s = String::new();
            for i in val {
                s = format!("{s}{} ", i.to_string());
            }
            Some(s)
        }
        None => None,
    }
}

pub fn fmt_bool(val: Option<bool>) -> Option<String> {
    match val {
        Some(val) => match val {
            true => Some(fl!("bool-true")),
            false => Some(fl!("bool-false")),
        },
        None => None,
    }
}

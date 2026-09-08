/* keyboard_n_mouse.rs
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

use iced::{
    Event, Task,
    keyboard::{Event as Kevent, Key, Modifiers, key},
    widget::{
        Id,
        operation::{self, AbsoluteOffset, RelativeOffset},
    },
};

use crate::{ferrix::Ferrix, message::Message, pages::PageVariant};

#[derive(Debug, Clone)]
pub enum KeyboardAndMouse {
    Event(Event),
    LinkButtonPressed(String),
    CopyButtonPressed(String),
}

impl KeyboardAndMouse {
    pub fn update<'a>(self, fx: &'a mut Ferrix) -> Task<Message> {
        match self {
            Self::Event(event) => match event {
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowDown),
                    modifiers,
                    ..
                }) if !modifiers.control() => scroll_down(fx.active_page, modifiers),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowUp),
                    modifiers,
                    ..
                }) if !modifiers.control() => scroll_up(fx.active_page, modifiers),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowDown),
                    modifiers,
                    ..
                }) if modifiers.control() => scroll_sidebar_down(),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::ArrowUp),
                    modifiers,
                    ..
                }) if modifiers.control() => scroll_sidebar_up(),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::PageDown),
                    ..
                }) => snap_down(fx.active_page),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::PageUp),
                    ..
                }) => snap_up(fx.active_page),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F1),
                    ..
                }) => fx.select_page(PageVariant::ProgramAbout),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F2),
                    ..
                }) => fx.select_page(PageVariant::ExportData),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::F9),
                    ..
                }) => fx.select_page(PageVariant::ProgramSettings),
                Event::Keyboard(Kevent::KeyPressed {
                    key: Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) if modifiers.control() => fx.select_page(if modifiers.shift() {
                    fx.active_page.prev_page()
                } else {
                    fx.active_page.next_page()
                }),
                _ => Task::none(),
            },
            Self::LinkButtonPressed(_url) => {
                todo!()
            }
            Self::CopyButtonPressed(text) => iced::clipboard::write(text),
        }
    }
}

const SCROLL_UP: f32 = -20.;
const SCROLL_DOWN: f32 = 20.;

fn get_id(page: PageVariant, m: Modifiers) -> Id {
    if m.shift() {
        page.scrolled_id().unwrap_or(Id::new(""))
    } else {
        page.id()
    }
}

fn scroll_up(page: PageVariant, m: Modifiers) -> Task<Message> {
    let id = get_id(page, m);
    operation::scroll_by(
        id,
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_UP,
        },
    )
}

fn scroll_down(page: PageVariant, m: Modifiers) -> Task<Message> {
    let id = get_id(page, m);
    operation::scroll_by(
        id,
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_DOWN,
        },
    )
}

fn scroll_sidebar_up() -> Task<Message> {
    operation::scroll_by(
        Id::new("sidebar"),
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_UP,
        },
    )
}

fn scroll_sidebar_down() -> Task<Message> {
    operation::scroll_by(
        Id::new("sidebar"),
        AbsoluteOffset {
            x: 0.,
            y: SCROLL_DOWN,
        },
    )
}

fn snap_up(page: PageVariant) -> Task<Message> {
    let id = page.id();
    operation::snap_to(id, RelativeOffset::START)
}

fn snap_down(page: PageVariant) -> Task<Message> {
    let id = page.id();
    operation::snap_to(id, RelativeOffset::END)
}

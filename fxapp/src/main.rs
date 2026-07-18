/* main.rs
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

pub mod ferrix;
pub mod i18n;
pub mod icons;
pub mod message;
pub mod navigation;
pub mod pages;
pub mod widgets;

use ferrix::Ferrix;
use iced::{Size, window::Settings};

fn main() -> iced::Result {
    iced::application(Ferrix::new, Ferrix::message, Ferrix::view)
        .subscription(Ferrix::subscription)
        .settings(iced::Settings {
            default_text_size: iced::Pixels(12.),
            ..Default::default()
        })
        .window(Settings {
            min_size: Some(Size {
                width: 780.,
                height: 470.,
            }),
            ..Default::default()
        })
        .window_size((780., 470.))
        .antialiasing(true)
        .theme(Ferrix::theme)
        .title("Ferrix System Monitor")
        .run()
}

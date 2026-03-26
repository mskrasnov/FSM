/* icons.rs
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

//! Hardcoded icons bytes

pub const ERROR_ICON: &[u8] = include_bytes!("../data/icons/actions/ferrix-error.svg");
pub const SETTINGS_ICON: &[u8] = include_bytes!("../data/icons/actions/ferrix-settings.svg");
pub const ABOUT_ICON: &[u8] = include_bytes!("../data/icons/actions/ferrix-about.svg");
pub const EXPORT_ICON: &[u8] = include_bytes!("../data/icons/actions/ferrix-export.svg");
pub const FERRIX_ICON: &[u8] = include_bytes!("../data/com.mskrasnov.Ferrix.svg");

pub fn get_svg_bytes<'a>(icon_name: &'a str) -> &'static [u8] {
    match icon_name {
        "about" => ABOUT_ICON,
        "error" => ERROR_ICON,
        "export" => EXPORT_ICON,
        "settings" => SETTINGS_ICON,
        _ => &[],
    }
}

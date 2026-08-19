/* icons.rs
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

//! Icons embedded to the binary

pub const ERROR_ICON: &[u8] =
    include_bytes!("../data/icons/hicolor/symbolic/actions/ferrix-error.svg");
pub const SETTINGS_ICON: &[u8] =
    include_bytes!("../data/icons/hicolor/symbolic/actions/ferrix-settings.svg");
pub const ABOUT_ICON: &[u8] =
    include_bytes!("../data/icons/hicolor/symbolic/actions/ferrix-about.svg");
pub const EXPORT_ICON: &[u8] =
    include_bytes!("../data/icons/hicolor/symbolic/actions/ferrix-export.svg");
pub const FERRIX_ICON: &[u8] =
    include_bytes!("../data/icons/hicolor/scalable/apps/com.mskrasnov.Ferrix.svg");

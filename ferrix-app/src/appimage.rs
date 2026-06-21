/* appimage.rs
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

//! Copy `ferrix-polkit` into `~`

use std::{
    env::var,
    fs::{copy, create_dir_all},
    path::{Path, PathBuf},
};

use anyhow::Result;

// #[cfg(feature = "appimage")]
const FX_POLKIT_BINDIR: &'static str = ".local/bin/";

fn get_home_path() -> String {
    var("HOME").unwrap_or("./".to_string())
}

fn get_appimage_rundir_path() -> String {
    var("APPDIR").unwrap_or("./".to_string())
}

fn get_fx_polkit_appimage_path() -> PathBuf {
    Path::new(&get_appimage_rundir_path())
        .join("usr/")
        .join("bin/")
        .join("ferrix-polkit")
}

fn get_fx_polkit_destination_path() -> PathBuf {
    Path::new(&get_home_path()).join(FX_POLKIT_BINDIR)
}

fn check_dest(path: &PathBuf) -> bool {
    path.is_file()
}

pub fn copy_fx_polkit() -> Result<()> {
    let in_pth = get_fx_polkit_appimage_path();
    let dest_pth = get_fx_polkit_destination_path();

    if !dest_pth.exists() {
        create_dir_all(&dest_pth)?;
    }
    let dest_pth = dest_pth.join("ferrix-polkit");

    if !check_dest(&dest_pth) {
        copy(in_pth, dest_pth)?;
    }
    Ok(())
}

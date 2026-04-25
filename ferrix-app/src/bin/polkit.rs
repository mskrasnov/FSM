/* polkit.rs
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

use ferrix_app::kernel::KResult;
use ferrix_data::dmi::DMIData;
use std::env;

fn print_error_mode() {
    let json = serde_json::json!({
        "error": "The ferrix-polkit operating mode is not specified!",
    });
    eprintln!("{}", json.to_string());
    std::process::exit(1);
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next();

    match mode {
        Some(mode) => {
            if &mode == "dmi" {
                let data = DMIData::new();
                println!("{}", data.to_json().unwrap());
            } else if &mode == "kmods" {
                let data = KResult::new();
                println!("{}", data.to_json().unwrap());
            } else {
                print_error_mode();
            }
        }
        None => print_error_mode(),
    }
}

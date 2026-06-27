/* log.rs
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

//! Logging functions

use std::path::PathBuf;

pub fn log_path() -> PathBuf {
    crate::utils::get_home().join(".fsm.log")
}

#[macro_export]
macro_rules! log {
    ($path:expr, $fmt:expr) => {{
        use std::fs::OpenOptions;
        use std::io::Write;

        eprintln!("FSM: {}", $fmt);

        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open($path)
            .expect(format!("Failed to open log file (path: {:?})!", $path).as_str());

        writeln!(f, $fmt).expect("Failed to write to the log file!");
    }};

    ($path:expr, $fmt:expr, $($arg:tt)*) => {{
        use std::fs::OpenOptions;
        use std::io::Write;

        let message = format!($fmt, $($arg)*);
        eprintln!("{message}");

        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open($path)
            .expect(format!("Failed to open log file (path: {:?})!", $path).as_str());

        writeln!(f, $fmt, $($arg)*)
            .expect("Failed to write to the log file!");
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn log_with_single_arg_test() {
        log!("test.log", "test string\n");
    }

    #[test]
    fn log_with_many_args_test() {
        log!(
            "test.log",
            "test string with args: {} {:.2}\n",
            "aaa",
            3.141592
        );
    }
}

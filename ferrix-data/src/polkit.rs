/* polkit.rs
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

//! Run tasks as `root`

use crate::load_state::LoadState;
use std::{env, path::Path, process::Command, sync::LazyLock};

pub static PATH: LazyLock<Vec<String>> = LazyLock::new(|| path());

/// Get directories list from the `PATH` environment variable
pub fn path() -> Vec<String> {
    env::var("PATH")
        .ok()
        .and_then(|path| Some(path.split(':').map(|p| p.to_string()).collect::<Vec<_>>()))
        .unwrap_or(vec!["/usr/bin".to_string()])
}

/// Returns the name of authentification app
pub fn auth_app() -> Option<String> {
    let apps = ["pkexec", "gksudo"];
    let bin_dirs = &PATH;

    for app in apps {
        for dir in bin_dirs.as_slice() {
            let path = Path::new(dir).join(app);
            if path.exists() {
                return Some(path.display().to_string());
            }
        }
    }
    None
}

/// Returns the `ferrix-polkit` application path
pub fn fx_polkit_app() -> Option<String> {
    let bin_dirs = &PATH;
    for dir in bin_dirs.as_slice() {
        let path = Path::new(dir).join("ferrix-polkit");
        if path.exists() {
            return Some(path.display().to_string());
        }
    }
    None
}

pub async fn get_data(data_type: String) -> LoadState<serde_json::Value> {
    let auth_app = match auth_app() {
        Some(app) => app,
        None => return LoadState::Error("No authentication software found".to_string()),
    };
    let fx_app = match fx_polkit_app() {
        Some(app) => app,
        None => return LoadState::Error("No `ferrix-polkit` program found".to_string()),
    };

    let output = tokio::task::spawn_blocking(move || {
        Command::new(auth_app)
            .arg(fx_app)
            .arg(data_type)
            .output()
            .unwrap() // NOTE: shitcode?
    })
    .await;

    if let Err(why) = output {
        return LoadState::Error(why.to_string());
    }
    let output = output.unwrap();

    if output.status.code().unwrap_or(0) != 0 {
        return LoadState::Error(format!(
            "[ferrix-polkit] Non-zero return code:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str = String::from_utf8_lossy(&output.stdout).to_string();
    match serde_json::from_str(&json_str) {
        Ok(data) => LoadState::Loaded(data),
        Err(why) => LoadState::Error(format!("Failed to parse `ferrix-polkit` output:\n{why}")),
    }
}

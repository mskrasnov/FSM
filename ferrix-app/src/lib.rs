/* lib.rs
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

pub mod export;
pub mod i18n;
pub mod icons;
pub mod load_state;
// pub mod modals;
pub mod pages;
pub mod styles;
pub mod utils;
pub mod widgets;

pub mod dmi;
pub mod kernel;

// REFACTORED MODULES
pub mod ferrix;
pub mod messages;
pub mod settings;
pub mod sidebar;
pub mod subscription;

use messages::*;

pub use load_state::DataLoadingState;
pub use pages::*;

use serde::Serialize;

use anyhow::Result;
use ferrix_lib::sys::{LoadAVG, Uptime, get_current_desktop, get_env_vars, get_hostname, get_lang};

const SETTINGS_PATH: &str = "./ferrix.conf";

#[derive(Debug, Clone, Serialize)]
pub struct System {
    pub hostname: Option<String>,
    pub loadavg: Option<LoadAVG>,
    pub uptime: Option<Uptime>,
    pub desktop: Option<String>,
    pub language: Option<String>,
    pub env_vars: Vec<(String, String)>,
}

impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            hostname: get_hostname(),
            loadavg: Some(LoadAVG::new()?),
            uptime: Some(Uptime::new()?),
            desktop: get_current_desktop(),
            language: get_lang(),
            env_vars: get_env_vars(),
        })
    }
}

impl Default for System {
    fn default() -> Self {
        Self {
            hostname: Some("unknown-host".to_string()),
            loadavg: None,
            uptime: None,
            desktop: Some("Unknown DE".to_string()),
            language: Some("Unknown locale".to_string()),
            env_vars: Vec::new(),
        }
    }
}

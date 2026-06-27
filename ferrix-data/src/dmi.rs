/* dmi.rs
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

//! DMI Service Provider

use crate::{
    FromJson,
    load_state::{LoadState, ToLoadState},
    polkit::*,
};
use anyhow::Result;
use ferrix_lib::dmi::{Baseboard, Bios, Chassis, Processor};
use serde::{Deserialize, Serialize};

pub async fn get_dmi_data() -> LoadState<DMIData> {
    let json = get_data("dmi".to_string()).await;
    match json {
        LoadState::Loaded(json) => DMIData::from_json(json).to_load_state(),
        LoadState::Error(why) => LoadState::Error(why),
        _ => LoadState::Loading,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DMIData {
    pub bios: LoadState<Bios>,
    pub baseboard: LoadState<Baseboard>,
    pub chassis: LoadState<Chassis>,
    pub processor: LoadState<Processor>,
    // pub memory_devices: LoadState<MemoryDevices>,
}

impl DMIData {
    pub fn new() -> Self {
        Self {
            bios: Bios::new().to_load_state(),
            baseboard: Baseboard::new().to_load_state(),
            chassis: Chassis::new().to_load_state(),
            processor: Processor::new().to_load_state(),
            // memory_devices: MemoryDevices::new().to_load_state(),
        }
    }

    pub fn to_json(&self) -> Result<String> {
        let contents = serde_json::to_string(&self)?;
        Ok(contents)
    }
}

impl FromJson for DMIData {}

/* battery.rs
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

//! Get information about notebook's battery

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use crate::traits::ToJson;

/// Information about all installed batteries
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BatInfo {
    pub bats: Vec<Battery>,
}

impl BatInfo {
    pub fn new() -> Result<Self> {
        let mut bats = Vec::new();
        let base_path = Path::new("/sys/class/power_supply/");

        let dir_contents = read_dir(base_path)?;
        for dir in dir_contents {
            let dir = dir?.path();
            let bat_path = dir.join("type");
            let bat_type = read_to_string(&bat_path)?;
            if bat_type.trim() == "Battery" {
                let uevent_path = dir.join("uevent");
                if uevent_path.is_file() {
                    bats.push(Battery::new(uevent_path)?);
                }
            } else {
                continue;
            }
        }
        Ok(Self { bats })
    }
}

/// Information from the `uevent` file
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Battery {
    pub name: Option<String>,
    pub status: Option<Status>,
    pub technology: Option<String>,
    pub cycle_count: Option<usize>,
    pub voltage_min_design: Option<f32>,
    pub voltage_now: Option<f32>,
    pub power_now: Option<f32>,
    pub energy_full_design: Option<f32>,
    pub energy_full: Option<f32>,
    pub energy_now: Option<f32>,
    pub capacity: Option<u8>,
    pub capacity_level: Option<Level>,
    pub model_name: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub charge_types: Option<String>,
    pub health: Option<f32>,
    pub estimated_time: Option<f32>,
    pub charge_time: Option<f32>,
}

impl ToJson for Battery {}

impl Battery {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = read_to_string(&path)?;
        let lines = contents.lines().map(|line| line.trim());
        let mut bat = Battery::default();

        for line in lines {
            let mut chunks = line.split('=');
            match (chunks.next(), chunks.next()) {
                (Some(key), Some(val)) => parse_chunks(&mut bat, key, val),
                _ => continue,
            }
        }
        calculate_time(&mut bat);
        calculate_health(&mut bat);
        polish_values(&mut bat);

        Ok(bat)
    }
}

fn parse_chunks(bat: &mut Battery, key: &str, val: &str) {
    let val = val.trim();
    match key {
        "POWER_SUPPLY_NAME" => bat.name = Some(val.to_string()),
        "POWER_SUPPLY_STATUS" => bat.status = Some(Status::from(val)),
        "POWER_SUPPLY_TECHNOLOGY" => bat.technology = Some(val.to_string()),
        "POWER_SUPPLY_CYCLE_COUNT" => bat.cycle_count = val.parse().ok(),
        "POWER_SUPPLY_VOLTAGE_MIN_DESIGN" => bat.voltage_min_design = val.parse().ok(),
        "POWER_SUPPLY_VOLTAGE_NOW" => bat.voltage_now = val.parse().ok(),
        "POWER_SUPPLY_POWER_NOW" => bat.power_now = val.parse().ok(),
        "POWER_SUPPLY_ENERGY_FULL_DESIGN" => bat.energy_full_design = val.parse().ok(),
        "POWER_SUPPLY_ENERGY_FULL" => bat.energy_full = val.parse().ok(),
        "POWER_SUPPLY_ENERGY_NOW" => bat.energy_now = val.parse().ok(),
        "POWER_SUPPLY_CAPACITY" => bat.capacity = val.parse().ok(),
        "POWER_SUPPLY_CAPACITY_LEVEL" => bat.capacity_level = Some(Level::from(val)),
        "POWER_SUPPLY_MODEL_NAME" => bat.model_name = Some(val.to_string()),
        "POWER_SUPPLY_MANUFACTURER" => bat.manufacturer = Some(val.to_string()),
        "POWER_SUPPLY_SERIAL_NUMBER" => bat.serial_number = Some(val.to_string()),
        "POWER_SUPPLY_CHARGE_TYPES" => bat.charge_types = Some(val.to_string()),
        _ => {}
    }
}

fn polish_values(bat: &mut Battery) {
    if let Some(vmd) = bat.voltage_min_design {
        bat.voltage_min_design = Some(vmd / 1_000_000.);
    }
    if let Some(pn) = bat.power_now {
        bat.power_now = Some(pn / 1_000_000.);
    }
    if let Some(vn) = bat.voltage_now {
        bat.voltage_now = Some(vn / 1_000_000.);
    }
    if let Some(efd) = bat.energy_full_design {
        bat.energy_full_design = Some(efd / 1_000_000.);
    }
    if let Some(ef) = bat.energy_full {
        bat.energy_full = Some(ef / 1_000_000.);
    }
    if let Some(en) = bat.energy_now {
        bat.energy_now = Some(en / 1_000_000.);
    }
}

fn calculate_health(bat: &mut Battery) {
    if bat.energy_full.is_some() && bat.energy_full_design.is_some() {
        let (energy_full, energy_full_design) =
            (bat.energy_full.unwrap(), bat.energy_full_design.unwrap());
        bat.health = Some((energy_full / energy_full_design * 100.).min(100.));
    }
}

fn calculate_time(bat: &mut Battery) {
    bat.estimated_time = match (
        bat.status.as_ref(),
        bat.energy_now,
        bat.energy_full,
        bat.power_now,
    ) {
        (Some(Status::Discharging) | Some(Status::NotCharging), Some(now), _, Some(p))
            if p > 0.001 =>
        {
            Some((now / p).clamp(0., 999.))
        }
        (Some(Status::Charging), Some(now), Some(full), Some(p)) if p > 0.001 => {
            Some(((full - now) / p).clamp(0., 999.))
        }
        _ => None,
    }
}

/// Charging status
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Status {
    Full,
    Discharging,
    Charging,
    NotCharging,
    Unknown(String),
    #[default]
    None,
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "Full" => Self::Full,
            "Discharging" => Self::Discharging,
            "Charging" => Self::Charging,
            "Not charging" => Self::NotCharging,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

/// Capacity level
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum Level {
    Full,
    Normal,
    High,
    Low,
    Critical,
    Unknown(String),
    #[default]
    None,
}

impl From<&str> for Level {
    fn from(value: &str) -> Self {
        match value {
            "Full" => Self::Full,
            "Normal" => Self::Normal,
            "High" => Self::High,
            "Low" => Self::Low,
            "Critical" => Self::Critical,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

/* firmware.rs
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

//! Get firmware settings (only for modern systems with UEFI)

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Firmware {
    pub attributes: AttributesList,
}

impl Firmware {
    pub fn new() -> Result<Self> {
        let attributes = AttributesList::read()?;
        Ok(Self { attributes })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributesList {
    pub driver_name: String,
    pub attributes: Vec<Attribute>,
}

impl AttributesList {
    const FIRMWARE_DIR: &'static str = "/sys/class/firmware-attributes/";

    pub fn read() -> Result<Self> {
        let mut firmware_dir_contents = read_dir(Self::FIRMWARE_DIR)?;
        let driver_dir = firmware_dir_contents
            .next()
            .ok_or_else(|| anyhow!("No `firmware-attributes` directory found."))??;

        let driver_name = os_str_into_str(driver_dir.file_name())?;
        let attributes_dir_contents = read_dir(driver_dir.path())?;
        let mut attributes = vec![];

        for attr in attributes_dir_contents {
            let attr = attr?;
            let metadata = attr.metadata()?;
            let fname = os_str_into_str(attr.file_name())?;

            if !metadata.is_dir() || &fname != "attributes" {
                continue;
            }

            let attribute = Self::read_attributes(attr.path())?;
            attributes = attribute; // NOTE: shitcode?
            break; // NOTE: shitcode?
        }

        Ok(Self {
            driver_name,
            attributes,
        })
    }

    fn read_attributes<P>(dir: P) -> Result<Vec<Attribute>>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let mut attrs = vec![];
        let dir_contents = read_dir(dir)?;

        for d in dir_contents {
            let d = d?;
            if !d.metadata()?.is_dir() {
                continue;
            }
            let attribute = Attribute::read_dir(d.path())?;
            attrs.push(attribute);
        }
        attrs.sort_by_key(|key| key.display_name.clone()); // NOTE: SHITCODE

        Ok(attrs)
    }
}

fn os_str_into_str(os_str: OsString) -> Result<String> {
    let s = os_str
        .into_string()
        .map_err(|err| anyhow!("Failed to convert directory name into the string: {err:?}"))?;
    Ok(s)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub current_value: String,
    pub display_name: String,
    pub possible_values: String,
    pub param_type: String,
}

impl Attribute {
    pub fn read_dir<P>(dir: P) -> Result<Self>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        // NOTE: We assume that we receive a DIRECTORY as input
        let dir_contents = read_dir(dir)?;

        let mut current_value = String::new();
        let mut display_name = String::new();
        let mut possible_values = String::new();
        let mut param_type = String::new();

        for d in dir_contents {
            let d = d?;
            if !d.metadata()?.is_file() {
                continue;
            }
            let name = os_str_into_str(d.file_name())?;

            let read = || -> Result<String> {
                let s = read_to_string(d.path())?;
                Ok(s.trim().to_string())
            };

            match &name as &str {
                "display_name" => display_name = read()?,
                "current_value" => current_value = read()?,
                "possible_values" => possible_values = read()?,
                "type" => param_type = read()?,
                _ => {}
            }
        }

        Ok(Self {
            current_value,
            display_name,
            possible_values,
            param_type,
        })
    }
}

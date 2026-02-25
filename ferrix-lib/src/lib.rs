/* lib.rs
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

//! ferrix-lib is a library for obtaining information about the
//! hardware and software of a PC running Linux OS.
//!
//! ## Examples
//! Get all information about hardware and software (NOTE: needed
//! `root` permissions!):
//! ```no-test
//! use ferrix_lib::Ferrix;
//!
//! let data = Ferrix::new()?; // get all data
//!
//! let json_str = data.to_json()?; // get machine-readable JSON from this data
//! let pjson_str = data.to_json_pretty()?; // get human-readable JSON
//! let xml_str = data.to_xml()?; // get XML
//! ```
//!
//! Get information about CPU:
//! ```no-test
//! use ferrix_lib::cpu::Processors;
//! let proc = Processors::new()?;
//!
//! let json_str = data.to_json()?;
//! let pjson_str = data.to_json_pretty()?;
//! ```

pub mod battery;
pub mod cpu;
pub mod cpu_freq;
pub mod dmi;
pub mod drm;
pub mod init;
pub mod net;
pub mod parts;
pub mod ram;
pub mod soft;
pub mod sys;
pub mod vmstat;
pub mod vulnerabilities;

pub mod traits;
pub mod utils;

use crate::traits::ToPlainText;
use anyhow::Result;
use serde::Serialize;

pub const FX_LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize)]
pub struct Ferrix {
    pub cpu: cpu::Processors,
    pub ram: ram::RAM,
    pub swaps: ram::Swaps,
    pub dmi: dmi::DMITable,
    pub drm: drm::Video,
    pub sys: sys::Sys,
    pub init: init::SystemdServices,
}

impl Ferrix {
    pub async fn new() -> Result<Self> {
        let conn = zbus::Connection::system().await?;
        Ok(Self {
            cpu: cpu::Processors::new()?,
            ram: ram::RAM::new()?,
            swaps: ram::Swaps::new()?,
            dmi: dmi::DMITable::new()?,
            drm: drm::Video::new()?,
            sys: sys::Sys::new()?,
            init: init::SystemdServices::new_from_connection(&conn).await?,
        })
    }

    fn _update(&mut self) -> Result<()> {
        self.cpu = cpu::Processors::new()?;
        self.ram = ram::RAM::new()?;
        self.swaps = ram::Swaps::new()?;
        self.sys.update()?;

        Ok(())
    }

    pub async fn update(&mut self, conn: &zbus::Connection) -> Result<()> {
        self._update()?;
        self.init = init::SystemdServices::new_from_connection(&conn).await?;
        Ok(())
    }

    pub async fn update1(&mut self) -> Result<()> {
        self._update()?;
        let conn = zbus::Connection::system().await?;
        self.init = init::SystemdServices::new_from_connection(&conn).await?;
        Ok(())
    }

    /// Performs serialization of structure data in JSON.
    ///
    /// The returned value will be a SINGLE LINE of JSON data
    /// intended for reading by third-party software or for
    /// transmission over the network.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Performs serialization in "pretty" JSON
    ///
    /// JSON will contain unnecessary newline transitions and spaces
    /// to visually separate the blocks. It is well suited for human
    /// reading and analysis.
    pub fn to_json_pretty(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }

    /// Performs data serialization in XML format
    pub fn to_xml(&self) -> Result<String> {
        let xml = XMLData::from(self);
        let data = XMLFerrixData::from(&xml);
        data.to_xml()
    }
}

impl ToPlainText for Ferrix {
    fn to_plain(&self) -> String {
        let mut s = format!("");
        s += &self.cpu.to_plain();
        s += &self.init.to_plain();

        s
    }
}

#[derive(Serialize)]
struct XMLFerrixData<'a> {
    data: &'a XMLData<'a>,
}

#[derive(Serialize)]
struct XMLData<'a> {
    cpu: &'a cpu::Processors,
    ram: &'a ram::RAM,
    dmi: dmi::DMITableXml<'a>,
    sys: &'a sys::Sys,
    init: &'a init::SystemdServices,
}

impl<'a> From<&'a Ferrix> for XMLData<'a> {
    fn from(value: &'a Ferrix) -> Self {
        Self {
            cpu: &value.cpu,
            ram: &value.ram,
            dmi: dmi::DMITableXml::from(&value.dmi),
            sys: &value.sys,
            init: &value.init,
        }
    }
}

impl<'a> XMLFerrixData<'a> {
    fn to_xml(&self) -> Result<String> {
        Ok(xml_serde::to_string(&self)?)
    }
}

impl<'a> From<&'a XMLData<'a>> for XMLFerrixData<'a> {
    fn from(value: &'a XMLData) -> Self {
        Self { data: value }
    }
}

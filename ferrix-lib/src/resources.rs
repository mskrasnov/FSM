/* resources.rs
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

//! Resources and addresses

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::traits::ToJson;

/// Resources and addresses
///
/// > **NOTE!** Needs `root` permissions to get addresses!
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resources {
    pub io_ports: Vec<Resource>,
    pub io_mem: Vec<Resource>,
    pub dma: Vec<Resource>,
}

impl ToJson for Resources {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resource {
    pub address: String,
    pub title: String,
}

impl Resources {
    pub fn new() -> Result<Self> {
        let ports = read_to_string("/proc/ioports")?;
        let ports = ports.lines();

        let mem = read_to_string("/proc/iomem")?;
        let mem = mem.lines();

        let dma = read_to_string("/proc/dma")?;
        let dma = dma.lines();

        let mut io_ports = Vec::new();
        let mut io_mem = Vec::new();
        let mut io_dma = Vec::new();

        for line in ports {
            io_ports.push(Resource::try_from(line)?);
        }
        io_ports.shrink_to_fit();
        for line in mem {
            io_mem.push(Resource::try_from(line)?);
        }
        io_mem.shrink_to_fit();
        for line in dma {
            io_dma.push(Resource::try_from(line)?);
        }
        io_dma.shrink_to_fit();

        Ok(Self {
            io_ports,
            io_mem,
            dma: io_dma,
        })
    }
}

impl TryFrom<&str> for Resource {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut v = value.split(':');
        match (v.next(), v.next()) {
            (Some(addr), Some(value)) => Ok(Self {
                address: addr.trim_end().to_string(),
                title: value.trim().to_string(),
            }),
            _ => Err(anyhow!("Unknown string format: \"{value}\"")),
        }
    }
}

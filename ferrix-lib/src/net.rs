/* net.rs
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

//! Network statistics

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

use crate::traits::ToJson;

const NET_DIR: &str = "/sys/class/net/";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Networks {
    pub networks: Vec<Network>,
}

impl Networks {
    pub fn new() -> Result<Self> {
        let net_dirs = read_dir(NET_DIR)?;
        let mut networks = Vec::new();
        for dir in net_dirs {
            let dir = dir?.path();
            networks.push(Network::new(&dir)?);
        }
        Ok(Self { networks })
    }
}

impl ToJson for Networks {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Network {
    pub name: String,
    pub address: String,
    pub broadcast: String,
    pub mtu: u64,
    pub operstate: String,
}

impl Network {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let name = path.strip_prefix(NET_DIR)?.display().to_string();
        let address = read_to_string(path.join("address"))
            .and_then(|address| Ok(address.trim().to_string()))?;
        let broadcast = read_to_string(path.join("broadcast"))
            .and_then(|broadcast| Ok(broadcast.trim().to_string()))?;
        let mtu = read_to_string(path.join("mtu"))
            .and_then(|mtu| Ok(mtu.trim().parse::<u64>().unwrap_or(0)))?;
        let operstate = read_to_string(path.join("operstate"))
            .and_then(|opstate| Ok(opstate.trim().to_string()))?;

        Ok(Self {
            name,
            address,
            broadcast,
            mtu,
            operstate,
        })
    }
}

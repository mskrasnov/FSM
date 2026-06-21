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

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
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
    pub statistics: Statistics,
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
        let statistics = Statistics::new(&name)?;

        Ok(Self {
            name,
            address,
            broadcast,
            mtu,
            operstate,
            statistics,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Statistics {
    pub collisions: u64,
    pub multicast: u64,

    pub rx_bytes: u64,
    pub rx_compressed: u64,
    pub rx_crc_errors: u64,
    pub rx_dropped: u64,
    pub rx_errors: u64,
    pub rx_fifo_errors: u64,
    pub rx_frame_errors: u64,
    pub rx_length_erorrs: u64,
    pub rx_missed_errors: u64,
    pub rx_nohandler: u64,
    pub rx_over_errors: u64,
    pub rx_packets: u64,

    pub tx_aborted_errors: u64,
    pub tx_bytes: u64,
    pub tx_carrier_errors: u64,
    pub tx_compressed: u64,
    pub tx_dropped: u64,
    pub tx_errors: u64,
    pub tx_fifo_errors: u64,
    pub tx_heartbeat_errors: u64,
    pub tx_packets: u64,
    pub tx_window_errors: u64,
}

impl Statistics {
    pub fn new(interface: &str) -> Result<Self> {
        let dir = Path::new(NET_DIR).join(interface).join("statistics");
        if !dir.is_dir() {
            return Err(anyhow!(
                "Failed to open '{}' directory: not found",
                dir.display(),
            ));
        }

        let read = |file: &str| -> Result<u64> {
            let file = dir.join(file);
            let contents = read_to_string(&file)
                .map_err(|err| anyhow!("Failed to read '{}' file: {err}", file.display()))?;

            contents
                .trim()
                .parse::<u64>()
                .map_err(|err| anyhow!("Failed to parse '{interface}' value: {err}"))
        };

        let collisions = read("collisions")?;
        let multicast = read("multicast")?;

        let rx_bytes = read("rx_bytes")?;
        let rx_compressed = read("rx_compressed")?;
        let rx_crc_errors = read("rx_crc_errors")?;
        let rx_dropped = read("rx_dropped")?;
        let rx_errors = read("rx_errors")?;
        let rx_fifo_errors = read("rx_fifo_errors")?;
        let rx_frame_errors = read("rx_frame_errors")?;
        let rx_length_erorrs = read("rx_length_errors")?;
        let rx_missed_errors = read("rx_missed_errors")?;
        let rx_nohandler = read("rx_nohandler")?;
        let rx_over_errors = read("rx_over_errors")?;
        let rx_packets = read("rx_packets")?;

        let tx_aborted_errors = read("tx_aborted_errors")?;
        let tx_bytes = read("tx_bytes")?;
        let tx_carrier_errors = read("tx_carrier_errors")?;
        let tx_compressed = read("tx_compressed")?;
        let tx_dropped = read("tx_dropped")?;
        let tx_errors = read("tx_errors")?;
        let tx_fifo_errors = read("tx_fifo_errors")?;
        let tx_heartbeat_errors = read("tx_heartbeat_errors")?;
        let tx_packets = read("tx_packets")?;
        let tx_window_errors = read("tx_window_errors")?;

        Ok(Self {
            collisions,
            multicast,
            rx_bytes,
            rx_compressed,
            rx_crc_errors,
            rx_dropped,
            rx_errors,
            rx_fifo_errors,
            rx_frame_errors,
            rx_length_erorrs,
            rx_missed_errors,
            rx_nohandler,
            rx_over_errors,
            rx_packets,
            tx_aborted_errors,
            tx_bytes,
            tx_carrier_errors,
            tx_compressed,
            tx_dropped,
            tx_errors,
            tx_fifo_errors,
            tx_heartbeat_errors,
            tx_packets,
            tx_window_errors,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ARP {
    pub tables: Vec<ARPTable>,
}

impl ToJson for ARP {}

impl ARP {
    pub fn new() -> Result<Self> {
        let contents = read_to_string("/proc/net/arp")?;
        let lines = contents.lines().skip(1);
        let mut tables = vec![];

        for line in lines {
            tables.push(ARPTable::try_from(line)?);
        }
        tables.shrink_to_fit();

        Ok(Self { tables })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ARPTable {
    pub ip_addr: String,
    pub hw_type: String,
    pub flags: String,
    pub hw_addr: String,
    pub mask: String,
    pub device: String,
}

impl TryFrom<&str> for ARPTable {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let chunks = value.split_whitespace().collect::<Vec<_>>();
        if chunks.len() != 6 {
            return Err(anyhow!(
                "ARP Table parsing failed: String '{value}' is incorrect!"
            ));
        }

        let ip_addr = chunks[0].to_string();
        let hw_type = chunks[1].to_string();
        let flags = chunks[2].to_string();
        let hw_addr = chunks[3].to_string();
        let mask = chunks[4].to_string();
        let device = chunks[5].to_string();

        Ok(Self {
            ip_addr,
            hw_type,
            flags,
            hw_addr,
            mask,
            device,
        })
    }
}

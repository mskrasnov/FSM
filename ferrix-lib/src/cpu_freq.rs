/* cpu_freq.rs
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

//! Get information about CPU frequency

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::traits::ToJson;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CpuFreq {
    pub policy: Vec<Policy>,
    pub boost: Option<bool>,
}

const CPU_FREQ_DIR: &str = "/sys/devices/system/cpu/cpufreq/";

impl CpuFreq {
    pub fn new() -> Result<Self> {
        let pth = Path::new(CPU_FREQ_DIR);
        if !pth.exists() {
            return Err(anyhow!(
                "The directory '{CPU_FREQ_DIR}' was not found. Is \
                 your system able to manage CPU frequencies for sure?",
            ));
        }

        let boost = read_to_string(pth.join("boost"))
            .ok()
            .map(|boost| &boost == "1");

        let mut policy = Vec::new();
        for dir in read_dir(CPU_FREQ_DIR)? {
            let dir = dir?;
            let fname = dir.file_name();
            if fname.to_string_lossy().contains("policy") {
                policy.push(Policy::new(fname)?);
            }
        }

        Ok(Self { policy, boost })
    }
}

impl ToJson for CpuFreq {}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Policy {
    /// Maximum frequency from the BIOS
    pub bios_limit: Option<u32>,

    /// Core Performance Boost (only for AMD)
    pub cpb: Option<bool>,

    /// Maximum hardware frequency
    pub cpu_max_freq: Option<u32>,

    /// Minimum hardware frequency
    pub cpu_min_freq: Option<u32>,

    /// Time (nsecs) for transition between frequencies
    pub cpuinfo_transition_latency: Option<bool>,

    /// Available frequencies
    pub scaling_available_frequencies: Option<Vec<u32>>,

    /// Available frequency governors
    pub scaling_available_governors: Option<Vec<String>>,

    /// Current core frequency
    pub scaling_cur_freq: Option<u32>,

    /// Using cpufreq driver
    pub scaling_driver: Option<String>,

    /// Using governor
    pub scaling_governor: Option<String>,

    pub scaling_max_freq: Option<u32>,
    pub scaling_min_freq: Option<u32>,
    pub scaling_setspeed: Option<String>,
}

impl Policy {
    fn get_data<T>(data: Option<String>) -> Option<T>
    where
        T: FromStr,
    {
        data.and_then(|d| d.trim().parse::<T>().ok())
    }

    pub fn new(policy: OsString) -> Result<Self> {
        let dir = Path::new(CPU_FREQ_DIR);
        let tgt = dir.join(policy);
        if !dir.exists() || !tgt.exists() {
            return Err(anyhow!("Directory {} does not exists!", dir.display()));
        }

        let read = |path: &PathBuf, name: &str| read_to_string(path.join(name));
        let get_bool = |num: Option<u8>| num.map(|n| n != 0);

        Ok(Self {
            bios_limit: Self::get_data(read(&tgt, "bios_limit").ok()),
            cpb: get_bool(Self::get_data(read(&tgt, "cpb").ok())),
            cpu_max_freq: Self::get_data(read(&tgt, "cpuinfo_max_freq").ok()),
            cpu_min_freq: Self::get_data(read(&tgt, "cpuinfo_min_freq").ok()),
            cpuinfo_transition_latency: get_bool(
                read(&tgt, "cpuinfo_transition_latency")
                    .map(|d| d.trim().parse::<u8>().unwrap_or(0))
                    .ok(),
            ),
            scaling_available_frequencies: read(&tgt, "scaling_available_frequencies")
                .map(|d| {
                    d.trim()
                        .split_whitespace()
                        .map(|freq| freq.parse::<u32>().ok())
                        .filter(|freq| freq.is_some())
                        .map(|freq| freq.unwrap())
                        .collect::<Vec<_>>()
                })
                .ok(),
            scaling_available_governors: read(&tgt, "scaling_available_governors")
                .map(|d| {
                    d.trim()
                        .split_whitespace()
                        .map(|gov| gov.to_string())
                        .collect::<Vec<_>>()
                })
                .ok(),
            scaling_cur_freq: Self::get_data(read(&tgt, "scaling_cur_freq").ok()),
            scaling_driver: read(&tgt, "scaling_driver")
                .ok()
                .map(|s| s.trim().to_string()),
            scaling_governor: read(&tgt, "scaling_governor")
                .ok()
                .map(|s| s.trim().to_string()),
            scaling_max_freq: Self::get_data(read(&tgt, "scaling_max_freq").ok()),
            scaling_min_freq: Self::get_data(read(&tgt, "scaling_min_freq").ok()),
            scaling_setspeed: read(&tgt, "scaling_setspeed")
                .ok()
                .map(|s| s.trim().to_string()),
        })
    }
}

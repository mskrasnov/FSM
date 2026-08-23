/* cpu.rs
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

//! Get information about installed CPUs
//!
//! Reads information from `/proc/cpuinfo` file
//!
//! ## Example
//! ```
//! use ferrix_lib::cpu::Processors;
//! use ferrix_lib::traits::ToJson;
//!
//! let proc = Processors::new().unwrap();
//! let json = proc.to_json().unwrap();
//! dbg!(json);
//! ```

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::traits::{ToJson, ToPlainText, print_opt_val};
use crate::utils::Size;

/// A structure containing data from the `/proc/cpuinfo` file
#[derive(Debug, Serialize, Clone)]
pub struct Processors {
    /// Information about all core/thread
    pub entries: Vec<CPU>,
}

impl Processors {
    pub fn new() -> Result<Self> {
        Ok(Self {
            entries: read_info()?,
        })
    }
}

impl ToJson for Processors {}
impl ToPlainText for Processors {
    fn to_plain(&self) -> String {
        let mut s = "Information about processors".to_string();
        for proc in &self.entries {
            s += &proc.to_plain();
        }
        s
    }
}

/// A structure with data about each processor core/thread
#[derive(Debug, Serialize, Default, Clone)]
pub struct CPU {
    /// Entry number (index)
    pub processor: Option<usize>,

    /************************ NOTE ***********************
     *   Parameters for x86 and x86_64 architectures     *
     *****************************************************/
    /// Vendor name
    pub vendor_id: Option<String>,

    /// CPU Family ID
    pub cpu_family: Option<u32>,

    /// Model ID
    pub model: Option<u32>,

    /// Model name
    pub model_name: Option<String>,

    /// Stepping
    pub stepping: Option<u32>,

    /// Microcode number (representation as a `String`!)
    pub microcode: Option<String>,

    /// CPU core/thread *current* frequency
    pub cpu_mhz: Option<f32>,

    /// L3 cache size
    pub cache_size: Option<Size>,

    /// Physical ID of CPU core/thread
    pub physical_id: Option<u32>,

    /// Siblings
    pub siblings: Option<u32>,

    /// Core ID
    pub core_id: Option<u32>,

    /// CPU cores count
    pub cpu_cores: Option<u32>,

    /// APIC ID
    pub apicid: Option<u32>,

    /// Initial APIC ID
    pub initial_apicid: Option<u32>,

    /// Is FPU exists?
    pub fpu: Option<bool>,

    pub fpu_exception: Option<bool>,
    pub cpuid_level: Option<u32>,
    pub wp: Option<bool>,
    pub flags: Option<Vec<String>>,
    pub bugs: Option<Vec<String>>,
    pub bogomips: Option<f64>,
    pub clflush_size: Option<u32>,
    pub cache_alignment: Option<u32>,
    pub address_sizes: Option<String>,
    pub power_management: Option<String>,

    /************************ NOTE ***********************
     *    Parameters for AArch64 (ARMv8) architecture    *
     *****************************************************/
    pub cpu_implementer: Option<String>,
    pub cpu_architecture: Option<u8>,
    pub cpu_variant: Option<String>,
    pub cpu_part: Option<String>,
    pub cpu_revision: Option<u32>,

    /************************ NOTE ***********************
     *   Parameters for ppc64le (PowerPC) architecture   *
     *****************************************************/
    pub cpu: Option<String>,
    pub clock: Option<f32>,
    pub revision: Option<String>,
    pub timebase: Option<usize>,
    pub platform: Option<String>,
    pub machine: Option<String>,
    pub model_ppc: Option<String>,
}

impl ToJson for CPU {}

#[cfg(not(target_arch = "aarch64"))]
impl ToPlainText for CPU {
    fn to_plain(&self) -> String {
        let mut s = match self.processor {
            Some(proc) => format!("\nProcessor #{proc}\n"),
            None => "\nProcessor #unknown\n".to_string(),
        };
        s += "\tArchitecture: x86_64\n";
        s += &print_opt_val("Vendor ID", &self.vendor_id);
        s += &print_opt_val("CPU Family", &self.cpu_family);
        s += &print_opt_val("CPU Model ID", &self.model);
        s += &print_opt_val("CPU Model Name", &self.model_name);
        s += &print_opt_val("Stepping", &self.stepping);
        s += &print_opt_val("Microcode", &self.microcode);
        s += &print_opt_val("Current frequency", &self.cpu_mhz);
        s += &print_opt_val("L3 Cache Size", &self.cache_size);
        s += &print_opt_val("Physical ID of CPU Core", &self.physical_id);
        s += &print_opt_val("Siblings", &self.siblings);
        s += &print_opt_val("Core ID", &self.core_id);
        s += &print_opt_val("CPU cores", &self.cpu_cores);
        s += &print_opt_val("APIC ID", &self.apicid);
        s += &print_opt_val("Initial APIC ID", &self.initial_apicid);
        s += &print_opt_val("FPU", &self.fpu);
        s += &print_opt_val("FPU Exception", &self.fpu_exception);
        s += &print_opt_val("CPUID Level", &self.cpuid_level);
        s += &print_opt_val("WP", &self.wp);
        s += &print_opt_val("Bogo MIPS", &self.bogomips);
        s += &print_opt_val("Clflush Size", &self.clflush_size);
        s += &print_opt_val("Cache alignment", &self.cache_alignment);
        s += &print_opt_val("Address sizes", &self.address_sizes);
        s += &print_opt_val("Power management", &self.power_management);

        s
    }
}

#[cfg(target_arch = "aarch64")]
impl ToPlainText for CPU {
    fn to_plain(&self) -> String {
        let mut s = match self.processor {
            Some(proc) => format!("Processor #{proc}\n"),
            None => format!("Processor #unknown\n"),
        };
        s += &print_opt_val("CPU Implementer", &self.cpu_implementer);
        s += &print_opt_val("CPU Architecture", &self.cpu_architecture);
        s += &print_opt_val("CPU Variant", &self.cpu_variant);
        s += &print_opt_val("CPU Part", &self.cpu_part);
        s += &print_opt_val("CPU Revision", &self.cpu_revision);

        s
    }
}

fn read_info() -> Result<Vec<CPU>> {
    let blocks = read_to_string("/proc/cpuinfo")
        .map_err(|err| anyhow!("read_info(): Failed to read `/proc/cpuinfo` file: {err}"))?;
    let blocks = blocks
        .split("\n\n") // split by CPU blocks
        .collect::<Vec<_>>();
    let mut processors = Vec::with_capacity(blocks.len());

    for block in blocks {
        if block.trim().is_empty() {
            continue;
        }
        let mut cpu = CPU::default();
        for line in block.lines() {
            parse_cpuinfo(&mut cpu, line);
        }
        processors.push(cpu);
    }
    Ok(processors)
}

fn get_parts(s: &str) -> impl Iterator<Item = &str> {
    s.splitn(2, ':').map(|item| item.trim())
}

#[cfg(not(target_arch = "aarch64"))]
fn parse_cpuinfo(cpu: &mut CPU, parts: &str) {
    let mut parts = get_parts(parts);
    if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
        match key {
            "processor" => cpu.processor = val.parse().ok(),
            "vendor_id" => cpu.vendor_id = Some(val.to_string()),
            "cpu family" => cpu.cpu_family = val.parse().ok(),
            "model" => cpu.model = val.parse().ok(),
            "model name" => cpu.model_name = Some(val.to_string()),
            "stepping" => cpu.stepping = val.parse().ok(),
            "microcode" => cpu.microcode = Some(val.to_string()),
            "cpu MHz" => cpu.cpu_mhz = val.parse().ok(),
            "cache size" => cpu.cache_size = Size::try_from(val).ok(),
            "physical id" => cpu.physical_id = val.parse().ok(),
            "siblings" => cpu.siblings = val.parse().ok(),
            "core id" => cpu.core_id = val.parse().ok(),
            "cpu cores" => cpu.cpu_cores = val.parse().ok(),
            "apicid" => cpu.apicid = val.parse().ok(),
            "initial apicid" => cpu.initial_apicid = val.parse().ok(),
            "fpu" => cpu.fpu = Some(get_bool(val)),
            "fpu_exception" => cpu.fpu_exception = Some(get_bool(val)),
            "cpuid level" => cpu.cpuid_level = val.parse().ok(),
            "wp" => cpu.wp = Some(get_bool(val)),
            "flags" | "Features" => {
                cpu.flags = Some(val.split_whitespace().map(String::from).collect())
            }
            "bugs" => cpu.bugs = Some(val.split_whitespace().map(String::from).collect()),
            "bogomips" | "BogoMIPS" => cpu.bogomips = val.parse().ok(),
            "clflush size" => cpu.clflush_size = val.parse().ok(),
            "cache_alignment" => cpu.cache_alignment = val.parse().ok(),
            "address sizes" => cpu.address_sizes = Some(val.to_string()),
            "power management" => cpu.power_management = Some(val.to_string()),
            _ => {} // ignore unknown entry
        }
    }
}

#[cfg(target_arch = "aarch64")]
fn parse_cpuinfo(cpu: &mut CPU, parts: &str) {
    let mut parts = get_parts(parts);
    match (parts.next(), parts.next()) {
        (Some(key), Some(val)) => match key {
            // ARM
            "CPU implementer" => cpu.cpu_implementer = Some(val.to_string()),
            "CPU architecture" => cpu.cpu_architecture = val.parse().ok(),
            "CPU variant" => cpu.cpu_variant = Some(val.to_string()),
            "CPU part" => cpu.cpu_part = Some(val.to_string()),
            "CPU revision" => cpu.cpu_revision = val.parse().ok(),
            _ => {} // ignore unknown entry
        },
        _ => {}
    }
}

fn get_bool(s: &str) -> bool {
    matches!(s, "yes" | "ok")
}

/// Processor usage statistics
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Stat {
    pub cpu: Option<CpuUsage>,
    pub cpus: Vec<CpuUsage>,
    pub interrupts: Option<u64>,
    pub context_switches: Option<u64>,
    pub boot_time: Option<u64>,
    pub processes_created: Option<u64>,
    pub processes_running: Option<u64>,
    pub processes_blocked: Option<u64>,
    pub softirq: Option<SoftIrq>,
}

impl Stat {
    pub fn new() -> Result<Self> {
        parse_proc_stat()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
pub struct CpuUsage {
    pub user: Option<u64>,
    pub nice: Option<u64>,
    pub system: Option<u64>,
    pub idle: Option<u64>,
    pub iowait: Option<u64>,
    pub irq: Option<u64>,
    pub softirq: Option<u64>,
    pub steal: Option<u64>,
    pub guest: Option<u64>,
    pub guest_nice: Option<u64>,
}

impl CpuUsage {
    pub fn total_time(&self) -> u64 {
        self.user.unwrap_or(0)
            + self.nice.unwrap_or(0)
            + self.system.unwrap_or(0)
            + self.idle.unwrap_or(0)
            + self.iowait.unwrap_or(0)
            + self.irq.unwrap_or(0)
            + self.softirq.unwrap_or(0)
            + self.steal.unwrap_or(0)
    }

    pub fn active_time(&self) -> u64 {
        self.total_time() - self.idle.unwrap_or(0) - self.iowait.unwrap_or(0)
    }

    pub fn usage_percentage(&self, prev: Option<Self>) -> f32 {
        if prev.is_none() {
            return 0.0;
        }
        let prev = prev.unwrap();

        let total_diff = self.total_time().wrapping_sub(prev.total_time()); // shitcode (see #50 issue)!
        let active_diff = self.active_time().wrapping_sub(prev.active_time());

        if total_diff > 0 {
            (active_diff as f32 / total_diff as f32) * 100.0
        } else {
            0.0
        }
    }
}

impl From<&str> for CpuUsage {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace().collect::<Vec<&str>>();
        if parts.is_empty() || parts.len() < 11 {
            return Self::default();
        }

        Self {
            user: parts[1].parse().ok(),
            nice: parts[2].parse().ok(),
            system: parts[3].parse().ok(),
            idle: parts[4].parse().ok(),
            iowait: parts[5].parse().ok(),
            irq: parts[6].parse().ok(),
            softirq: parts[7].parse().ok(),
            steal: parts[8].parse().ok(),
            guest: parts[9].parse().ok(),
            guest_nice: parts[10].parse().ok(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Default)]
pub struct SoftIrq {
    pub total: Option<u64>,
    pub hi: Option<u64>,
    pub timer: Option<u64>,
    pub net_tx: Option<u64>,
    pub net_rx: Option<u64>,
    pub block: Option<u64>,
    pub irq_poll: Option<u64>,
    pub tasklet: Option<u64>,
    pub shed: Option<u64>,
    pub hrtimer: Option<u64>,
    pub rcu: Option<u64>,
}

impl From<&str> for SoftIrq {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace().collect::<Vec<&str>>();
        if parts.is_empty() || parts.len() < 12 {
            return Self::default();
        }

        Self {
            total: parts[1].parse().ok(),
            hi: parts[2].parse().ok(),
            timer: parts[3].parse().ok(),
            net_tx: parts[4].parse().ok(),
            net_rx: parts[5].parse().ok(),
            block: parts[6].parse().ok(),
            irq_poll: parts[7].parse().ok(),
            tasklet: parts[8].parse().ok(),
            shed: parts[9].parse().ok(),
            hrtimer: parts[10].parse().ok(),
            rcu: parts[11].parse().ok(),
        }
    }
}

fn parse_proc_stat() -> Result<Stat> {
    let content = read_to_string("/proc/stat")
        .map_err(|err| anyhow!("parse_proc_stat(): Failed to read `/proc/stat` file: {err}"))?;
    let mut stat = Stat::default();

    for line in content.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "cpu" => {
                if parts.len() >= 11 {
                    stat.cpu = Some(CpuUsage::from(line));
                }
            }
            key if key.starts_with("cpu")
                && key[3..]
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false) =>
            {
                if parts.len() >= 11 {
                    stat.cpus.push(CpuUsage::from(line));
                }
            }
            "intr" => {
                if parts.len() >= 2 {
                    stat.interrupts = parts[1].parse().ok();
                }
            }
            "ctxt" => {
                if parts.len() >= 2 {
                    stat.context_switches = parts[1].parse().ok();
                }
            }
            "btime" => {
                if parts.len() >= 2 {
                    stat.boot_time = parts[1].parse().ok();
                }
            }
            "processes" => {
                if parts.len() >= 2 {
                    stat.processes_created = parts[1].parse().ok();
                }
            }
            "procs_running" if parts.len() >= 2 => {
                stat.processes_running = parts[1].parse().ok();
            }
            "procs_blocked" if parts.len() >= 2 => {
                stat.processes_blocked = parts[1].parse().ok();
            }
            "softirq" if parts.len() >= 12 => {
                stat.softirq = Some(SoftIrq::from(line));
            }
            _ => {}
        }
    }
    Ok(stat)
}

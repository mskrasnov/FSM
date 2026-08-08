/* init.rs
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

//! Get information about `systemd` services
//! 
//! ## Usage
//! ```no-test
//! use ferrix_lib::init::SystemdServices;
//! use zbus::Connection;
//! 
//! let mut conn = Connection::system().await.unwrap();
//! let systemd = SystemdServices::new_from_connection(&conn)
//!     .await
//!     .unwrap();
//! 
//! dbg!(systemd);
//! ```

use anyhow::{Result, anyhow};
use libc::{CLOCK_MONOTONIC, CLOCK_REALTIME, clock_gettime, timespec};
use serde::Serialize;
use std::{fmt::Display, io::Error, mem::MaybeUninit};
pub use zbus::{Connection, zvariant::OwnedObjectPath};
use zbus_systemd::systemd1::ManagerProxy;

use crate::traits::*;

/// A structure containing information about `systemd` services
#[derive(Debug, Serialize, Clone)]
pub struct SystemdServices {
    pub timestamps: BootTimestamps,
    pub units: Vec<ServiceInfo>,
}

impl SystemdServices {
    /// Get current systemd services
    /// 
    /// ## Usage
    /// ```no-test
    /// use ferrix_lib::init::SystemdServices;
    /// use zbus::Connection;
    /// 
    /// let mut conn = Connection::system().await.unwrap();
    /// let sysd = SystemdServices::new_from_connection(&conn)
    ///     .await
    ///     .unwrap();
    /// dbg!(&sysd.units);
    /// ```
    pub async fn new_from_connection(conn: &Connection) -> Result<Self> {
        let mgr = ManagerProxy::new(conn).await?;
        let mut units = vec![];
        for unit in mgr.list_units().await? {
            units.push(ServiceInfo::from(unit));
        }
        let timestamps = BootTimestamps::get().await?;
        Ok(Self { timestamps, units })
    }
}

impl ToJson for SystemdServices {}

impl ToPlainText for SystemdServices {
    fn to_plain(&self) -> String {
        let mut s = format!("\nSystemd services list:");
        for service in &self.units {
            s += &service.to_plain();
        }

        s
    }
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
pub struct BootTimestamps {
    pub firmware: u64,
    pub loader: u64,
    pub kernel: u64,
    pub initrd_timestamp_mono: u64,
    pub userspace: u64,
    pub finish_timestamp_mono: u64,
    pub total: u64,
}

impl BootTimestamps {
    pub async fn get<'a>() -> Result<Self> {
        let conn = zbus::Connection::system().await?;
        let mgr = ManagerProxy::new(&conn).await?;
        Ok(Self {
            firmware: mgr.cached_firmware_timestamp_monotonic()?.unwrap_or(0),
            loader: mgr.loader_timestamp_monotonic().await?,
            kernel: mgr.kernel_timestamp().await?,
            initrd_timestamp_mono: mgr.init_rd_timestamp_monotonic().await?,
            userspace: mgr.userspace_timestamp_monotonic().await?,
            finish_timestamp_mono: mgr.finish_timestamp_monotonic().await?,
            total: 0,
        })
    }

    pub fn calc_boot_time(&mut self) -> Result<()> {
        if self.userspace == 0 || self.finish_timestamp_mono == 0 {
            return Err(anyhow!("Failed to get system load time: not enough data"));
        }
        let offset = {
            let now_rt = get_clock_time(CLOCK_REALTIME)?;
            let now_mono = get_clock_time(CLOCK_MONOTONIC)?;
            now_rt.saturating_sub(now_mono)
        };

        let userspace_usec = self.finish_timestamp_mono.saturating_sub(self.userspace);
        let kernel_usec = if self.kernel > 0 {
            let kernel_timestamp_mono = self.kernel.saturating_sub(offset);
            self.userspace.saturating_sub(kernel_timestamp_mono)
        } else {
            0
        };
        let loader_usec = if self.loader > 0 {
            // if self.initrd_timestamp_mono > 0 {
            //     self.initrd_timestamp_mono.saturating_sub(self.loader)
            // } else {
                self.userspace.saturating_sub(self.loader)
            // }
        } else {
            0
        };
        let firmware_usec = if self.loader > 0 {
            self.loader.saturating_sub(self.firmware)
        } else {
            0
        };

        self.firmware = firmware_usec;
        self.loader = loader_usec;
        self.kernel = kernel_usec;
        self.userspace = userspace_usec;

        self.total = firmware_usec + loader_usec + kernel_usec + userspace_usec;
        Ok(())
    }
}

fn get_clock_time(clock_id: i32) -> Result<u64> {
    let mut tp = MaybeUninit::<timespec>::uninit();
    let res = unsafe { clock_gettime(clock_id, tp.as_mut_ptr()) };
    if res == 0 {
        let tp = unsafe { tp.assume_init() };
        Ok(tp.tv_sec as u64 * 1_000_000 + (tp.tv_nsec as u64 / 1_000))
    } else {
        Err(anyhow!(
            "Failed to get clock_time: {}",
            Error::last_os_error()
        ))
    }
}

fn unescape(s: &str) -> String {
    s.replace("\\x20", " ")
        .replace("\\x5c", "\\")
        .replace("\\x2f", "/")
        .replace("\\x2d", "-")
}

type ServiceTuple = (
    String,
    String,
    String,
    String,
    String,
    String,
    OwnedObjectPath,
    u32,
    String,
    OwnedObjectPath,
);

#[derive(Debug, Serialize, Clone)]
pub struct ServiceInfo {
    /// Unit name (e.g. `hibernate.target`)
    pub name: String,

    /// Unit description (e.g. `System Hibernation`)
    pub description: String,

    /// Load state
    pub load_state: LoadState,

    /// Active state
    pub active_state: ActiveState,

    /// Work state
    pub work_state: WorkState,

    /// Daemon path
    pub daemon_path: String,

    /// Job ID
    pub job_id: u32,

    /// Unit type
    pub unit_type: UnitType,
}

impl ToPlainText for ServiceInfo {
    fn to_plain(&self) -> String {
        let mut s = format!("\nService \"{}\"\n", &self.name);
        s += &print_val("Description", &self.description);
        s += &print_val("Load state", &self.load_state);
        s += &print_val("Active state", &self.active_state);
        s += &print_val("Work state", &self.work_state);
        s += &print_val("Daemon path", &self.daemon_path);
        s += &print_val("Job ID", &self.job_id);
        s += &print_val("Unit type", &self.unit_type);

        s
    }
}

impl ToJson for ServiceInfo {}

impl From<ServiceTuple> for ServiceInfo {
    fn from(value: ServiceTuple) -> Self {
        Self {
            name: unescape(&value.0),
            description: unescape(&value.1),
            load_state: LoadState::from(&value.2),
            active_state: ActiveState::from(&value.3),
            work_state: WorkState::from(&value.4),
            daemon_path: unescape(&value.5),
            job_id: value.7,
            unit_type: UnitType::from(&value.8),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum LoadState {
    Loaded,
    Stub,
    Masked,
    NotFound,
    Unknown(String),
}

impl Display for LoadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Loaded => "Loaded",
                Self::Stub => "Stub",
                Self::Masked => "Masked",
                Self::NotFound => "Not found",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for LoadState {
    fn from(value: &String) -> Self {
        match value as &str {
            "loaded" => Self::Loaded,
            "stub" => Self::Stub,
            "masked" => Self::Masked,
            "not-found" => Self::NotFound,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum ActiveState {
    Active,
    Inactive,
    Activating,
    Deactivating,
    Failed,
    Unknown(String),
}

impl Display for ActiveState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Active => "Active",
                Self::Inactive => "Inactive",
                Self::Activating => "Activating",
                Self::Deactivating => "Deactivating",
                Self::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for ActiveState {
    fn from(value: &String) -> Self {
        match value as &str {
            "active" => Self::Active,
            "inactive" => Self::Inactive,
            "activating" => Self::Activating,
            "deactivating" => Self::Deactivating,
            "failed" => Self::Failed,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum WorkState {
    Active,
    Running,
    Exited,
    Dead,
    Mounted,
    Mounting,
    Plugged,
    Listening,
    Waiting,
    Failed,
    Unknown(String),
}

impl Display for WorkState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Active => "Active",
                Self::Running => "Running",
                Self::Exited => "Exited",
                Self::Dead => "Dead",
                Self::Mounted => "Mounted",
                Self::Mounting => "Mounting",
                Self::Plugged => "Plugged",
                Self::Listening => "Listening",
                Self::Waiting => "Waiting",
                Self::Failed => "Failed",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for WorkState {
    fn from(value: &String) -> Self {
        match value as &str {
            "active" => Self::Active,
            "running" => Self::Running,
            "exited" => Self::Exited,
            "dead" => Self::Dead,
            "mounted" => Self::Mounted,
            "mounting" => Self::Mounting,
            "plugged" => Self::Plugged,
            "listening" => Self::Listening,
            "waiting" => Self::Waiting,
            "failed" => Self::Failed,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum UnitType {
    Target,
    Service,
    Mount,
    Swap,
    None,
    Unknown(String),
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Target => "Target",
                Self::Service => "Service",
                Self::Mount => "Mount",
                Self::Swap => "Swap",
                Self::None => "None-type",
                _ => "Unknown",
            }
        )
    }
}

impl From<&String> for UnitType {
    fn from(value: &String) -> Self {
        match value as &str {
            "target" => Self::Target,
            "service" => Self::Service,
            "mount" => Self::Mount,
            "swap" => Self::Swap,
            "" => Self::None,
            _ => Self::Unknown(value.to_string()),
        }
    }
}

/* parts.rs
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

//! Get information about mounted partitions
//!
//! - [`Partitions`] - parses and represents entries from `/proc/partitions`;
//! - [`Storages`] - represents physical block devices from `/sys/block`;
//! - [`Mounts`] - represents currently mounted fs from `/proc/mounts`;
//! - [`FileSystemStats`] - provides calculated metrics: used space, usage
//!   percentage based on raw `statvfs` data.

use anyhow::{Result, anyhow};
use libc::statvfs;
use serde::{Deserialize, Serialize};
use std::ffi::{CString, c_char};
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

use crate::traits::ToJson;
use crate::utils::Size;

// NOTE: Is this structure really necessary, since there are `Mounts`?
/// List of partitions from `/proc/partitions` file
///
/// > **Note:** this structure filters out virtual devices like `loop` and
/// > `ram` to focus on actual block devices and their partitions.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Partitions {
    pub parts: Vec<Partition>,
}

impl Partitions {
    pub fn new() -> Result<Self> {
        let contents = read_to_string("/proc/partitions")?;
        Self::from_str(&contents)
    }

    /// Parse a raw string representation of `/proc/partitions` into a `Self`
    /// structure
    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().skip(1).filter(|s| {
            !s.is_empty() && !s.starts_with('m') && !s.contains("loop") && !s.contains("ram")
        });

        let mut parts = Vec::new();
        for line in lines {
            match Partition::try_from(line) {
                Ok(part) => parts.push(part),
                Err(why) => return Err(anyhow!("{why}")),
            }
        }

        Ok(Self { parts })
    }
}

impl ToJson for Partitions {}

/// Single block device partition
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Partition {
    /// Major device number
    pub major: usize,

    /// Minor device num
    pub minor: usize,

    /// Size of the partition in 1K-blocks
    pub blocks: u64,

    /// Name of the device (e.g. `sda1`, `nvme0n1p2`)
    pub name: String,

    /// Hardware-level metadata retrieved from `/sys/block/`
    pub dev_info: DeviceInfo,

    /// Filesystem statistics, if applicable and retrievable
    pub statvfs: Option<FileSystemStats>,
}

impl Partition {
    /// Calculate the logical sise of the partition
    ///
    /// Multiplies the number of blocks by the logical block size. Returns
    /// `None` of the logical size is unknown
    pub fn get_logical_size(&self) -> Option<Size> {
        let lbsize = self.dev_info.logical_block_size;
        match lbsize {
            Some(lbsize) => {
                let blocks = self.blocks;
                Some(Size::B(blocks * lbsize))
            }
            None => None,
        }
    }
}

impl TryFrom<&str> for Partition {
    type Error = String;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut chs = value.split_whitespace();

        match (chs.next(), chs.next(), chs.next(), chs.next()) {
            (Some(major), Some(minor), Some(blocks), Some(name)) => {
                let major = major.parse::<usize>().map_err(|err| format!("{err}"))?;
                let minor = minor.parse::<usize>().map_err(|err| format!("{err}"))?;
                let blocks = blocks.parse::<u64>().map_err(|err| format!("{err}"))?;

                Ok(Self {
                    major,
                    minor,
                    blocks,
                    name: name.to_string(),
                    dev_info: DeviceInfo::get(name),
                    statvfs: FileSystemStats::from_path(Path::new("/dev/").join(name)).ok(), // .map_err(|err| format!("Failed to get file system statistics for device {name}: {err}"))?,
                })
            }
            _ => Err(format!("String '{value}' parsing error")),
        }
    }
}

/// Hardware-level metadata for a block device
///
/// This data is read directly from the `/sys/block/<device>/device/` and
/// `/sys/block/<device>/queue/` dirs
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceInfo {
    /// The model name of this device
    pub model: Option<String>,

    /// Manufacturer of the device
    pub vendor: Option<String>,

    /// The unique serial number of the device
    pub serial: Option<String>,

    /// The logical block size in bytes (typically 510 or 4096)
    pub logical_block_size: Option<u64>,
}

impl DeviceInfo {
    /// Get device information for the given device name (e.g. `sda`)
    pub fn get(devname: &str) -> Self {
        let path = Path::new("/sys/block/").join(devname);
        let device = path.join("device");
        let queue = path.join("queue");

        let model = device.join("model");
        let vendor = device.join("vendor");
        let serial = device.join("serial");

        let logical_block_size = queue.join("logical_block_size");
        let logical_block_size = match read_to_string(logical_block_size) {
            Ok(lbs) => lbs.trim().parse::<u64>().ok(),
            Err(_) => None,
        };

        Self {
            model: read_to_string(model)
                .ok()
                .and_then(|m| Some(m.trim().to_string())),
            vendor: read_to_string(vendor)
                .ok()
                .and_then(|v| Some(v.trim().to_string())),
            serial: read_to_string(serial)
                .ok()
                .and_then(|s| Some(s.trim().to_string())),
            logical_block_size,
        }
    }

    /// Returns `true` if all fields in the `DeviceInfo` are `None`
    pub fn is_none(&self) -> bool {
        self.model.is_none()
            && self.vendor.is_none()
            && self.serial.is_none()
            && self.logical_block_size.is_none()
    }
}

/// Physical disk drives from `/sys/block/` directory
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Storages {
    pub storages: Vec<Storage>,
}

impl Storages {
    /// Scan `/sys/block/` and populate the list of physical storage devices
    ///
    /// > **Note:** this method filters out virtual devies like `loop` and
    /// > `zram`
    pub fn new() -> Result<Self> {
        let dir_contents = read_dir("/sys/block")?.filter(|entry| {
            if entry.is_err() {
                false
            } else {
                let entry = entry.as_ref().unwrap();
                let s = entry.path().to_string_lossy().to_string();
                !(s.contains("loop") || s.contains("zram"))
            }
        });

        let mut storages = vec![];
        for dir in dir_contents {
            let dir = dir?.path();
            storages.push(Storage::from_pathbuf(&dir)?);
        }
        Ok(Self { storages })
    }
}

/// Physical storage device info (e.g. `sda`, `mmcblk0`, `nvme0n1`, etc.)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Storage {
    /// `/sys/block/` subdirectory name (e.g. `sda`, `mmcblk0`, `nvme0n1`, etc.)
    ///
    /// (the device name as it appears in `/sys/block/` directory)
    pub devname: String,

    /// Indicates if the device is removable
    pub removable: bool,

    /// Indicates if the device is currently mounted or configured as read-only
    pub ro: bool,

    /// Total disk size, bytes
    pub size: Size,

    /// Indicates if the device is hidden from the system
    pub hidden: bool,

    /// The filesystem UUID, if applicable
    pub uuid: Option<String>,

    /// Device model
    pub model: Option<String>,

    /// Device vendor
    pub vendor: Option<String>,

    /// Device serial number
    pub serial: Option<String>,

    /// Firmware revision
    pub revision: Option<String>,

    /// The World Wide Name (WWN) of EUI of the device, stripped of the
    /// `eui.` prefix
    pub wwid_eui: Option<String>,

    /// The transport protocol used (e.g. `sata`, `nvme`, `usb`, etc.)
    pub transport: Option<String>,
}

impl Storage {
    /// Get a `Storage` instance from a given `/sys/block/` path
    pub fn from_pathbuf(path: &PathBuf) -> Result<Self> {
        let read = |file: &str| read_to_string(path.join(file));

        let devname = path
            .strip_prefix("/sys/block/")?
            .to_string_lossy()
            .to_string();
        let removable = {
            let data = read("removable")?;
            if data.trim() == "0" { false } else { true }
        };
        let ro = {
            let data = read("ro")?;
            if data.trim() == "0" { false } else { true }
        };
        let hidden = {
            let data = read("hidden")?;
            if data.trim() == "0" { false } else { true }
        };
        let size = {
            let data = read("size")?;
            Size::B(data.trim().parse()?)
        };
        let uuid = read("uuid").and_then(|a| Ok(a.trim().to_string())).ok();
        let model = read("device/model")
            .and_then(|a| Ok(a.trim().to_string()))
            .ok();
        let vendor = read("device/vendor")
            .and_then(|a| Ok(a.trim().to_string()))
            .ok();
        let serial = read("device/serial")
            .and_then(|a| Ok(a.trim().to_string()))
            .ok();
        let revision = read("device/firmware_rev")
            .and_then(|a| Ok(a.trim().to_string()))
            .ok();
        let transport = read("device/transport")
            .and_then(|a| Ok(a.trim().to_string()))
            .ok();
        let wwid_eui = read("wwid").and_then(|a| Ok(a.replace("eui.", ""))).ok();

        Ok(Self {
            devname,
            removable,
            ro,
            size,
            hidden,
            uuid,
            model,
            vendor,
            serial,
            transport,
            wwid_eui,
            revision,
        })
    }
}

/// Mounted filesystems list from `/proc/mounts` file
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mounts {
    pub mounts: Vec<MountEntry>,
}

/// Single mounted filesystem entry
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MountEntry {
    /// The block device or a virtual fs src (e.g. `/dev/sda1`, `tmpfs`)
    pub device: String,

    /// The directory where the fs is mounted
    pub mount_point: String,

    /// The type of the fs (e.g. `ext4`, `btrfs`, `vfat`)
    pub filesystem: String,

    /// Comma-separated mount options (e.g. `rw,realtime`)
    pub options: String,

    /// Dump flag (used by the `dump` utility, usually 0)
    pub dump: u8,

    /// Pass number (used by the `fsck` to determine check order)
    pub pass: u8,

    /// Filesystem usage statistics, if retrievable
    pub fstats: Option<FileSystemStats>,
}

impl TryFrom<&str> for MountEntry {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let values = value.split_whitespace().collect::<Vec<_>>();
        if values.len() != 6 {
            return Err(anyhow!(
                "Format of mount string is incorrect\n(string: \"{value}\")",
            ));
        }

        Ok(Self {
            device: values[0].to_string(),
            mount_point: values[1].to_string(),
            filesystem: values[2].to_string(),
            options: values[3].to_string(),
            dump: values[4].parse()?,
            pass: values[5].parse()?,
            fstats: FileSystemStats::from_path(values[1]).ok(),
        })
    }
}

impl Mounts {
    pub fn new() -> Result<Self> {
        let contents = read_to_string("/proc/mounts")?;
        let lines = contents.lines();
        let mut mounts = vec![];

        for line in lines {
            if line.starts_with("/")
                || line.starts_with("udev")
                || line.starts_with("sysfs")
                || line.starts_with("tmpfs")
            {
                mounts.push(MountEntry::try_from(line)?);
            }
        }
        Ok(Self { mounts })
    }
}

/// Filesystem usage statistics (via `statvfs` C function)
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct FileSystemStats {
    /// Block size, bytes
    pub block_size: u64,

    /// Fragment size, bytes
    pub fragment_size: u64,

    /// Total number of blocks in this fs
    pub total_blocks: u64,

    /// Total number of free blocks
    pub free_blocks: u64,

    /// Total number of free blocks available to non-privileged
    /// processes
    pub available_blocks: u64,

    /// Total number of inodes
    pub total_inodes: u64,

    /// Total number of free inodes
    pub free_inodes: u64,
}

impl FileSystemStats {
    /// Get fs stats for the given path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path
            .as_ref()
            .to_str()
            .ok_or_else(|| anyhow!("Invalid characters in path ()"))?;
        let c_path = CString::new(path_str)
            .map_err(|err| anyhow!("Failed to convert Rust string into C string: {err}"))?;

        // SAFETY: we are passing a valid null-terminated C-string to statvfs,
        // and providing a valid zeroed mutable pointer for the output
        unsafe { Self::statvfs(c_path.as_ptr()) }
    }

    /// Unsafe wrapper for the `libc::statvfs` system call
    unsafe fn statvfs(path: *const c_char) -> Result<Self> {
        let mut stats: libc::statvfs = unsafe { std::mem::zeroed() };
        let result = unsafe { statvfs(path, &mut stats) };

        if result == 0 {
            Ok(Self {
                block_size: stats.f_bsize as u64,
                fragment_size: stats.f_frsize as u64,
                total_blocks: stats.f_blocks as u64,
                free_blocks: stats.f_bfree as u64,
                available_blocks: stats.f_bavail as u64,
                total_inodes: stats.f_files as u64,
                free_inodes: stats.f_ffree as u64,
            })
        } else {
            Err(anyhow!(
                "statvfs() failed: errno {}",
                std::io::Error::last_os_error()
            ))
        }
    }

    /// Calculate the total capacity of the fs in bytes
    pub fn total_bytes(&self) -> u64 {
        self.total_blocks * self.fragment_size
    }

    /// Calculate the total capacity of the fs as a [`Size`] enum
    pub fn total_size(&self) -> Size {
        Size::B(self.total_bytes())
    }

    /// Calculate the free space in bytes
    pub fn free_bytes(&self) -> u64 {
        self.free_blocks * self.fragment_size
    }

    /// Calculate the free space as a [`Size`] enum
    pub fn free_size(&self) -> Size {
        Size::B(self.free_bytes())
    }

    /// Calculate the space available to non-privileged users in bytes
    pub fn avail_bytes(&self) -> u64 {
        self.available_blocks * self.fragment_size
    }

    pub fn avail_size(&self) -> Size {
        Size::B(self.avail_bytes())
    }

    /// Calculate the used space in bytes
    pub fn used_bytes(&self) -> u64 {
        if self.total_bytes() == 0 {
            return 0;
        }
        self.total_bytes() - self.free_bytes()
    }

    pub fn used_size(&self) -> Size {
        Size::B(self.used_bytes())
    }

    /// Calculate the percentage of the fs that is currently used (0.0 to 100.0)
    pub fn usage_percent(&self) -> f64 {
        if self.total_bytes() == 0 {
            return 0.;
        }
        let used = self.used_bytes() as f64;
        let total = self.total_bytes() as f64;
        (used / total) * 100.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARTITIONS: &str = "major minor  #blocks  name

 259        0  250059096 nvme0n1
 259        1     102400 nvme0n1p1
 259        2      16384 nvme0n1p2
 259        3  249068548 nvme0n1p3
 259        4     866304 nvme0n1p4
   8        0  468851544 sda
   8        1     614400 sda1
   8        2   73138176 sda2
   8        3  337163264 sda3
   8        4   57933824 sda4
 253        0    3976960 zram0";

    #[test]
    fn partitions_from_str_test() {
        let parts = Partitions::from_str(PARTITIONS).unwrap();
        dbg!(&parts);
        assert_eq!(parts.parts.len(), 10);
        assert_eq!(&parts.parts[0].name, "nvme0n1");
        assert_eq!(parts.parts[0].major, 259);
        assert_eq!(parts.parts[0].minor, 0);
        assert_eq!(parts.parts[0].blocks, 250059096);
        let _ = std::fs::write("./test-filesystems.json", parts.to_json_pretty().unwrap());
    }

    #[test]
    fn partition_invalid_str_test() {
        let s = "256 0 nvme";
        let part = Partition::try_from(s);
        assert!(part.is_err());
    }

    #[test]
    fn partition_valid_str_test() {
        let s = "255 4 666 sda";
        let part = Partition::try_from(s);
        assert!(part.is_ok());
    }
}

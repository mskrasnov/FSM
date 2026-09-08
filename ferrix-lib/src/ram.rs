/* ram.rs
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

//! Get information about RAM
//!
//! Reads information from `/proc/meminfo` file
//!
//! ## Example
//! ```
//! use ferrix_lib::ram::RAM;
//! use ferrix_lib::traits::ToJson;
//!
//! let ram = RAM::new().unwrap();
//! let json = ram.to_json().unwrap();
//! dbg!(json);
//! ```

use anyhow::{Result, anyhow};
use serde::Serialize;
use std::fs::read_to_string;
use std::path::Path;

use crate::traits::ToJson;
use crate::utils::Size;

/// A structure containing data from the `/proc/meminfo` file
#[derive(Debug, Serialize, Default, Clone)]
pub struct RAM {
    /// Total usable physical RAM (excludes reserved/firmware memory)
    pub total: Size,

    /// Completely unused physical RAM. Often **misleadingly low**
    /// due to caching
    pub free: Size,

    /// **Best estimate** of memory available for new apps (accounts
    /// for caches/reclaimable memory)
    pub available: Size,

    /// Temporary storage for raw disk blocks (e.g. filesystem
    /// metadata)
    pub buffers: Size,

    /// Page cache for files read from disk (reclaimed when apps
    /// need memory)
    pub cached: Size,

    /// Memory swapped out but later accessed, now present in both
    /// RAM and swap
    pub swap_cached: Size,

    /// Recently used memory (harder to reclaim)
    pub active: Size,

    /// Less recently used memory (eased to reclaim)
    pub inactive: Size,

    /// Active anonymous pages (e.g. heap/stack, not file-backed)
    pub active_anon: Size,

    /// Inactive anonymous pages
    pub inactive_anon: Size,

    /// Active file-backed pages (cached files)
    pub active_file: Size,

    /// Inactive file-backed pages
    pub inactive_file: Size,

    /// Memory that cannot be pages out (e.g. locked with `mlock()`)
    pub unevictable: Size,

    /// Memory that is locked (cannot be swapped out)
    pub mlocked: Size,

    /// Total amount of swap space available
    pub swap_total: Size,

    /// Amount of swap space that is currently unused
    pub swap_free: Size,

    pub zswap: Size,
    pub zswapped: Size,

    /// Data waiting to be written to disk
    pub dirty: Size,

    /// Data actively being written to disk
    pub writeback: Size,

    /// Non file-backed pages mapped into user-space page tables
    pub anon_pages: Size,

    /// Files (like libraries) that have been mapped into memory
    /// (also includes `tmpfs`/`shmem`)
    pub mapped: Size,

    /// Total memory used by shared memory (`shmem`) and `tmpfs`
    pub shmem: Size,

    /// Kernel allocations that the kernel will attempt to reclaim
    /// under memory pressure (includes `SReclaimable` and other
    /// reclaimable slabs)
    pub kreclaimable: Size,

    /// In-kernel data structures cache (includes `SReclaimable` and
    /// `SUnreclaim`)
    pub slab: Size,

    /// Part of `Slab` that might be reclaimed, such as caches for
    /// directory inodes, etc.
    pub sreclaimable: Size,

    /// Part of `Slab` that cannot be reclaimed
    pub sunreclaim: Size,

    /// Memory used by kernel stacks
    pub kernel_stack: Size,

    /// Memory used by page tables (to map virtual to physical
    /// addresses)
    pub page_tables: Size,

    pub sec_page_tables: Size,

    /// Memory that has been sent to the NFS server but not yet
    /// committed to stable storage
    pub nfs_unstable: Size,

    /// Memory used for block device bounce buffers (rarely used on
    /// modern systems)
    pub bounce: Size,

    /// Memory used by FUSE for temporary writeback buffers
    pub writeback_tmp: Size,

    /// Based on the overcommit ratio, this is the total amount of
    /// memory currently available to be allocated
    pub commit_limit: Size,

    /// he amount of memory currently allocated by the system. The
    /// kernel may overcommit this
    pub commited_as: Size,

    /// Total size of `vmalloc` memory area
    pub vmalloc_total: Size,

    /// Amount of `vmalloc` area which is used
    pub vmalloc_used: Size,

    /// Largest contiguous block of free `vmalloc` space
    pub vmalloc_chunk: Size,

    /// Memory used for per-cpu allocations (each CPU has its own
    /// block)
    pub percpu: Size,

    /// Memory that the kernel identified as corrupted (when
    /// `CONFIG_MEMORY_FAILURE` is enabled)
    pub hardware_corrupted: Size,

    /// Non-file backed huge pages mapped into user-space page tables
    /// (transparent hugepages)
    pub anon_huge_pages: Size,

    /// Huge pages used by shared memory (`shmem`) and `tmpfs`.
    pub shmem_huge_pages: Size,

    /// `shmem`/`tmpfs` memory that is mapped into user space with
    /// huge pages
    pub shmem_pmd_mapped: Size,

    /// Total CMA (Contiguous Memory Allocator) area
    pub cma_total: Option<Size>,

    /// Free memory in the CMA area
    pub cma_free: Option<Size>,

    pub file_huge_pages: Size,
    pub file_pmd_mapped: Size,
    pub unaccepted: Size,
    pub huge_pages_total: u32,
    pub huge_pages_free: u32,
    pub huge_pages_rsvd: u32,
    pub huge_pages_surp: u32,
    pub huge_page_size: Size,
    pub huge_tlb: Size,
    pub direct_map_4k: Size,
    pub direct_map_2m: Size,
    pub direct_map_1g: Size,
}

impl RAM {
    pub fn new() -> Result<Self> {
        let chunks = read_to_string("/proc/meminfo")?;
        let chunks = chunks
            .lines()
            .map(|item| {
                let mut items = item.split(':').map(|item| item.trim());
                (items.next(), items.next())
            })
            .collect::<Vec<_>>();
        let mut ram = RAM::default();
        for chunk in chunks {
            match chunk {
                (Some(key), Some(val)) => match key {
                    "MemTotal" => ram.total = Size::try_from(val)?,
                    "MemFree" => ram.free = Size::try_from(val)?,
                    "MemAvailable" => ram.available = Size::try_from(val)?,
                    "Buffers" => ram.buffers = Size::try_from(val)?,
                    "Cached" => ram.cached = Size::try_from(val)?,
                    "SwapCached" => ram.swap_cached = Size::try_from(val)?,
                    "Active" => ram.active = Size::try_from(val)?,
                    "Inactive" => ram.inactive = Size::try_from(val)?,
                    "Active(anon)" => ram.active_anon = Size::try_from(val)?,
                    "Inactive(anon)" => ram.inactive_anon = Size::try_from(val)?,
                    "Active(file)" => ram.active_file = Size::try_from(val)?,
                    "Inactive(file)" => ram.inactive_file = Size::try_from(val)?,
                    "Unevictable" => ram.unevictable = Size::try_from(val)?,
                    "Mlocked" => ram.mlocked = Size::try_from(val)?,
                    "SwapTotal" => ram.swap_total = Size::try_from(val)?,
                    "SwapFree" => ram.swap_free = Size::try_from(val)?,
                    "Zswap" => ram.zswap = Size::try_from(val)?,
                    "Zswapped" => ram.zswapped = Size::try_from(val)?,
                    "Dirty" => ram.dirty = Size::try_from(val)?,
                    "Writeback" => ram.writeback = Size::try_from(val)?,
                    "AnonPages" => ram.anon_pages = Size::try_from(val)?,
                    "Mapped" => ram.mapped = Size::try_from(val)?,
                    "Shmem" => ram.shmem = Size::try_from(val)?,
                    "KReclaimable" => ram.kreclaimable = Size::try_from(val)?,
                    "Slab" => ram.slab = Size::try_from(val)?,
                    "SReclaimable" => ram.sreclaimable = Size::try_from(val)?,
                    "SUnreclaim" => ram.sunreclaim = Size::try_from(val)?,
                    "KernelStack" => ram.kernel_stack = Size::try_from(val)?,
                    "PageTables" => ram.page_tables = Size::try_from(val)?,
                    "SecPageTables" => ram.sec_page_tables = Size::try_from(val)?,
                    "NFS_Unstable" => ram.nfs_unstable = Size::try_from(val)?,
                    "Bounce" => ram.bounce = Size::try_from(val)?,
                    "WritebackTmp" => ram.writeback_tmp = Size::try_from(val)?,
                    "CommitLimit" => ram.commit_limit = Size::try_from(val)?,
                    "Committed_AS" => ram.commited_as = Size::try_from(val)?,
                    "VmallocTotal" => ram.vmalloc_total = Size::try_from(val)?,
                    "VmallocUsed" => ram.vmalloc_used = Size::try_from(val)?,
                    "VmallocChunk" => ram.vmalloc_chunk = Size::try_from(val)?,
                    "Percpu" => ram.percpu = Size::try_from(val)?,
                    "HardwareCorrupted" => ram.hardware_corrupted = Size::try_from(val)?,
                    "AnonHugePages" => ram.anon_huge_pages = Size::try_from(val)?,
                    "ShmemHugePages" => ram.shmem_huge_pages = Size::try_from(val)?,
                    "ShmemPmdMapped" => ram.shmem_pmd_mapped = Size::try_from(val)?,
                    "CmaTotal" => ram.cma_total = Size::try_from(val).ok(),
                    "CmaFree" => ram.cma_free = Size::try_from(val).ok(),
                    "FileHugePages" => ram.file_huge_pages = Size::try_from(val)?,
                    "FilePmdMapped" => ram.file_pmd_mapped = Size::try_from(val)?,
                    "Unaccepted" => ram.unaccepted = Size::try_from(val)?,
                    "HugePages_Total" => ram.huge_pages_total = val.parse()?,
                    "HugePages_Free" => ram.huge_pages_free = val.parse()?,
                    "HugePages_Rsvd" => ram.huge_pages_rsvd = val.parse()?,
                    "HugePages_Surp" => ram.huge_pages_surp = val.parse()?,
                    "Hugepagesize" => ram.huge_page_size = Size::try_from(val)?,
                    "Hugetlb" => ram.huge_tlb = Size::try_from(val)?,
                    "DirectMap4k" => ram.direct_map_4k = Size::try_from(val)?,
                    "DirectMap2M" => ram.direct_map_2m = Size::try_from(val)?,
                    "DirectMap1G" => ram.direct_map_1g = Size::try_from(val)?,
                    _ => continue,
                },
                _ => {}
            }
        }
        Ok(ram)
    }

    pub fn used_ram(&self, base: u8) -> Size {
        if base != 2 && base != 10 {
            panic!("Unknown base: {base} (supported values: 2 or 10)");
        }

        let total = if base == 2 {
            self.total.get_bytes2().unwrap_or(0) as f32 / 1024. / 1024. / 1024.
        } else
        /* base == 10 */
        {
            self.total.get_bytes10().unwrap_or(0) as f32 / 1000. / 1000. / 1000.
        };

        let avail = if base == 2 {
            self.available.get_bytes2().unwrap_or(0) as f32 / 1024. / 1024. / 1024.
        } else
        /* base == 10 */
        {
            self.available.get_bytes10().unwrap_or(0) as f32 / 1000. / 1000. / 1000.
        };
        let used = total - avail;

        Size::GB(used)
    }

    pub fn usage_percentage(&self) -> Option<f64> {
        let total_ram = self.total.get_bytes2()?;
        let avail_ram = self.available.get_bytes2()?;
        let used_ram = total_ram - avail_ram;

        Some(used_ram as f64 / total_ram as f64 * 100.)
    }
}

impl ToJson for RAM {}

/// Information about swap files or partitions
#[derive(Debug, Serialize, Clone)]
pub struct Swaps {
    pub swaps: Vec<Swap>,
}

impl Swaps {
    pub fn new() -> Result<Self> {
        let mut swaps = vec![];

        let data = read_to_string("/proc/swaps")?;
        let items = data.lines().skip(1);

        for swap in items {
            swaps.push(Swap::try_from(swap)?)
        }

        Ok(Self { swaps })
    }
}

impl ToJson for Swaps {}

#[derive(Debug, Serialize, Clone)]
pub struct Swap {
    /// Path to the file or partition
    pub filename: String,

    /// Type
    pub swap_type: String,

    /// Swap size
    pub size: Size,

    /// Used space
    pub used: Size,

    /// Priority of this swap file/partition
    pub priority: i8,
}

impl Swap {
    pub fn used_swap(&self, base: u8) -> Size {
        if base != 2 && base != 10 {
            panic!("Unknown base: {base} (supported values: 2 or 10)");
        }
        let used = if base == 2 {
            self.used.get_bytes2()
        } else {
            self.used.get_bytes10()
        }
        .unwrap_or(0);
        Size::B(used)
    }

    pub fn usage_percentage(&self) -> Option<f64> {
        let total_swap = self.size.get_bytes2()?;
        let used_swap = self.used.get_bytes2()?;

        Some(used_swap as f64 / total_swap as f64 * 100.)
    }
}

impl TryFrom<&str> for Swap {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let data = value.split_whitespace().collect::<Vec<_>>();
        if data.len() != 5 {
            return Err(anyhow!("Format of the \"{value}\" string is incorrect!"));
        }

        Ok(Self {
            filename: data[0].to_string(),
            swap_type: data[1].to_string(),
            size: Size::KB(data[2].parse()?),
            used: Size::KB(data[3].parse()?),
            priority: data[4].parse()?,
        })
    }
}
impl ToJson for Swap {}

#[derive(Debug, Clone, Serialize)]
pub struct Zswap {
    pub accept_threshold_percent: Option<u8>,
    pub compressor: Option<String>,
    pub enabled: Option<bool>,
    pub max_pool_percent: Option<u8>,
    pub non_same_filled_pages_enabled: Option<bool>,
    pub same_filled_pages_enabled: Option<bool>,
    pub zpool: Option<String>,
    pub pool_total_size: Option<Size>,
    pub stored_pages_size: Option<Size>,
    pub compression_ratio: Option<f32>,
}

fn get_page_size() -> u64 {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as u64 }
}

impl Zswap {
    pub fn new() -> Self {
        let rdbg = |f: &str| read_to_string(Path::new("/sys/kernel/debug/zswap/").join(f)).ok();
        let fcnf =
            |f: &str| read_to_string(Path::new("/sys/module/zswap/parameters/").join(f)).ok();
        let get_bool = |content: &str| {
            if content.trim() == "Y" || content != "0" {
                true
            } else {
                false
            }
        };

        let accept_threshold_percent =
            fcnf("accept_threshold_percent").and_then(|atp| atp.trim().parse::<u8>().ok());
        let compressor = fcnf("compressor");
        let enabled = fcnf("enabled").and_then(|e| Some(get_bool(e.trim())));
        let max_pool_percent =
            fcnf("max_pool_percent").and_then(|mpp| mpp.trim().parse::<u8>().ok());
        let non_same_filled_pages_enabled =
            fcnf("non_same_filled_pages_enabled").and_then(|nsfpe| Some(get_bool(nsfpe.trim())));
        let same_filled_pages_enabled =
            fcnf("same_filled_pages_enabled").and_then(|sfpe| Some(get_bool(sfpe.trim())));
        let zpool = fcnf("zpool");

        let pool_total_size =
            rdbg("pool_total_size").and_then(|pts| pts.trim().parse::<u64>().ok()); // bytes
        let stored_page_size =
            rdbg("stored_page_size").and_then(|sps| sps.trim().parse::<u64>().ok()); // bytes
        let compression_ratio = match (pool_total_size, stored_page_size) {
            (Some(pts), Some(sps)) => Some(sps as f32 / pts as f32),
            _ => None,
        };

        Self {
            accept_threshold_percent,
            compressor,
            enabled,
            max_pool_percent,
            non_same_filled_pages_enabled,
            same_filled_pages_enabled,
            zpool,
            pool_total_size: pool_total_size.and_then(|pts| Some(Size::B(pts))),
            stored_pages_size: stored_page_size
                .and_then(|sps| Some(Size::B(sps * get_page_size()))),
            compression_ratio,
        }
    }
}

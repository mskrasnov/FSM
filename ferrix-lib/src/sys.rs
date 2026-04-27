/* sys.rs
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

//! Get information about installed system

use crate::utils::read_to_string;
use crate::{traits::*, utils::Size};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::env::{var, vars};

/// A structure containing all collected information about
/// installed system
#[derive(Debug, Serialize)]
pub struct Sys {
    /// Machine ID
    pub machine_id: Option<String>,

    /// Timezone
    pub timezone: Option<String>,

    /// Environment variables for current user
    pub env_vars: Vec<(String, String)>,

    /// Uptime
    pub uptime: Uptime,

    /// System load (average)
    pub loadavg: LoadAVG,

    /// List of installed shells
    pub shells: Shells,

    /// Host name
    pub hostname: Option<HostName>,
    // /// Current locale
    // pub locale: Locale,
}

impl Sys {
    pub fn new() -> Result<Self> {
        Ok(Self {
            machine_id: read_to_string("/etc/machine-id").ok(),
            timezone: read_to_string("/etc/timezone").ok(),
            env_vars: get_env_vars(),
            uptime: Uptime::new()?,
            loadavg: LoadAVG::new()?,
            shells: get_shells()?,
            hostname: get_hostname(),
            // locale: todo!(),
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.uptime = Uptime::new()?;
        self.loadavg = LoadAVG::new()?;
        Ok(())
    }
}

impl ToJson for Sys {}

/// Information about Linux kernel
#[derive(Debug, Serialize, Clone)]
pub struct Kernel {
    /// All data about kernel
    pub uname: Option<String>, // /proc/version

    /// Kernel command line
    pub cmdline: Option<String>, // /proc/cmdline

    /// Kernel architecture
    pub arch: Option<String>, // /proc/sys/kernel/arch

    /// Kernel version
    pub version: Option<String>, // /proc/sys/kernel/osrelease

    /// Kernel build info
    pub build_info: Option<String>, // /proc/sys/kernel/version

    /// Max processes count
    pub pid_max: u32, // /proc/sys/kernel/pid_max

    /// Max threads count
    pub threads_max: u32, // /proc/sys/kernel/threads-max

    /// Max user events
    pub user_events_max: Option<u32>, // /proc/sys/kernel/user_events_max

    /// Available enthropy
    pub enthropy_avail: Option<u16>, // /proc/sys/kernel/random/entropy_avail
}

impl Kernel {
    pub fn new() -> Result<Self> {
        Ok(Self {
            uname: read_to_string("/proc/version").ok(),
            cmdline: read_to_string("/proc/cmdline").ok(),
            arch: read_to_string("/proc/sys/kernel/arch").ok(),
            version: read_to_string("/proc/sys/kernel/osrelease").ok(),
            build_info: read_to_string("/proc/sys/kernel/version").ok(),
            pid_max: read_to_string("/proc/sys/kernel/pid_max")?.parse()?,
            threads_max: read_to_string("/proc/sys/kernel/threads-max")?.parse()?,
            user_events_max: match read_to_string("/proc/sys/kernel/user_events_max").ok() {
                Some(uem) => uem.parse().ok(),
                None => None,
            },
            enthropy_avail: match read_to_string("/proc/sys/kernel/random/entropy_avail").ok() {
                Some(ea) => ea.parse().ok(),
                None => None,
            },
        })
    }
}

impl ToJson for Kernel {}

/// Information about installed distro from `/etc/os-release`
///
/// > Information from *[freedesktop](https://www.freedesktop.org/software/systemd/man/249/os-release.html)* portal.
#[derive(Debug, Serialize, Default, Clone)]
pub struct OsRelease {
    /// The operating system name without a version component
    ///
    /// If not set, a default `Linux` value may be used
    pub name: String,

    /// A lower-case string identifying the OS, excluding any
    /// version information
    pub id: Option<String>,

    /// A space-separated list of operating system identifiers in the
    /// same syntax as the `id` param.
    pub id_like: Option<String>,

    /// A pretty OS name in a format suitable for presentation to
    /// the user. May or may not contain a release code or OS version
    /// of some kind, as suitable
    pub pretty_name: Option<String>,

    /// A CPE name for the OS, in URI binding syntax
    pub cpe_name: Option<String>,

    /// Specific variant or edition of the OS suitable for
    /// presentation to the user
    pub variant: Option<String>,

    /// Lower-case string identifying a specific variant or edition
    /// of the OS
    pub variant_id: Option<String>,

    /// The OS version, excluding any OS name information, possibly
    /// including a release code name, and suitable for presentation
    /// to the user
    pub version: Option<String>,

    /// A lower-case string identifying the OS version, excluding any
    /// OS name information or release code name
    pub version_id: Option<String>,

    /// A lower-case string identifying the OS release code name,
    /// excluding any OS name information or release version
    pub version_codename: Option<String>,

    /// A string uniquely identifying the system image originally
    /// used as the installation base
    pub build_id: Option<String>,

    /// A lower-case string, identifying a specific image of the OS.
    /// This is supposed to be used for envs where OS images are
    /// prepared, built, shipped and updated as comprehensive,
    /// consistent OS images
    pub image_id: Option<String>,

    /// A lower-case string identifying the OS image version. This is
    /// supposed to be used together with `image_id` describes above,
    /// to discern different versions of the same image
    pub image_version: Option<String>,

    /// Home URL of installed OS
    pub home_url: Option<String>,

    /// Documentation URL of installed OS
    pub documentation_url: Option<String>,

    /// Support URL of installed OS
    pub support_url: Option<String>,

    /// URL for bug reports
    pub bug_report_url: Option<String>,

    /// URL with information about privacy policy of the installed OS
    pub privacy_policy_url: Option<String>,

    /// A string, specifying the name of an icon as defined by
    /// [freedesktop.org Icon Theme Specification](http://standards.freedesktop.org/icon-theme-spec/latest)
    pub logo: Option<String>,

    /// Default hostname if `hostname(5)` isn't present and no other
    /// configuration source specifies the hostname
    pub default_hostname: Option<String>,

    /// A lower-case string identifying the OS extensions support
    /// level, to indicate which extension images are supported.
    ///
    /// See [systemd-sysext(8)](https://www.freedesktop.org/software/systemd/man/249/systemd-sysext.html#) for more information
    pub sysext_level: Option<String>,
}

impl OsRelease {
    pub fn new() -> Result<Self> {
        let chunks = get_chunks_osrelease(read_to_string("/etc/os-release")?);
        let mut osr = Self::default();
        for chunk in chunks {
            parse_osrelease(&mut osr, chunk);
        }
        Ok(osr)
    }
}

impl ToJson for OsRelease {}

fn get_chunks_osrelease(contents: String) -> Vec<(Option<String>, Option<String>)> {
    contents
        .lines()
        .map(|item| {
            let mut items = item.split('=').map(sanitize_str);
            (items.next(), items.next())
        })
        .collect::<Vec<_>>()
}

fn parse_osrelease(osr: &mut OsRelease, chunk: (Option<String>, Option<String>)) {
    match chunk {
        (Some(key), Some(val)) => {
            let key = &key as &str;
            match key {
                "NAME" => osr.name = val.to_string(),
                "ID" => osr.id = Some(val.to_string()),
                "ID_LIKE" => osr.id_like = Some(val.to_string()),
                "PRETTY_NAME" => osr.pretty_name = Some(val.to_string()),
                "CPE_NAME" => osr.cpe_name = Some(val.to_string()),
                "VARIANT" => osr.variant = Some(val.to_string()),
                "VARIANT_ID" => osr.variant_id = Some(val.to_string()),
                "VERSION" => osr.version = Some(val.to_string()),
                "VERSION_CODENAME" => osr.version_codename = Some(val.to_string()),
                "VERSION_ID" => osr.version_id = Some(val.to_string()),
                "BUILD_ID" => osr.build_id = Some(val.to_string()),
                "IMAGE_ID" => osr.image_id = Some(val.to_string()),
                "IMAGE_VERSION" => osr.image_version = Some(val.to_string()),
                "HOME_URL" => osr.home_url = Some(val.to_string()),
                "DOCUMENTATION_URL" => osr.documentation_url = Some(val.to_string()),
                "SUPPORT_URL" => osr.support_url = Some(val.to_string()),
                "BUG_REPORT_URL" => osr.bug_report_url = Some(val.to_string()),
                "PRIVACY_POLICY_URL" => osr.privacy_policy_url = Some(val.to_string()),
                "LOGO" => osr.logo = Some(val.to_string()),
                "DEFAULT_HOSTNAME" => osr.default_hostname = Some(val.to_string()),
                "SYSEXT_LEVEL" => osr.sysext_level = Some(val.to_string()),
                _ => {}
            }
        }
        _ => {}
    }
}

/// System uptime
#[derive(Debug, Serialize, Clone)]
pub struct Uptime(
    /// Uptime
    pub f32,
    /// Downtime
    pub f32,
);

impl Uptime {
    pub fn new() -> Result<Self> {
        let data = read_to_string("/proc/uptime")?;
        let mut chunks = data.split_whitespace();
        match (chunks.next(), chunks.next()) {
            (Some(a), Some(b)) => Ok(Self(a.parse()?, b.parse()?)),
            _ => Err(anyhow!("`/proc/uptime` file format is incorrect!")),
        }
    }
}

impl ToPlainText for Uptime {
    fn to_plain(&self) -> String {
        format!(
            "\nUptime: {} seconds; downtime: {} seconds\n",
            self.0, self.1
        )
    }
}

/// System load (average)
#[derive(Debug, Serialize, Clone)]
pub struct LoadAVG(
    /// 1minute
    pub f32,
    /// 5minutes
    pub f32,
    /// 15minutes
    pub f32,
);

impl LoadAVG {
    pub fn new() -> Result<Self> {
        let data = read_to_string("/proc/loadavg")?;
        let mut chunks = data.split_whitespace();
        match (chunks.next(), chunks.next(), chunks.next()) {
            (Some(a), Some(b), Some(c)) => Ok(Self(a.parse()?, b.parse()?, c.parse()?)),
            _ => Err(anyhow!("`/proc/loadavg` file format is incorrect!")),
        }
    }
}

impl ToPlainText for LoadAVG {
    fn to_plain(&self) -> String {
        let mut s = format!("\nAverage system load:\n");
        s += &print_val("1 minute", &self.0);
        s += &print_val("5 minutes", &self.1);
        s += &print_val("15 minutes", &self.2);

        s
    }
}

/// Information about users
#[derive(Debug, Serialize, Clone)]
pub struct Users {
    pub users: Vec<User>,
}

impl ToJson for Users {}

impl Users {
    pub fn new() -> Result<Self> {
        let mut users = vec![];
        for user in read_to_string("/etc/passwd")?.lines() {
            match User::try_from(user) {
                Ok(user) => users.push(user),
                Err(_) => continue,
            }
        }

        Ok(Self { users })
    }
}

/// Information about followed user
#[derive(Debug, Serialize, Clone)]
pub struct User {
    /// User's login name (case-sensitive, 1-32 characters)
    pub name: String,

    /// User ID
    ///
    /// ## Examples
    /// | UID   | User name     |
    /// |:-----:|---------------|
    /// | 0     | `root`        |
    /// | 1-999 | System users  |
    /// | 1000+ | Regular users |
    pub uid: u32,

    /// Group ID links to `/etc/group` ([`Groups`]). Defines default
    /// group ownership for new files
    pub gid: u32,

    /// Optional comment field (traditionally for user info). Often
    /// holds:
    ///
    /// - Full name;
    /// - Room number;
    /// - Contact info;
    ///
    ///  Multiple entries comma-separated.
    pub gecos: Option<String>,

    /// Absolute path to the user's home directory
    pub home_dir: String,

    /// Absolute path to the user's default shell (e.g., `/bin/bash`).
    /// If set to `/usr/sbin/nologin` or `/bin/false`, the user cannot
    /// log in
    pub login_shell: String,
}

impl TryFrom<&str> for User {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let chunks = value
            .trim()
            .split(':')
            .map(sanitize_str)
            .collect::<Vec<_>>();
        if chunks.len() != 7 {
            return Err(anyhow!("Field \"{value}\" is incorrect user entry"));
        }

        Ok(Self {
            name: sanitize_str(&chunks[0]),
            uid: chunks[2].parse()?,
            gid: chunks[3].parse()?,
            gecos: match chunks[4].is_empty() {
                true => None,
                false => Some(sanitize_str(&chunks[4])),
            },
            home_dir: sanitize_str(&chunks[5]),
            login_shell: sanitize_str(&chunks[6]),
        })
    }
}

impl ToPlainText for User {
    fn to_plain(&self) -> String {
        let mut s = format!("\nUser '{}':\n", &self.name);
        s += &print_val("User ID", &self.uid);
        s += &print_val("Group ID", &self.gid);
        s += &print_opt_val("GECOS", &self.gecos);
        s += &print_val("Home directory", &self.home_dir);
        s += &print_val("Login shell", &self.login_shell);

        s
    }
}

/// Get current user string (user name)
pub fn current_user() -> Option<String> {
    std::env::var("USER").ok()
}

/// Information about groups
#[derive(Debug, Serialize, Clone)]
pub struct Groups {
    pub groups: Vec<Group>,
}

impl ToJson for Groups {}

impl Groups {
    pub fn new() -> Result<Self> {
        let mut groups = vec![];
        for group in read_to_string("/etc/group")?.lines() {
            match Group::try_from(group) {
                Ok(group) => groups.push(group),
                Err(_) => continue,
            }
        }
        Ok(Self { groups })
    }
}

/// Information about followed group
#[derive(Debug, Serialize, Clone)]
pub struct Group {
    /// Group name
    pub name: String,

    /// Group ID
    pub gid: u32,

    /// List of users (links to `/etc/passwd` ([`Users`])) in this
    /// group
    pub users: Vec<String>,
}

impl TryFrom<&str> for Group {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let chunks = value
            .trim()
            .split(':')
            .map(sanitize_str)
            .collect::<Vec<_>>();
        if chunks.len() != 4 {
            return Err(anyhow!("Field \"{value}\" is incorrect group entry"));
        }

        Ok(Self {
            name: chunks[0].to_string(),
            gid: chunks[2].parse()?,
            users: {
                let mut users = vec![];
                for user in chunks[3].split(',') {
                    if !user.is_empty() {
                        users.push(user.to_string());
                    }
                }
                users
            },
        })
    }
}

/// List of installed console shells
pub type Shells = Vec<String>;

fn get_shells() -> Result<Shells> {
    let mut shells = vec![];
    for shell in read_to_string("/etc/shells")?
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        shells.push(shell.to_string());
    }
    Ok(shells)
}

/// Host name
pub type HostName = String;

pub fn get_hostname() -> Option<HostName> {
    match read_to_string("/etc/hostname") {
        Ok(s) => Some(sanitize_str(&s)),
        Err(_) => None,
    }
}

/// Information about current locale
#[derive(Debug, Serialize, Clone)]
pub struct Locale {}

fn sanitize_str(s: &str) -> String {
    s.trim().replace('"', "").replace('\'', "")
}

/// Linux kernel modules list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KModules {
    pub modules: Vec<Module>,
}

impl ToJson for KModules {}

impl KModules {
    pub fn new() -> Result<Self> {
        let contents = read_to_string("/proc/modules")?;
        let contents = contents.lines();
        let mut modules = Vec::new();

        for s in contents {
            modules.push(Module::try_from(s)?);
        }

        Ok(Self { modules })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Module {
    /// The name of the loaded kernel module
    pub name: String,

    /// Size of the module
    pub size: Size,

    /// Number of times the module is currently in use or loaded
    pub instances: usize,

    /// A comma-separated list of other modules that this module
    /// depends on
    pub dependencies: String,

    /// The current state of the module
    pub state: String,

    /// The memory addresses where the module is loaded (may not
    /// always be present or fully detailed depending on the kernel
    /// version and configuration)
    pub memory_addrs: String,
}

impl TryFrom<&str> for Module {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        let mut ch = value.split_whitespace();
        match (
            ch.next(),
            ch.next(),
            ch.next(),
            ch.next(),
            ch.next(),
            ch.next(),
        ) {
            (
                Some(name),
                Some(size),
                Some(instances),
                Some(dependencies),
                Some(state),
                Some(memory_addrs),
            ) => {
                let size = size.parse::<u64>().map_err(|err| anyhow!("{err}"))?;
                let instances = instances.parse::<usize>().map_err(|err| anyhow!("{err}"))?;

                Ok(Self {
                    name: name.to_string(),
                    size: Size::B(size),
                    instances,
                    dependencies: dependencies.to_string(),
                    state: state.to_string(),
                    memory_addrs: memory_addrs.to_string(),
                })
            }
            _ => Err(anyhow!("Unknown field: \"{value}\"")),
        }
    }
}

pub fn get_current_desktop() -> Option<String> {
    var("XDG_CURRENT_DESKTOP").ok()
}

pub fn get_lang() -> Option<String> {
    let lang = var("LANG").ok();
    let lc_all = var("LC_ALL").ok();
    if lang.is_some() { lang } else { lc_all }
}

pub fn get_env_vars() -> Vec<(String, String)> {
    let mut vars = vars().collect::<Vec<(String, String)>>();
    vars.sort_by_key(|v| v.0.clone());
    vars
}

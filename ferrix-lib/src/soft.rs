/* soft.rs
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

//! Installed software list

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::Path, process::Command};

use crate::traits::ToJson;

/// Type of installed Linux distro packaging system or concrete package
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum PkgType {
    /// System with `dpkg` package manager only
    Deb,

    /// System with `rpm` package manager only
    Rpm,

    /// System with `dpkg` and `rpm` package managers
    DebRpm, // If deb and rpm package managers is installed

    /// Unknown and/or unsupported Linux distro
    Other,
}

impl PkgType {
    const BINARY_PATHES: &[&str] = &[
        "/bin/",
        "/usr/bin/",
        "/usr/local/bin/",
        "/sbin/",
        "/usr/sbin/",
        "/usr/local/sbin/",
    ];

    fn is_deb() -> bool {
        for dir in Self::BINARY_PATHES {
            if Path::new(*dir).join("dpkg-query").exists() {
                return true;
            }
        }
        false
    }

    fn is_rpm() -> bool {
        for dir in Self::BINARY_PATHES {
            if Path::new(*dir).join("rpm").exists() {
                return true;
            }
        }
        false
    }

    /// Detect Linux packaging system
    pub fn detect() -> Self {
        let deb = Self::is_deb();
        let rpm = Self::is_rpm();

        if deb && rpm {
            Self::DebRpm
        } else if deb {
            Self::Deb
        } else if rpm {
            Self::Rpm
        } else {
            Self::Other
        }
    }
}

impl From<&str> for PkgType {
    fn from(value: &str) -> Self {
        // dbg!(value);
        match &value.replace("'", "") as &str {
            "<DEB>" => Self::Deb,
            "<RPM>" => Self::Rpm,
            _ => Self::Other,
        }
    }
}

impl Display for PkgType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Deb => "deb",
                Self::Rpm => "rpm",
                Self::DebRpm => "deb+rpm",
                Self::Other => "unknown",
            }
        )
    }
}

/// List of installed software
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstalledPackages {
    /// Package list
    pub packages: Vec<Package>,
}

impl ToJson for InstalledPackages {}

impl InstalledPackages {
    pub fn get() -> Result<Self> {
        let pkg_type = PkgType::detect();

        match pkg_type {
            PkgType::Deb => Self::get_deb_packages(),
            PkgType::Rpm => Self::get_rpm_packages(),
            PkgType::DebRpm => {
                let mut deb = Self::get_deb_packages()?;
                let mut rpm = Self::get_rpm_packages()?;
                deb.packages.append(&mut rpm.packages);

                Ok(deb)
            }
            PkgType::Other => Err(anyhow!(
                "Unsupported packaging system type! Supports only `deb` and `rpm` package types."
            )),
        }
    }

    fn command(args: &[&str]) -> Result<Self> {
        let pkglist = Command::new("/bin/env").args(args).output()?;
        let pkglist_stdout = String::from_utf8(pkglist.stdout)?;
        let mut packages = Vec::new();

        for pkg_str in pkglist_stdout.lines().map(|line| line.trim()) {
            let package = Package::try_from(pkg_str);
            if let Ok(pkg) = package {
                packages.push(pkg);
            }
        }

        Ok(Self { packages })
    }

    fn get_deb_packages() -> Result<Self> {
        Self::command(&[
            "dpkg-query",
            "-W",
            "-f='<DEB>\t${Package}\t${Version}\t${Architecture}\n",
        ])
    }

    fn get_rpm_packages() -> Result<Self> {
        Self::command(&[
            "rpm",
            "-qa",
            "--queryformat='<RPM>\t%{NAME}\t%{VERSION}\t%{ARCH}\n'",
        ])
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub pkg_type: PkgType,
}

impl TryFrom<&str> for Package {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        // dbg!(value);
        let mut chunks = value.trim().split('\t').map(|s| s.trim().replace("'", ""));

        match (chunks.next(), chunks.next(), chunks.next(), chunks.next()) {
            (Some(pkg), Some(name), Some(version), Some(arch)) => Ok(Self {
                pkg_type: PkgType::from(pkg.as_str()),
                name,
                version,
                arch,
            }),
            _ => Err(anyhow!("String \"{value}\" has incorrect format!")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkg_list_test() {
        let pkgs = InstalledPackages::get();
        dbg!(&pkgs);
        assert!(pkgs.is_ok());
    }
}

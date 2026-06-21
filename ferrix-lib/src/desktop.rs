/* desktop.rs
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

//! Get information about desktop environment

use crate::traits::ToJson;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path, process::Command};

/// Session info (desktop, window manager)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub desktop: Option<String>,
    pub desktop_ver: Option<String>,
    pub window_manager: Option<String>,
}

impl ToJson for SessionInfo {}

impl SessionInfo {
    pub fn new() -> Result<Self> {
        let desktop = Self::get_desktop();
        let desktop_ver = match &desktop {
            Some(de) => Self::get_desktop_version(de),
            _ => None,
        };

        Ok(Self {
            desktop: if let Some(ref de) = desktop {
                Some(Self::format_desktop_name(de))
            } else {
                desktop
            },
            desktop_ver,
            window_manager: None,
        })
    }

    pub fn get_desktop() -> Option<String> {
        if let Ok(session) = env::var("DESKTOP_SESSION") {
            if &session == "regolith" {
                return Some("Regolith".to_string());
            }
        }

        if let Some(de) = Self::get_de_from_env() {
            return Some(de);
        }
        if let Some(de) = Self::get_de_from_xprop()
            && env::var("DISPLAY").is_ok()
        {
            return Some(de);
        }
        None
    }

    fn get_de_from_env() -> Option<String> {
        if let Ok(xdg) = env::var("XDG_CURRENT_DESKTOP") {
            let mut de = xdg.replace("X-", "");
            de = de.replace("Budgie:GNOME", "Budgie");
            de = de.replace(":Unity7:ubuntu", "");
            return Some(de);
        }

        if let Ok(session) = env::var("DESKTOP_SESSION") {
            let de = Path::new(&session)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&session)
                .to_string();
            return Some(de);
        }

        if env::var("GNOME_DESKTOP_SESSION_ID").is_ok() {
            return Some("GNOME".to_string());
        }
        if env::var("MATE_DESKTOP_SESSION_ID").is_ok() {
            return Some("MATE".to_string());
        }
        if env::var("TDE_FULL_SESSION").is_ok() {
            return Some("Trinity Desktop".to_string());
        }
        None
    }

    fn get_de_from_xprop() -> Option<String> {
        let out = Command::new("xprop").arg("-root").output().ok()?;
        let stdout = String::from_utf8_lossy(&out.stdout);

        for line in stdout.lines() {
            if line.contains("KDE_SESSION_VERSION")
                || line.contains("_MUFFIN")
                || line.contains("xfce")
            {
                return Some(line.to_string());
            }
        }
        None
    }

    fn format_desktop_name(de: &str) -> String {
        let de = de.to_string();

        if de.starts_with("KDE_SESSION_VERSION") {
            if let Some(pos) = de.find(" = ") {
                return format!("KDE{}", &de[pos + 3..]);
            }
        }

        if de.contains("xfce4") {
            return "Xfce4".to_string();
        }
        if de.contains("xfce5") {
            return "Xfce5".to_string();
        }
        if de.contains("xfce") {
            return "Xfce".to_string();
        }
        if de.contains("mate") {
            return "MATE".to_string();
        }
        if de.contains("GNOME") {
            return "GNOME".to_string();
        }
        if de.contains("MUFFIN") {
            return "Cinnamon".to_string();
        }
        de
    }

    pub fn get_desktop_version(de: &str) -> Option<String> {
        let cmd = if de.starts_with("Plasma") {
            Some("plasmashell")
        } else if de.starts_with("MATE") {
            Some("mate-session")
        } else if de.starts_with("Xfce") {
            Some("xfce4-session")
        } else if de.starts_with("GNOME") {
            Some("gnome-shell")
        } else if de.starts_with("Cinnamom") {
            Some("cinnamon")
        } else if de.starts_with("Budgie") {
            Some("budgie-desktop")
        } else if de.starts_with("LXQt") {
            Some("lxqt-session")
        } else if de.starts_with("Lumina") {
            Some("lumina-desktop")
        } else if de.starts_with("Trinity") {
            Some("tde-config")
        } else if de.starts_with("Unity") {
            Some("unity")
        } else if de.starts_with("Deepin") {
            // WARN: Special case
            if let Ok(contents) = fs::read_to_string("/etc/deepin-version") {
                for line in contents.lines() {
                    if line.starts_with("Version=") {
                        return Some(line[8..].to_string());
                    }
                }
            }
            None
        } else {
            None
        };

        if let Some(cmd) = cmd {
            let out = Command::new(cmd).arg("--version").output().ok()?;

            let mut version = String::from_utf8_lossy(&out.stdout).to_string();
            if version.is_empty() {
                version = String::from_utf8_lossy(&out.stderr).to_string();
            }

            version = version
                .replace("TDE:", "")
                .replace("tde-config", "")
                .replace("liblxqt", "")
                .replace("Copyright", "")
                .replace(")", "");

            if let Some(last) = version.split_whitespace().last() {
                return Some(last.trim_matches('"').to_string());
            }
        }
        None
    }
}

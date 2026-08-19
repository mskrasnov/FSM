/* utils.rs
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

//! Utilities and helpers

use std::{fmt::Display, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub enum Size {
    B(u64),
    KB(f32),
    MB(f32),
    GB(f32),
    TB(f32),
    UnknownUnits(u64),
    #[default]
    None,
}

impl Size {
    fn get_num(&self) -> Option<f32> {
        match self {
            Self::B(num) | Self::UnknownUnits(num) => Some(*num as f32),
            Self::KB(num) | Self::MB(num) | Self::GB(num) | Self::TB(num) => Some(*num),
            _ => None,
        }
    }

    /// base: 2 or 10
    pub fn round(&self, base: u8) -> Option<Self> {
        if base != 2 && base != 10 {
            panic!("Unknown base: {base} (supported values: 2 or 10)")
        }

        let mut size = *self;
        let num = size.get_num();

        if let None = num {
            return None;
        }
        let mut num = num.unwrap();
        let div = match base {
            2 => 1024.,
            10 => 1000.,
            _ => panic!("Unknown base: {base} (supported values: 2 or 10)"), // unreachable branch
        };

        while num >= div {
            if let Self::TB(_) = size {
                break;
            } else {
                size = match size {
                    Size::B(_) => {
                        num /= div;
                        Size::KB(num)
                    }
                    Size::KB(_) => {
                        num /= div;
                        Size::MB(num)
                    }
                    Size::MB(_) => {
                        num /= div;
                        Size::GB(num)
                    }
                    Size::GB(_) => {
                        num /= div;
                        Size::TB(num)
                    }
                    _ => size,
                }
            }
        }
        Some(size)
    }

    pub fn get_bytes10(&self) -> Option<u64> {
        match self {
            Self::B(b) => Some(*b),
            Self::KB(kb) => Some((*kb * 10f32.powi(3)) as u64),
            Self::MB(mb) => Some((*mb * 10f32.powi(6)) as u64),
            Self::GB(gb) => Some((*gb * 10f32.powi(9)) as u64),
            Self::TB(tb) => Some((*tb * 10f32.powi(12)) as u64),
            _ => None,
        }
    }

    pub fn get_bytes2(&self) -> Option<u64> {
        match self {
            Self::B(b) => Some(*b),
            Self::KB(kb) => Some((*kb * 2f32.powi(10)) as u64),
            Self::MB(mb) => Some((*mb * 2f32.powi(20)) as u64),
            Self::GB(gb) => Some((*gb * 2f32.powi(30)) as u64),
            Self::TB(tb) => Some((*tb * 2f32.powi(40)) as u64),
            _ => None,
        }
    }
}

impl TryFrom<&str> for Size {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut items = value.split_whitespace();
        Ok(match (items.next(), items.next()) {
            (Some(num), Some(units)) => {
                let num = num.parse::<f32>()?;
                match units.to_lowercase().as_str() {
                    "kb" | "kbytes" => Self::KB(num),
                    "mb" | "mbytes" => Self::MB(num),
                    "gb" | "gbytes" => Self::GB(num),
                    "tb" | "tbytes" => Self::TB(num),
                    _ => Self::B(num as u64),
                }
            }
            (Some(num), None) => {
                let num = num.parse::<u64>()?;
                Self::UnknownUnits(num)
            }
            _ => Self::None,
        })
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Size::B(n) => format!("{n}  B"),
                Size::KB(n) => format!("{n:.2} KB"),
                Size::MB(n) => format!("{n:.2} MB"),
                Size::GB(n) => format!("{n:.2} GB"),
                Size::TB(n) => format!("{n:.2} TB"),
                Size::UnknownUnits(n) => format!("{n} ??"),
                Size::None => format!("None"),
            }
        )
    }
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let c = std::fs::read_to_string(path)?.trim().to_string();
    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bytes_test() {
        let s1 = Size::B(1024);
        assert_eq!(s1.get_bytes10().unwrap_or(0), 1024);
        let s2 = Size::KB(1.);
        assert_eq!(s2.get_bytes2().unwrap(), 1024);
    }
}

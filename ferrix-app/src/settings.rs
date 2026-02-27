/* settings.rs
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

use anyhow::Result;
use iced::{Theme, color};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, fs, path::Path};

use crate::{fl, styles::CPU_CHARTS_COLORS};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FXSettings {
    pub update_period: u8,
    pub charts_update_period_nsecs: u8,
    pub style: Style,
    pub chart_line_thickness: ChartLineThickness,
    pub chart_colors: ChartColors,
}

impl FXSettings {
    pub fn read<P: AsRef<Path>>(pth: P) -> Result<Self> {
        let contents = fs::read_to_string(pth)?;
        let data = toml::from_str(&contents)?;
        Ok(data)
    }

    pub fn write<'a, P: AsRef<Path>>(&'a self, pth: P) -> Result<()> {
        let contents = toml::to_string(&self)?;
        fs::write(pth, contents)?;
        Ok(())
    }
}

impl Default for FXSettings {
    fn default() -> Self {
        Self {
            update_period: 1,
            charts_update_period_nsecs: 5,
            style: Style::default(),
            chart_line_thickness: ChartLineThickness::default(),
            chart_colors: ChartColors::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq)]
pub enum Style {
    Light,
    #[default]
    Dark,
}

impl Style {
    pub const ALL: &[Self] = &[Self::Light, Self::Dark];

    pub fn to_theme(&self) -> Theme {
        match self {
            Self::Light => {
                let mut palette = Theme::GruvboxLight.palette();
                palette.success = color!(0x98971a);
                palette.danger = color!(0xaf3a03);
                palette.warning = color!(0xb57614);
                palette.primary = color!(0xd79921);
                palette.background = color!(0xebdbb2);

                Theme::custom("Ferrix Light Theme", palette)
            }
            Self::Dark => {
                let mut palette = Theme::GruvboxDark.palette();
                palette.success = color!(0x98971a);
                palette.danger = color!(0xfb4934);
                palette.warning = color!(0xfabd2f);
                palette.primary = color!(0xfabd2f);

                Theme::custom("Ferrix Dark Theme", palette)
            }
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Light => fl!("style-light"),
                Self::Dark => fl!("style-dark"),
            }
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Deserialize, Serialize)]
pub enum ChartLineThickness {
    #[default]
    OnePixel,
    TwoPixel,
}

impl ChartLineThickness {
    pub const ALL: &[Self] = &[Self::OnePixel, Self::TwoPixel];

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::OnePixel => 1,
            Self::TwoPixel => 2,
        }
    }
}

impl Display for ChartLineThickness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::OnePixel => fl!("lthick-one"),
                Self::TwoPixel => fl!("lthick-two"),
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartColors {
    pub colors: HashMap<String, (u8, u8, u8)>,
    // pub default_colors: Vec<(u8, u8, u8)>,
}

impl Default for ChartColors {
    fn default() -> Self {
        Self {
            colors: {
                let mut colors = HashMap::new();
                let mut i = 0;
                for color in CPU_CHARTS_COLORS {
                    let c = color.into_rgba8();
                    let c = (c[0], c[1], c[2]);
                    colors.insert(format!("CPU #{i}"), c);
                    i += 1;
                }
                colors
            },
            // default_colors: CPU_CHARTS_COLORS
            //     .iter()
            //     .map(|c| {
            //         let c = c.into_rgba8();
            //         (c[0], c[1], c[2])
            //     })
            //     .collect(),
        }
    }
}

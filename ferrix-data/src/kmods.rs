/* kmods.rs
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

//! Kernel modules Service Provider

use crate::{
    FromJson,
    load_state::{LoadState, ToLoadState},
    polkit::*,
};
use anyhow::Result;
use ferrix_lib::sys::KModules;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum KResult {
    Ok { data: KModules },
    Err { error: String },
}

impl KResult {
    pub fn new() -> Self {
        match KModules::new() {
            Ok(mut data) => {
                sort_kmods(&mut data);
                Self::Ok { data }
            }
            Err(why) => Self::Err {
                error: why.to_string(),
            },
        }
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn to_load_state(self) -> LoadState<KModules> {
        match self {
            Self::Ok { data } => LoadState::Loaded(data),
            Self::Err { error } => LoadState::Error(error),
        }
    }
}

impl FromJson for KResult {}

pub async fn get_kmods_list() -> LoadState<KResult> {
    let json = get_data("kmods".to_string()).await;
    match json {
        LoadState::Loaded(json) => KResult::from_json(json).to_load_state(),
        LoadState::Error(why) => LoadState::Error(why),
        _ => LoadState::Loading,
    }
}

fn sort_kmods<'a>(data: &'a mut KModules) {
    data.modules.sort_by_key(|module| module.name.clone());
}

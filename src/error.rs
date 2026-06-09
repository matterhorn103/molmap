// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::Entity;

#[derive(thiserror::Error, Debug)]
pub enum MolMapError {
    #[error("The Id was not found in the Map")]
    Id(Entity),
}

pub type MolMapResult<T> = core::result::Result<T, MolMapError>;

// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod building;
mod map;
mod membership;
mod positions;

use crate::units::{Angstrom, Relative};

pub use map::SpatialMolMap;
pub(crate) use positions::MolecularPositions;

/// A [`MolMap`] extended to also hold the two-dimensional positions of its entities.
pub type MolMap2 = SpatialMolMap<2, Relative>;

/// A [`MolMap`] extended to also hold the three-dimensional positions of its entities.
pub type MolMap3 = SpatialMolMap<3, Angstrom>;

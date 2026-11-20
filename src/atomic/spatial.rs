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

pub(crate) use positions::AtomicPositions;

pub use map::SpatialAtomMap;

/// An [`AtomMap`] extended to also hold the two-dimensional positions of its entities.
pub type AtomMap2 = SpatialAtomMap<2, Relative>;

/// An [`AtomMap`] extended to also hold the three-dimensional positions of its entities.
pub type AtomMap3 = SpatialAtomMap<3, Angstrom>;

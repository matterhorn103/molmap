// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functionality generic over any spatial `AtomMap`.

use nalgebra as na;
use nalgebra::{Point, SVector};

/// Calculates the mean position from a set of positions, or `None` if the iterator is empty.
pub(crate) fn mean_point<'a, I, const D: usize>(positions: I) -> Option<Point<f64, D>>
where
    I: IntoIterator<Item = &'a Point<f64, D>>,
{
    let mut count: u32 = 0;
    let mut sum: SVector<f64, D> = SVector::zeros();
    for pos in positions {
        count += 1;
        sum += pos.coords
    }
    if count == 0 {
        None
    } else {
        let avg = sum / f64::from(count);
        Some(Point::from(avg))
    }
}

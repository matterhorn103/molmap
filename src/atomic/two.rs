// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nalgebra as na;
use slotmap::SecondaryMap;

use super::AtomGraph;

use crate::{
    entities::*,
    traits::{Spatial, TwoDimensional},
};

pub struct AtomMap2 {
    graph: AtomGraph,
    atom_positions: SecondaryMap<AtomKey, na::Point2<f64>>,
    pseudoatom_positions: SecondaryMap<PseudoatomKey, na::Point2<f64>>,
}

impl Spatial for AtomMap2 {
    const DIM: usize = 2;

    type Dim = na::Const<2>;

    type Point = na::Point2<f64>;

    type Vector = na::Vector2<f64>;
}

impl TwoDimensional for AtomMap2 {}

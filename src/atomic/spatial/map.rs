// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::marker::PhantomData;

use slotmap::SecondaryMap;

use crate::{
    entities::*,
    geometry::Point,
    traits::{Map, Spatial},
    units::Unit,
};

use super::super::{AtomMap, Atomic};

/// An [`AtomMap`] extended to also hold the positions of its entities in `D`-dimensional space.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpatialAtomMap<const D: usize, U> {
    pub(super) graph: AtomMap,
    pub(super) atom_positions: SecondaryMap<AtomKey, Point<D>>,
    pub(super) pseudoatom_positions: SecondaryMap<PseudoatomKey, Point<D>>,
    pub(super) unit: PhantomData<U>,
}

impl<const D: usize, U> SpatialAtomMap<D, U> {
    pub fn new() -> Self {
        Self {
            graph: AtomMap::new(),
            atom_positions: SecondaryMap::new(),
            pseudoatom_positions: SecondaryMap::new(),
            unit: PhantomData,
        }
    }
}

impl<const D: usize, U> Map for SpatialAtomMap<D, U> {
    type Graph = AtomMap;

    fn into_graph(self) -> Self::Graph {
        self.graph
    }

    fn as_graph(&self) -> &Self::Graph {
        &self.graph
    }

    fn contains<E: Entity>(&self, entity: E) -> bool {
        self.contains(entity)
    }
}

impl<const D: usize, U: Unit> Spatial<D> for SpatialAtomMap<D, U> {
    type Unit = U;
}

impl<const D: usize, U> Atomic for SpatialAtomMap<D, U> {
    fn into_atomic(self) -> AtomMap {
        self.graph
    }

    fn as_atomic(&self) -> &AtomMap {
        &self.graph
    }
}

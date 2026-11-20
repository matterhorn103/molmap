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
    atomic::{AtomMap, Atomic},
    entities::*,
    geometry::Point,
    traits::{Map, Spatial},
    units::Unit,
};

use super::super::{MolMap, Molecular};

/// A [`MolMap`] extended to also hold the positions of its entities in `D`-dimensional space.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
pub struct SpatialMolMap<const D: usize, U> {
    pub(super) graph: MolMap,
    pub(super) atom_positions: SecondaryMap<AtomKey, Point<D>>,
    pub(super) pseudoatom_positions: SecondaryMap<PseudoatomKey, Point<D>>,
    unit: PhantomData<U>,
}

impl<const D: usize, U> SpatialMolMap<D, U> {
    pub fn new() -> Self {
        Self {
            graph: MolMap::new(),
            atom_positions: SecondaryMap::new(),
            pseudoatom_positions: SecondaryMap::new(),
            unit: PhantomData,
        }
    }
}

impl<const D: usize, U> Map for SpatialMolMap<D, U> {
    type Graph = MolMap;

    fn into_graph(self) -> Self::Graph {
        self.graph
    }

    fn as_graph(&self) -> &Self::Graph {
        &self.graph
    }

    fn contains<E: Entity>(&self, entity: E) -> bool {
        self.as_graph().contains(entity)
    }
}

impl<const D: usize, U: Unit> Spatial<D> for SpatialMolMap<D, U> {
    type Unit = U;
}

impl<const D: usize, U> Atomic for SpatialMolMap<D, U> {
    fn into_atomic(self) -> AtomMap {
        self.graph.into_atomic()
    }

    fn as_atomic(&self) -> &AtomMap {
        self.graph.as_atomic()
    }
}

impl<const D: usize, U> Molecular for SpatialMolMap<D, U> {
    fn into_molecular(self) -> MolMap {
        self.graph
    }

    fn as_molecular(&self) -> &Self::Graph {
        &self.graph
    }
}

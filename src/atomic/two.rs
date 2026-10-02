// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nalgebra::{self as na, Point2, SVector, Vector2};
use slotmap::SecondaryMap;

use super::AtomGraph;

use crate::{BondType, Element, Pseudoelement, entities::*, error::*, traits::*, view::*};

pub struct AtomMap2 {
    graph: AtomGraph,
    atom_positions: SecondaryMap<AtomKey, na::Point2<f64>>,
    pseudoatom_positions: SecondaryMap<PseudoatomKey, na::Point2<f64>>,
}

impl CoreGraph for AtomMap2 {
    type Graph = AtomGraph;

    fn graph(&self) -> &Self::Graph {
        &self.graph
    }

    fn graph_mut(&mut self) -> &mut Self::Graph {
        &mut self.graph
    }
}

impl Map for AtomMap2 {
    fn new() -> Self {
        Self {
            graph: AtomGraph::new(),
            atom_positions: SecondaryMap::new(),
            pseudoatom_positions: SecondaryMap::new(),
        }
    }

    fn contains<E: Entity>(&self, entity: E) -> bool {
        self.graph.contains(entity)
    }
}

impl Spatial for AtomMap2 {
    const DIM: usize = 2;

    type Dim = na::Const<2>;

    type Scalar = f64;

    type Point = na::Point2<f64>;

    type Vector = na::Vector2<f64>;
}

impl TwoDimensional for AtomMap2 {}

/// Methods for entity addition.
impl AtomMap2 {
    /// Adds an atom to the map at the given position.
    pub fn add_atom(&mut self, element: Element, position: Point2<f64>) -> Atom {
        let atom = self.graph.add_atom(element);
        self.set_position(atom, position);
        atom
    }

    /// Adds a pseudoatom to the map at the given position.
    pub fn add_pseudoatom(
        &mut self,
        pseudoelement: Pseudoelement,
        position: Point2<f64>,
    ) -> Pseudoatom {
        let pseudoatom = self.graph.add_pseudoatom(pseudoelement);
        self.set_position(pseudoatom, position);
        pseudoatom
    }

    /// Creates a new (single covalent) bond between two bondable entities.
    ///
    /// # Errors
    ///
    /// Fails if either of `start` and `end` are invalid.
    pub fn add_bond<A, B>(&mut self, bond_type: BondType, start: A, end: B) -> MolMapResult<Bond>
    where
        A: Bondable,
        B: Bondable,
    {
        if !self.contains(start) {
            return Err(MolMapError::InvalidId(start.as_entity()));
        } else if !self.contains(end) {
            return Err(MolMapError::InvalidId(end.as_entity()));
        };
        Ok(self.graph.add_bond(bond_type, start, end))
    }

    /// Creates a new single covalent bond between two bondable entities.
    ///
    /// # Errors
    ///
    /// Fails if either of `start` and `end` are invalid.
    pub fn add_single_bond<A, B>(&mut self, start: A, end: B) -> MolMapResult<Bond>
    where
        A: Bondable,
        B: Bondable,
    {
        self.add_bond(BondType::Covalent { order: 1.0 }, start, end)
    }

    /// Creates a new double covalent bond between two bondable entities.
    ///
    /// # Errors
    ///
    /// Fails if either of `start` and `end` are invalid.
    pub fn add_double_bond<A, B>(&mut self, start: A, end: B) -> MolMapResult<Bond>
    where
        A: Bondable,
        B: Bondable,
    {
        self.add_bond(BondType::Covalent { order: 2.0 }, start, end)
    }
}

// Public API for deleting entities/changing their collection membership via mutable views
//
// For now these are identical to the equivalents for MolMap0, but may well
// diverge in future e.g. if we cache emergent information like the bond vectors
// or the centroids or bounding boxes of the collections.
//
// It is not necessary to delete the positions from the SecondaryMap; they will
// be overwritten.

impl<'m> ViewMut<'m, AtomMap2, Atom> {
    /// Removes the atom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.graph_mut().delete_atom(self.id);
    }
}

impl<'m> ViewMut<'m, AtomMap2, Pseudoatom> {
    /// Removes the pseudoatom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.graph_mut().delete_pseudoatom(self.id);
    }
}

impl<'m> ViewMut<'m, AtomMap2, Bond> {
    /// Removes the bond from the map (but not its bonding partners).
    pub fn delete(self) {
        self.map.graph_mut().delete_bond(self.id);
    }
}

// Methods for accessing and calculating positions.
impl AtomMap2 {
    /// Returns the position of the given atom or pseudoatom.
    ///
    /// A stale (i.e. invalid) ID may return a position, as spatial data is not
    /// automatically cleaned up. The caller is responsible for not making such a
    /// request.
    ///
    /// # Panics
    ///
    /// Panics if the atomlike has no stored position (usually meaning that it is not
    /// in the map).
    pub(crate) fn atomlike_position(&self, atomlike: impl Atomlike) -> &Point2<f64> {
        match Atomlike::to_resolved(atomlike) {
            ResolvedAtomlike::Atom(atom) => self.position(atom).unwrap(),
            ResolvedAtomlike::Pseudoatom(pseudoatom) => self.position(pseudoatom).unwrap(),
        }
    }

    /// Calculates the vector of the line between two atomlikes, from `a` to `b`.
    ///
    /// Stale (i.e. invalid) IDs may return positions, as spatial data is not
    /// automatically cleaned up. The caller is responsible for not making such a
    /// request.
    ///
    /// # Panics
    ///
    /// Panics if either atomlike has no stored position (usually meaning that it is not
    /// in the map).
    #[allow(unused)]
    pub(crate) fn interatomlike_line(&self, a: impl Atomlike, b: impl Atomlike) -> Vector2<f64> {
        self.atomlike_position(b) - self.atomlike_position(a)
    }

    /// Returns the position of the [`Bondable`] from which the bond starts.
    ///
    /// # Panics
    ///
    /// Panics if the bond is not in the map.
    pub(crate) fn bond_origin(&self, bond: Bond) -> &Point2<f64> {
        match self.graph().data(bond).unwrap().start.resolve() {
            ResolvedBondable::Atom(atom) => self.position(atom).unwrap(),
            ResolvedBondable::Pseudoatom(pseudoatom) => self.position(pseudoatom).unwrap(),
        }
    }

    /// Returns the position of the [`Bondable`] at which the bond ends.
    ///
    /// # Panics
    ///
    /// Panics if the bond is not in the map.
    pub(crate) fn bond_terminus(&self, bond: Bond) -> &Point2<f64> {
        match self.graph().data(bond).unwrap().end.resolve() {
            ResolvedBondable::Atom(atom) => self.position(atom).unwrap(),
            ResolvedBondable::Pseudoatom(pseudoatom) => self.position(pseudoatom).unwrap(),
        }
    }

    /// Calculates the midpoint of the bond.
    ///
    /// # Panics
    ///
    /// Panics if the bond is not in the map.
    pub(crate) fn bond_midpoint(&self, bond: Bond) -> Point2<f64> {
        na::center(self.bond_origin(bond), self.bond_terminus(bond))
    }

    /// Calculates the vector that the bond follows from its origin to its terminus.
    ///
    /// # Panics
    ///
    /// Panics if the bond is not in the map.
    pub(crate) fn bond_vector(&self, bond: Bond) -> Vector2<f64> {
        self.bond_terminus(bond) - self.bond_origin(bond)
    }
}

impl StoresPosition<Atom> for AtomMap2 {
    fn position(&self, entity: Atom) -> Option<&Point2<f64>> {
        // When an entity is removed from the map, any spatial data is not deleted.
        // Thus it is possible to obtain a stale position from the SecondaryMap that does
        // not correspond to any existing entity.
        //
        // If the entity ID is stale, it's not possible for the user to construct a
        // view of it. When working internally, it would be possible to get a spurious
        // position from a stale ID; to help avoid that, this method confirms the validity
        // of the ID first.
        if self.contains(entity) {
            self.atom_positions.get(entity.into())
        } else {
            None
        }
    }

    fn set_position(&mut self, entity: Atom, new: Self::Point) -> Option<Point2<f64>> {
        // Here, slotmap's insert method takes care of the checking for us
        // The promised behaviour of set_position matches that of slotmap::insert
        self.atom_positions.insert(entity.into(), new)
    }
}

impl StoresPosition<Pseudoatom> for AtomMap2 {
    fn position(&self, entity: Pseudoatom) -> Option<&Point2<f64>> {
        if self.contains(entity) {
            self.pseudoatom_positions.get(entity.into())
        } else {
            None
        }
    }

    fn set_position(&mut self, entity: Pseudoatom, new: Self::Point) -> Option<Point2<f64>> {
        self.pseudoatom_positions.insert(entity.into(), new)
    }
}

impl<'m> View<'m, AtomMap2, Atom> {
    /// Returns the position of the atom.
    pub fn position(&self) -> &Point2<f64> {
        self.map.position(self.id).unwrap()
    }
}

impl<'m> ViewMut<'m, AtomMap2, Atom> {
    /// Sets the position of the atom to the provided value.
    pub fn set_position(self, position: Point2<f64>) {
        self.map.set_position(self.id, position);
    }
}

impl<'m> View<'m, AtomMap2, Pseudoatom> {
    /// Returns the position of the pseudoatom.
    pub fn position(&self) -> &Point2<f64> {
        self.map.position(self.id).unwrap()
    }
}

impl<'m> ViewMut<'m, AtomMap2, Pseudoatom> {
    /// Sets the position of the pseudoatom to the provided value.
    pub fn set_position(self, position: Point2<f64>) {
        self.map.set_position(self.id, position);
    }
}

impl<'m> View<'m, AtomMap2, Bond> {
    /// Returns the position of the [`Bondable`] from which the bond starts.
    pub fn origin(&self) -> &Point2<f64> {
        self.map.bond_origin(self.id)
    }

    /// Returns the position of the [`Bondable`] at which the bond ends.
    pub fn terminus(&self) -> &Point2<f64> {
        self.map.bond_terminus(self.id)
    }

    /// Calculates the midpoint of the bond.
    pub fn midpoint(&self) -> Point2<f64> {
        self.map.bond_midpoint(self.id)
    }

    /// Calculates the vector that the bond follows from its origin to its terminus.
    pub fn vector(&self) -> Vector2<f64> {
        self.map.bond_vector(self.id)
    }

    /// Calculates the distance from the bond's origin to its terminus.
    pub fn length(&self) -> f64 {
        self.vector().magnitude()
    }
}

#[cfg(test)]
#[allow(unused)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn add_atom() {
        let mut am = AtomMap2::new();
        assert_eq!(am.entities::<Atom>().count(), 0);
        let h1 = am.add_atom(Element::H, na::Point2::new(1.0, 2.0));
        assert_eq!(am.entities::<Atom>().count(), 1);
        // Confirm that the atom positions also have data for a single atom
        assert_eq!(am.atom_positions.len(), 1);
        // Confirm that the atom position can be accessed
        assert_eq!(
            am.atom_positions.get(h1.to_key()).unwrap().clone(),
            na::Point2::new(1.0, 2.0)
        );
    }

    #[test]
    fn delete_atom() {
        let mut am = AtomMap2::new();
        let h1 = am.add_atom(Element::H, na::Point2::new(1.0, 2.0));
        assert_eq!(am.entities::<Atom>().count(), 1);
        assert_eq!(am.atom_positions.len(), 1);
        am.view_mut(h1).unwrap().delete();
        assert_eq!(am.entities::<Atom>().count(), 0);
        // Note that stale position is still present, but can't be accessed
        assert!(am.atom_positions.contains_key(h1.to_key()));
        assert!(am.view(h1).is_none());
    }
}

// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    bonding::BondType,
    elements::{Element, Pseudoelement},
    entities::*,
    error::Error,
    geometry::Point,
    units::Unit,
};

use super::{AtomicPositions, SpatialAtomMap};

/// Methods for entity addition.
impl<const D: usize, U: Unit> SpatialAtomMap<D, U> {
    /// Adds an atom to the map at the given position.
    pub fn add_atom(&mut self, element: Element, position: Point<D>) -> Atom {
        let atom = self.graph.add_atom(element);
        self.set_atom_position(atom, position);
        atom
    }

    /// Adds a pseudoatom to the map at the given position.
    pub fn add_pseudoatom(
        &mut self,
        pseudoelement: Pseudoelement,
        position: Point<D>,
    ) -> Pseudoatom {
        let pseudoatom = self.graph.add_pseudoatom(pseudoelement);
        self.set_pseudoatom_position(pseudoatom, position);
        pseudoatom
    }

    /// Creates a new (single covalent) bond between two bondable entities.
    ///
    /// # Errors
    ///
    /// Fails if either of `start` and `end` are invalid.
    pub fn add_bond<A, B>(&mut self, bond_type: BondType, start: A, end: B) -> Result<Bond, Error>
    where
        A: Bondable,
        B: Bondable,
    {
        if !self.contains(start) {
            return Err(Error::InvalidId(start.as_entity()));
        } else if !self.contains(end) {
            return Err(Error::InvalidId(end.as_entity()));
        };
        Ok(self.graph.add_bond_unchecked(bond_type, start, end))
    }

    /// Creates a new single covalent bond between two bondable entities.
    ///
    /// # Errors
    ///
    /// Fails if either of `start` and `end` are invalid.
    pub fn add_single_bond<A, B>(&mut self, start: A, end: B) -> Result<Bond, Error>
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
    pub fn add_double_bond<A, B>(&mut self, start: A, end: B) -> Result<Bond, Error>
    where
        A: Bondable,
        B: Bondable,
    {
        self.add_bond(BondType::Covalent { order: 2.0 }, start, end)
    }
}

#[cfg(test)]
mod tests {
    use crate::{geometry::Point2, traits::Map, units::Angstrom};

    use super::*;

    #[test]
    fn add_atom() {
        let mut am: SpatialAtomMap<2, Angstrom> = SpatialAtomMap::new();
        assert_eq!(am.iter::<Atom>().count(), 0);
        let h1 = am.add_atom(Element::H, Point2::new(1.0, 2.0));
        assert_eq!(am.iter::<Atom>().count(), 1);
        // Confirm that the atom positions also have data for a single atom
        assert_eq!(am.atom_positions.len(), 1);
        // Confirm that the atom position can be accessed
        assert_eq!(
            am.atom_positions.get(h1.to_key()).unwrap().clone(),
            Point2::new(1.0, 2.0)
        );
    }
}

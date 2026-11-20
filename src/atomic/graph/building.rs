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
    traits::{Map, Stores},
};

use super::*;

/// Methods for entity addition.
impl AtomMap {
    /// Adds an atom to the map.
    pub fn add_atom(&mut self, element: Element) -> Atom {
        self.atoms.insert(AtomData::new(element)).into()
    }

    /// Adds an atom to the map along with the requested number of hydrogen atoms
    /// (and single covalent bonds from the central atom to them).
    ///
    /// Returns the ID of the added central atom as well as a slice over the IDs
    /// of the new bonds.
    pub fn add_atom_with_hydrogen(&mut self, element: Element, n_hydrogen: u8) -> (Atom, &[Bond]) {
        let centre = self.add_atom(element);
        for _ in 0..n_hydrogen {
            let new_h = self.add_atom(Element::H);
            self.add_bond_unchecked(BondType::Covalent { order: 1.0 }, centre, new_h);
        }
        // Don't waste memory allocating a new Vec to hold the bond IDs, since
        // they are already stored on the new central atom – return a slice instead
        (
            centre,
            self.data(centre)
                .expect("We just created this atom, so ID must be valid")
                .bonds
                .as_slice(),
        )
    }

    /// Adds an atom to the map along with the number of additional hydrogen atoms
    /// required to satisfy the most common valency for the central atom (and single
    /// covalent bonds from the central atom to them).
    ///
    /// Returns the ID of the added central atom as well as a slice over the IDs
    /// of the new bonds.
    pub fn add_atom_and_saturate(&mut self, element: Element) -> (Atom, &[Bond]) {
        let n_hydrogen = element.default_valency();
        self.add_atom_with_hydrogen(element, n_hydrogen)
    }

    /// Adds a pseudoatom to the map.
    pub fn add_pseudoatom(&mut self, pseudoelement: Pseudoelement) -> Pseudoatom {
        self.pseudoatoms
            .insert(PseudoatomData::new(pseudoelement))
            .into()
    }

    /// Creates a new bond between two bondable entities without validating them first.
    ///
    /// # Panics
    ///
    /// Panics if either of `start` and `end` are invalid.
    pub fn add_bond_unchecked<A, B>(&mut self, bond_type: BondType, start: A, end: B) -> Bond
    where
        A: Bondable,
        B: Bondable,
    {
        let bond: Bond = self
            .bonds
            .insert(BondData::new(
                bond_type,
                start.as_bondable(),
                end.as_bondable(),
            ))
            .into();
        for partner in [start.as_bondable(), end.as_bondable()] {
            match partner.resolve() {
                ResolvedBondable::Atom(id) => {
                    self.atoms.get_mut(id.into()).unwrap().bonds.push(bond)
                }
                ResolvedBondable::Pseudoatom(id) => self
                    .pseudoatoms
                    .get_mut(id.into())
                    .unwrap()
                    .bonds
                    .push(bond),
            }
        }
        bond
    }

    /// Creates a new bond between two bondable entities.
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
        Ok(self.add_bond_unchecked(bond_type, start, end))
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
    use super::*;

    #[test]
    fn add_atom() {
        let mut g = AtomMap::new();
        assert!(g.atoms.is_empty());
        let h1 = g.add_atom(Element::H);
        assert_eq!(g.atoms.len(), 1);
        let c1 = g.add_atom(Element::C);
        assert_eq!(g.atoms.len(), 2);
        // Check the atoms can be accessed by their ID, and that the elements are correct
        assert_eq!(g.atoms.get(h1.into()).unwrap().element, Element::H);
        assert_eq!(g.atoms.get(c1.into()).unwrap().element, Element::C);
        // Check that the bond arrays are created empty
        assert!(g.atoms.get(h1.into()).unwrap().bonds.is_empty());
    }

    #[test]
    fn add_pseudoatom() {
        let mut g = AtomMap::new();
        assert!(g.pseudoatoms.is_empty());
        g.add_pseudoatom(Pseudoelement::Ph);
        assert_eq!(g.pseudoatoms.len(), 1);
    }

    #[test]
    fn add_bond_between_atoms() {
        let mut g = AtomMap::new();
        assert!(g.bonds.is_empty());
        let h1 = g.add_atom(Element::H);
        let h2 = g.add_atom(Element::H);
        let b1 = g.add_bond_unchecked(BondType::Covalent { order: 1.0 }, h1, h2);
        assert_eq!(g.bonds.len(), 1);
        assert!(g.contains(b1));
        assert!(g.atoms.get(h1.into()).unwrap().bonds.contains(&b1));
        assert!(g.atoms.get(h2.into()).unwrap().bonds.contains(&b1));
        assert_eq!(g.bonds.get(b1.into()).unwrap().start, h1.into());
        assert_eq!(g.bonds.get(b1.into()).unwrap().end, h2.into());
    }
}

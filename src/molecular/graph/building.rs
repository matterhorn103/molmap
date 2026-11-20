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
};

use super::super::data::*;
use super::MolMap;

/// Methods for entity addition.
#[allow(dead_code)]
impl MolMap {
    /// Adds a substituent to the given molecule with the given atomlike as its centre.
    ///
    /// Note that this method will not fail, even if `centre` is an invalid ID.
    fn add_substituent_with_centre(&mut self, centre: impl Atomlike) -> Substituent {
        self.substituents
            .insert(SubstituentData::with_centre(centre.as_atomlike()))
            .into()
    }

    /// Adds a molecule to the map with the substituent as its child and the
    /// substituent's members as its own members.
    ///
    /// # Panics
    ///
    /// Panics if the substituent does not exist.
    fn add_molecule_with_substituent(&mut self, substituent: Substituent) -> Molecule {
        let mut mol_data = MoleculeData::new();
        mol_data.substituents.insert(substituent);
        self.molecules.insert(MoleculeData::new()).into()
    }

    /// Adds a new molecule to the map containing a single atom.
    ///
    /// Returns the new atom, its parent substituent, and its parent molecule.
    pub(crate) fn add_atomic(&mut self, element: Element) -> (Atom, Substituent, Molecule) {
        // First add everything
        let atom = self.inner.add_atom(element);
        let sub = self.add_substituent_with_centre(atom);
        let mol = self.add_molecule_with_substituent(sub);
        // Using the above methods ensures that children are stored on the data structs
        // appropriately, but we still need to sort out the parents at the graph level
        self.atom_parents.insert(atom.into(), sub);
        self.substituent_parents.insert(sub.into(), mol);
        (atom, sub, mol)
    }

    /// Adds a new molecule to the map containing a single pseudoatom.
    ///
    /// Returns the new pseudoatom, its parent substituent, and its parent molecule.
    pub(crate) fn add_pseudoatomic(
        &mut self,
        pseudoelement: Pseudoelement,
    ) -> (Pseudoatom, Substituent, Molecule) {
        // First add everything
        let pseudoatom = self.inner.add_pseudoatom(pseudoelement);
        let sub = self.add_substituent_with_centre(pseudoatom);
        let mol = self.add_molecule_with_substituent(sub);
        // Using the above methods ensures that children are stored on the data structs
        // appropriately, but we still need to sort out the parents at the graph level
        self.pseudoatom_parents.insert(pseudoatom.into(), sub);
        self.substituent_parents.insert(sub.into(), mol);
        (pseudoatom, sub, mol)
    }

    /// Creates a new bond between two bondable entities without validating them first.
    ///
    /// # Panics
    ///
    /// Panics if either of `start` and `end` are invalid.
    #[expect(unused_variables)]
    pub fn add_bond_unchecked<A, B>(&mut self, bond_type: BondType, start: A, end: B) -> Bond
    where
        A: Bondable,
        B: Bondable,
    {
        // This requires implementation of breadth-first searches/connectedness checks
        todo!()
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

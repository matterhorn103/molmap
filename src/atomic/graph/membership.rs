// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    entities::*,
    traits::{Map, Stores, StoresMut},
    view::ViewMut,
};

use super::*;

/// Methods for entity removal.
impl AtomMap {
    /// Removes an atom from the map, as well as any bonds to it.
    ///
    /// Returns whether the atom was present in the map.
    ///
    /// This is infallible – if the atom is not in the map, nothing changes.
    pub(crate) fn delete_atom(&mut self, atom: Atom) -> bool {
        if !self.contains(atom) {
            return false;
        }
        // Make sure we always remove bonds first
        let bonds = self.data(atom).unwrap().bonds.clone();
        for bond in bonds {
            self.delete_bond(bond);
        }
        // Now we can safely remove the atom itself without leaving dangling bonds
        self.atoms.remove(atom.into()).is_some() // Should always be `true`
    }

    /// Removes a pseudoatom from the map, as well as any bonds to it.
    ///
    /// Returns whether the pseudoatom was present in the map.
    ///
    /// This is infallible – if the pseudoatom is not in the map, nothing changes.
    pub(crate) fn delete_pseudoatom(&mut self, pseudoatom: Pseudoatom) -> bool {
        if !self.contains(pseudoatom) {
            return false;
        }
        // Make sure we always remove bonds first
        let bonds = self.data(pseudoatom).unwrap().bonds.clone();
        for bond in bonds {
            self.delete_bond(bond);
        }
        // Now we can safely remove the pseudoatom itself without leaving dangling bonds
        self.pseudoatoms.remove(pseudoatom.into()).is_some()
    }

    /// Removes a bond from the map (but not its bonding partners).
    ///
    /// Returns whether the bond was present in the map.
    ///
    /// If the bond is not in the map, nothing changes.
    ///
    /// # Panics
    ///
    /// Panics if either of the bond's bonding partners does not exist (which
    /// should never be the case – bonds are last in, first out).
    pub(crate) fn delete_bond(&mut self, bond: Bond) -> bool {
        if let Some(bond_data) = self.bonds.remove(bond.into()) {
            for bonding_partner in [bond_data.start, bond_data.end] {
                match bonding_partner.resolve() {
                    ResolvedBondable::Atom(atom) => {
                        let atom_data = self
                            .data_mut(atom)
                            .expect("Bonds are always removed before their bonding partners");
                        let pos = atom_data.bonds.iter().position(|x| *x == bond).expect(
                            "Bond should be listed in the bonding partner's bonds until deletion",
                        );
                        atom_data.bonds.remove(pos);
                    }
                    ResolvedBondable::Pseudoatom(pseudoatom) => {
                        let pseudoatom_data = self
                            .data_mut(pseudoatom)
                            .expect("Bonds are always removed before their bonding partners");
                        let pos = pseudoatom_data.bonds.iter().position(|x| *x == bond).expect(
                                "Bond should be listed in the bonding partner's bonds until deletion",
                            );
                        pseudoatom_data.bonds.remove(pos);
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl<'m> ViewMut<'m, AtomMap, Atom> {
    /// Removes the atom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.delete_atom(self.id);
    }
}

impl<'m> ViewMut<'m, AtomMap, Pseudoatom> {
    /// Removes the pseudoatom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.delete_pseudoatom(self.id);
    }
}

impl<'m> ViewMut<'m, AtomMap, Bond> {
    /// Removes the bond from the map (but not its bonding partners).
    pub fn delete(self) {
        self.map.delete_bond(self.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bonding::BondType,
        elements::{Element, Pseudoelement},
    };

    use super::*;

    #[test]
    fn contains() {
        let mut g = AtomMap::new();
        let h1 = g.add_atom(Element::H);
        assert!(g.contains(h1));
        let ph = g.add_pseudoatom(Pseudoelement::Ph);
        assert!(g.contains(ph));
        g.delete_atom(h1);
        assert!(!g.contains(h1));
        assert!(g.contains(ph));
        g.delete_pseudoatom(ph);
        assert!(!g.contains(ph));
    }

    #[test]
    fn delete_atom() {
        let mut g = AtomMap::new();
        let h1 = g.add_atom(Element::H);
        let c1 = g.add_atom(Element::C);
        assert_eq!(g.atoms.len(), 2);
        assert!(g.delete_atom(h1));
        assert_eq!(g.atoms.len(), 1);
        // Make sure the correct one got deleted
        assert_eq!(g.atoms.keys().next().unwrap(), c1.to_key());
        // Trying to delete the same atom again shouldn't panic but should return false
        assert!(!g.delete_atom(h1));
    }

    #[test]
    fn delete_pseudoatom() {
        let mut g = AtomMap::new();
        let et = g.add_pseudoatom(Pseudoelement::Et);
        assert_eq!(g.pseudoatoms.len(), 1);
        g.delete_pseudoatom(et);
        assert!(g.pseudoatoms.is_empty());
    }

    #[test]
    fn delete_bond_between_atoms() {
        let mut g = AtomMap::new();
        let h1 = g.add_atom(Element::H);
        let h2 = g.add_atom(Element::H);
        let b1 = g.add_bond_unchecked(BondType::Covalent { order: 1.0 }, h1, h2);
        for h in [h1, h2] {
            assert!(g.atoms.get(h.into()).unwrap().bonds.contains(&b1));
        }
        assert_eq!(g.bonds.get(b1.into()).unwrap().start, h1.as_bondable());
        assert_eq!(g.bonds.get(b1.into()).unwrap().end, h2.as_bondable());
        // Now delete the bond and check the effects
        g.delete_bond(b1);
        // Bond should obviously be gone
        assert!(!g.contains(b1));
        // Atoms should remain, however
        for h in [h1, h2] {
            assert!(g.contains(h));
            // Neither atom should have any bonds now
            assert!(g.atoms.get(h.into()).unwrap().bonds.is_empty());
        }
    }

    #[test]
    fn deleting_bonding_partner_deletes_bond() {
        let mut g = AtomMap::new();
        let h1 = g.add_atom(Element::H);
        let h2 = g.add_atom(Element::H);
        let b1 = g.add_bond_unchecked(BondType::Covalent { order: 1.0 }, h1, h2);
        // Delete one of the bonding atoms and check the effects
        g.delete_atom(h1);
        // Atom should obviously be gone
        assert!(!g.contains(h1));
        // Bond should have been removed too
        assert!(!g.contains(b1));
        // The other atom should still remain, but without any bonds now
        assert!(g.contains(h2));
        assert!(g.atoms.get(h2.into()).unwrap().bonds.is_empty());
    }
}

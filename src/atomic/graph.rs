// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Definition of the data structure that holds the core graph of atoms and bonds.

use slotmap::SlotMap;

use super::entities::*;

use crate::{
    entities::*,
    traits::{Graph, Store},
};

// An arena-like data structure to represent a set of chemical entities,
// their properties, and the relationships between them, as a molecular graph.
//
// A `AtomGraph` forms the core of all `MolMap` types, but the type is not meant
// for external use. Its `pub` visibility is necessary to match [`MolMapCore`].
//
// [`MolMap0`] is the `MolMap` type that provides a molecular graph for users.
//
// In general, the methods of `AtomGraph` should be small in scope and efficient
// so that the higher maps can combine them to create a nice public API.
// The methods should do relatively little checking and validation, with the
// higher maps responsible for careful usage. In particular, all IDs should be
// assumed to be valid.

/// Docstring TODO
#[derive(Clone, Debug, Default)]
pub struct AtomGraph {
    pub(super) atoms: SlotMap<AtomKey, AtomData>,
    pub(super) pseudoatoms: SlotMap<PseudoatomKey, PseudoatomData>,
    pub(super) bonds: SlotMap<BondKey, BondData>,
}

macro_rules! impl_store {
    ($kind: ident, $attr: ident) => {
        impl Store<$kind> for AtomGraph {
            fn slotmap(&self) -> &SlotMap<<$kind as Keyed>::Key, <$kind as Stored>::Data> {
                &self.$attr
            }

            fn slotmap_mut(
                &mut self,
            ) -> &mut SlotMap<<$kind as Keyed>::Key, <$kind as Stored>::Data> {
                &mut self.$attr
            }
        }
    };
}
impl_store!(Atom, atoms);
impl_store!(Bond, bonds);
impl_store!(Pseudoatom, pseudoatoms);

//impl Graph for AtomGraph {
//    fn contains<E: Entity>(&self, entity: E) -> bool {
//        match entity.to_resolved() {
//            //ResolvedEntity::Atom(atom) => self.slotmap().contains_key(atom.to_key()),
//            //ResolvedEntity::Bond(bond) => self.slotmap().contains_key(bond.to_key()),
//            //ResolvedEntity::Pseudoatom(pseudoatom) => {
//            //    self.slotmap().contains_key(pseudoatom.to_key())
//            //}
//            _ => false,
//        }
//    }
//}

/// Constructor methods.
impl AtomGraph {
    /// Creates a new, empty `AtomGraph`.
    pub(crate) fn new() -> Self {
        Self {
            atoms: SlotMap::with_key(),
            pseudoatoms: SlotMap::with_key(),
            bonds: SlotMap::with_key(),
        }
    }

    /// Creates a new `AtomGraph` with the specified capacities for each kind of entity.
    pub(crate) fn with_capacities(atoms: usize, pseudoatoms: usize, bonds: usize) -> Self {
        Self {
            atoms: SlotMap::with_capacity_and_key(atoms),
            pseudoatoms: SlotMap::with_capacity_and_key(pseudoatoms),
            bonds: SlotMap::with_capacity_and_key(bonds),
        }
    }
}

///// Methods for entity addition.
//impl AtomGraph {
//    /// Adds an atom to the map.
//    pub(crate) fn add_atom(&mut self, element: Element) -> Atom {
//        self.atoms.insert(AtomData::new(element)).into()
//    }
//
//    /// Adds a pseudoatom to the map.
//    pub(crate) fn add_pseudoatom(&mut self, pseudoelement: Pseudoelement) -> Pseudoatom {
//        self.pseudoatoms
//            .insert(PseudoatomData::new(pseudoelement))
//            .into()
//    }
//
//    /// Creates a new bond between two bondable entities.
//    ///
//    /// # Panics
//    ///
//    /// Panics if either of `start` and `end` are invalid.
//    pub(crate) fn add_bond(
//        &mut self,
//        bond_type: BondType,
//        start: impl Bondable,
//        end: impl Bondable,
//    ) -> Bond {
//        let bond: Bond = self
//            .bonds
//            .insert(BondData::new(
//                bond_type,
//                start.as_bondable(),
//                end.as_bondable(),
//            ))
//            .into();
//        for partner in [start.as_bondable(), end.as_bondable()] {
//            match partner.resolve() {
//                ResolvedBondable::Atom(id) => {
//                    self.atoms.get_mut(id.into()).unwrap().bonds.push(bond)
//                }
//                ResolvedBondable::Pseudoatom(id) => self
//                    .pseudoatoms
//                    .get_mut(id.into())
//                    .unwrap()
//                    .bonds
//                    .push(bond),
//            }
//        }
//        bond
//    }
//}
//
///// Methods for entity removal.
//impl AtomGraph {
//    /// Removes an atom from the map, as well as any bonds to it.
//    ///
//    /// Returns whether the atom was present in the map.
//    ///
//    /// This is infallible – if the atom is not in the map, nothing changes.
//    pub(crate) fn delete_atom(&mut self, atom: Atom) -> bool {
//        if !self.contains(atom) {
//            return false;
//        }
//        // Make sure we always remove bonds first
//        let bonds = self.data(atom).unwrap().bonds.clone();
//        for bond in bonds {
//            self.delete_bond(bond);
//        }
//        // Now we can safely remove the atom itself without leaving dangling bonds
//        self.atoms.remove(atom.into()).is_some() // Should always be `true`
//    }
//
//    /// Removes a pseudoatom from the map, as well as any bonds to it.
//    ///
//    /// Returns whether the pseudoatom was present in the map.
//    ///
//    /// This is infallible – if the pseudoatom is not in the map, nothing changes.
//    pub(crate) fn delete_pseudoatom(&mut self, pseudoatom: Pseudoatom) -> bool {
//        if !self.contains(pseudoatom) {
//            return false;
//        }
//        // Make sure we always remove bonds first
//        let bonds = self.data(pseudoatom).unwrap().bonds.clone();
//        for bond in bonds {
//            self.delete_bond(bond);
//        }
//        // Now we can safely remove the pseudoatom itself without leaving dangling bonds
//        self.pseudoatoms.remove(pseudoatom.into()).is_some()
//    }
//
//    /// Removes a bond from the map (but not its bonding partners).
//    ///
//    /// Returns whether the bond was present in the map.
//    ///
//    /// If the bond is not in the map, nothing changes.
//    ///
//    /// # Panics
//    ///
//    /// Panics if either of the bond's bonding partners does not exist (which
//    /// should never be the case – bonds are last in, first out).
//    pub(crate) fn delete_bond(&mut self, bond: Bond) -> bool {
//        if let Some(bond_data) = self.bonds.remove(bond.into()) {
//            for bonding_partner in [bond_data.start, bond_data.end] {
//                match bonding_partner.resolve() {
//                    ResolvedBondable::Atom(atom) => {
//                        let atom_data = self
//                            .data_mut(atom)
//                            .expect("Bonds are always removed before their bonding partners");
//                        let pos = atom_data.bonds.iter().position(|x| *x == bond).expect(
//                            "Bond should be listed in the bonding partner's bonds until deletion",
//                        );
//                        atom_data.bonds.remove(pos);
//                    }
//                    ResolvedBondable::Pseudoatom(pseudoatom) => {
//                        let pseudoatom_data = self
//                            .data_mut(pseudoatom)
//                            .expect("Bonds are always removed before their bonding partners");
//                        let pos = pseudoatom_data.bonds.iter().position(|x| *x == bond).expect(
//                            "Bond should be listed in the bonding partner's bonds until deletion",
//                        );
//                        pseudoatom_data.bonds.remove(pos);
//                    }
//                }
//            }
//            true
//        } else {
//            false
//        }
//    }
//}

// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Definition of the data structure that holds the core graph of atoms and bonds.

use slotmap::SlotMap;

use crate::{
    entities::*,
    traits::{Graph, Map, Stores, StoresMut},
};

use super::*;

/// A map holding only fundamental atomic entities, and a network of bonds between them.
///
/// This is the basic graph at the core of all other [`Map`] types.
///
/// The following invariants are promised to hold:
///
/// 1. Bonds only exist as long as both bonding partners continue to exist.
/// 2. The two bonding partners of a bond are always different.
/// 3. Only one `Bond` entity can connect a given pair of bonding partners
///    (though that bond may be a double bond/triple bond/etc.).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomMap {
    pub(super) atoms: SlotMap<AtomKey, AtomData>,
    pub(super) pseudoatoms: SlotMap<PseudoatomKey, PseudoatomData>,
    pub(super) bonds: SlotMap<BondKey, BondData>,
}

/// Constructor methods.
impl AtomMap {
    /// Creates a new, empty `AtomGraph`.
    pub fn new() -> Self {
        Self {
            atoms: SlotMap::with_key(),
            pseudoatoms: SlotMap::with_key(),
            bonds: SlotMap::with_key(),
        }
    }

    /// Creates a new `AtomGraph` with the specified capacities for each kind of entity.
    pub fn with_capacities(atoms: usize, pseudoatoms: usize, bonds: usize) -> Self {
        Self {
            atoms: SlotMap::with_capacity_and_key(atoms),
            pseudoatoms: SlotMap::with_capacity_and_key(pseudoatoms),
            bonds: SlotMap::with_capacity_and_key(bonds),
        }
    }
}

impl Map for AtomMap {
    type Graph = Self;

    fn into_graph(self) -> Self::Graph {
        self
    }

    fn as_graph(&self) -> &Self {
        self
    }

    fn contains<E: Entity>(&self, entity: E) -> bool {
        match entity.to_resolved() {
            ResolvedEntity::Atom(atom) => {
                <AtomMap as Stores<Atom>>::slotmap(self).contains_key(atom.to_key())
            }
            ResolvedEntity::Bond(bond) => {
                <AtomMap as Stores<Bond>>::slotmap(self).contains_key(bond.to_key())
            }
            ResolvedEntity::Pseudoatom(pseudoatom) => {
                <AtomMap as Stores<Pseudoatom>>::slotmap(self).contains_key(pseudoatom.to_key())
            }
            _ => false,
        }
    }
}

impl Graph for AtomMap {}

impl Atomic for AtomMap {
    fn into_atomic(self) -> AtomMap {
        self
    }

    fn as_atomic(&self) -> &AtomMap {
        self
    }
}

macro_rules! impl_stores {
    ($kind: ident, $attr: ident) => {
        impl Stores<$kind> for AtomMap {
            fn slotmap(&self) -> &SlotMap<<$kind as Keyed>::Key, <$kind as Stored>::Data> {
                &self.$attr
            }
        }

        impl StoresMut<$kind> for AtomMap {
            fn slotmap_mut(
                &mut self,
            ) -> &mut SlotMap<<$kind as Keyed>::Key, <$kind as Stored>::Data> {
                &mut self.$attr
            }
        }
    };
}
impl_stores!(Atom, atoms);
impl_stores!(Bond, bonds);
impl_stores!(Pseudoatom, pseudoatoms);

#[cfg(test)]
mod tests {
    use crate::elements::{Element, Pseudoelement};

    use super::*;

    #[test]
    fn slotmap() {
        let g = AtomMap::new();
        // Make sure that the generic slotmap method returns a reference to the appropriate slotmap
        assert!(std::ptr::eq(
            <AtomMap as Stores<Atom>>::slotmap(&g),
            &g.atoms
        ));
        assert!(std::ptr::eq(
            <AtomMap as Stores<Bond>>::slotmap(&g),
            &g.bonds
        ));
        assert!(std::ptr::eq(
            <AtomMap as Stores<Pseudoatom>>::slotmap(&g),
            &g.pseudoatoms
        ));
    }

    #[test]
    fn data() {
        let mut g = AtomMap::new();
        let h1 = g.add_atom(Element::H);
        let ph = g.add_pseudoatom(Pseudoelement::Ph);
        // Make sure that the generic data method returns a reference to the appropriate entity data struct
        let h1_data = g.data(h1).unwrap();
        assert!(std::ptr::eq(h1_data, g.atoms.get(h1.into()).unwrap()));
        assert_eq!(h1_data.element, Element::H);
        assert_eq!(h1_data.bonds, &[]);
        assert!(std::ptr::eq(
            g.data(ph).unwrap(),
            g.pseudoatoms.get(ph.into()).unwrap()
        ));
    }
}

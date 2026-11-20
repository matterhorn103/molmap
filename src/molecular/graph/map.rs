// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use slotmap::{SecondaryMap, SlotMap};

use crate::{
    atomic::{AtomMap, Atomic},
    entities::*,
    traits::{Graph, Map, Stores, StoresMut},
};

use super::super::{Molecular, data::*};

/// A map in which the atomic entities and bonds are organized into substituents
/// and molecules.
///
/// The following invariants are promised to hold (in addition to those promised
/// by [`AtomMap`]):
///
/// 1. No collection is ever empty.
///     - Collections are only ever created using methods that add at least one
///       initial member (this would imply that "splitting off" methods are
///       necessary to move fundamentals into a different, new collection).
///     - Collections are cleaned up automatically when emptied.
/// 2. All atomlikes are members of exactly one substituent.
/// 3. All substituents are members of exactly one molecule.
/// 4. Any atomlikes connected by a covalent (or dipolar) bond are members of the
///    same molecule.
///     - Joining two atomlikes that are *not* members of the same molecule causes
///       the parent molecules to be merged.
/// 5. All atomlikes of a substituent are connected by strong bonds.
///     - Bond deletion errors if it would cause fragmentation of a substituent.
/// 6. All substituents (and therefore atomlikes) in a molecule are connected by
///    strong bonds.
///     - If bond deletion causes a molecule to fragment into two disconnected
///       parts, the fragments become separate molecules.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
pub struct MolMap {
    pub(super) inner: AtomMap,
    pub(super) substituents: SlotMap<SubstituentKey, SubstituentData>,
    pub(super) molecules: SlotMap<MoleculeKey, MoleculeData>,
    pub(super) atom_parents: SecondaryMap<AtomKey, Substituent>,
    pub(super) pseudoatom_parents: SecondaryMap<PseudoatomKey, Substituent>,
    pub(super) bond_parents: SecondaryMap<BondKey, Option<AnyCollection>>,
    pub(super) substituent_parents: SecondaryMap<SubstituentKey, Molecule>,
}

/// Constructor methods.
impl MolMap {
    /// Creates a new, empty `MolMap`.
    pub(crate) fn new() -> Self {
        Self {
            inner: AtomMap::new(),
            substituents: SlotMap::with_key(),
            molecules: SlotMap::with_key(),
            atom_parents: SecondaryMap::new(),
            pseudoatom_parents: SecondaryMap::new(),
            bond_parents: SecondaryMap::new(),
            substituent_parents: SecondaryMap::new(),
        }
    }

    /// Creates a new `MolMap` with the specified capacities for each kind of entity.
    #[expect(dead_code)]
    pub(crate) fn with_capacities(
        atoms: usize,
        pseudoatoms: usize,
        bonds: usize,
        substituents: usize,
        molecules: usize,
    ) -> Self {
        Self {
            inner: AtomMap::with_capacities(atoms, pseudoatoms, bonds),
            substituents: SlotMap::with_capacity_and_key(substituents),
            molecules: SlotMap::with_capacity_and_key(molecules),
            atom_parents: SecondaryMap::with_capacity(atoms),
            pseudoatom_parents: SecondaryMap::with_capacity(pseudoatoms),
            bond_parents: SecondaryMap::with_capacity(bonds),
            substituent_parents: SecondaryMap::with_capacity(substituents),
        }
    }
}

impl Map for MolMap {
    type Graph = Self;

    fn into_graph(self) -> Self::Graph {
        self
    }

    fn as_graph(&self) -> &Self {
        self
    }

    fn contains<E: Entity>(&self, entity: E) -> bool {
        self.contains(entity)
    }
}

impl Graph for MolMap {}

impl Atomic for MolMap {
    fn into_atomic(self) -> AtomMap {
        self.inner
    }

    fn as_atomic(&self) -> &AtomMap {
        &self.inner
    }
}

impl Molecular for MolMap {
    fn into_molecular(self) -> MolMap {
        self
    }

    fn as_molecular(&self) -> &MolMap {
        self
    }
}

impl<E> Stores<E> for MolMap
where
    E: Kind,
    AtomMap: Stores<E>,
{
    fn slotmap(&self) -> &SlotMap<<E>::Key, <E>::Data> {
        self.inner.slotmap()
    }
}

impl<E> StoresMut<E> for MolMap
where
    E: Kind,
    AtomMap: StoresMut<E>,
{
    fn slotmap_mut(&mut self) -> &mut SlotMap<<E>::Key, <E>::Data> {
        self.inner.slotmap_mut()
    }
}

macro_rules! impl_stores {
    ($kind: ident, $attr: ident) => {
        impl Stores<$kind> for MolMap {
            fn slotmap(&self) -> &SlotMap<<$kind as Keyed>::Key, <$kind as Stored>::Data> {
                &self.$attr
            }
        }

        impl StoresMut<$kind> for MolMap {
            fn slotmap_mut(
                &mut self,
            ) -> &mut SlotMap<<$kind as Keyed>::Key, <$kind as Stored>::Data> {
                &mut self.$attr
            }
        }
    };
}
impl_stores!(Substituent, substituents);
impl_stores!(Molecule, molecules);

// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// The entity ID types are defined in the corresponding entity modules; we
// re-export them publicly here

pub use crate::entities::atom::AtomId;
pub use crate::entities::bond::BondId;
pub use crate::entities::molecule::MoleculeId;
pub use crate::entities::pseudoatom::PseudoatomId;
pub use crate::entities::substituent::SubstituentId;

// We use composite enums of IDs, not traits, to classify entities and narrow
// functionality

/// An ID of an atom or something that behaves like one (a pseudoatom).
///
/// Atomlikes are the true nodes of the molecular graph.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Atomlike {
    Atom(AtomId),
    Pseudoatom(PseudoatomId),
}

impl From<AtomId> for Atomlike {
    fn from(id: AtomId) -> Self {
        Atomlike::Atom(id)
    }
}

impl From<PseudoatomId> for Atomlike {
    fn from(id: PseudoatomId) -> Self {
        Atomlike::Pseudoatom(id)
    }
}

/// An ID of a fundamental entity, an entity that does not group other entities.
///
/// Fundamentals are the basic building blocks of a `MolMap`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Fundamental {
    Atom(AtomId),
    Pseudoatom(PseudoatomId),
    Bond(BondId),
}

impl From<AtomId> for Fundamental {
    fn from(id: AtomId) -> Self {
        Fundamental::Atom(id)
    }
}

impl From<PseudoatomId> for Fundamental {
    fn from(id: PseudoatomId) -> Self {
        Fundamental::Pseudoatom(id)
    }
}

impl From<BondId> for Fundamental {
    fn from(id: BondId) -> Self {
        Fundamental::Bond(id)
    }
}

impl From<Atomlike> for Fundamental {
    fn from(atomlike: Atomlike) -> Self {
        match atomlike {
            Atomlike::Atom(id) => Fundamental::Atom(id),
            Atomlike::Pseudoatom(id) => Fundamental::Pseudoatom(id),
        }
    }
}

/// An ID of a collection, an aggregation of fundamental entities.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Collection {
    Substituent(SubstituentId),
    Molecule(MoleculeId),
}

impl From<SubstituentId> for Collection {
    fn from(id: SubstituentId) -> Self {
        Collection::Substituent(id)
    }
}

impl From<MoleculeId> for Collection {
    fn from(id: MoleculeId) -> Self {
        Collection::Molecule(id)
    }
}

/// An ID of an entity that can form bonds.
///
/// The actual entities that bonds connect are represented by [`crate::entities::bond::BondingPartner`],
/// which is more restrictive.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Bondable {
    Atom(AtomId),
    Pseudoatom(PseudoatomId),
    //Bond(BondId),
    Substituent(SubstituentId),
}

impl From<AtomId> for Bondable {
    fn from(id: AtomId) -> Self {
        Bondable::Atom(id)
    }
}

impl From<PseudoatomId> for Bondable {
    fn from(id: PseudoatomId) -> Self {
        Bondable::Pseudoatom(id)
    }
}

impl From<SubstituentId> for Bondable {
    fn from(id: SubstituentId) -> Self {
        Bondable::Substituent(id)
    }
}

impl From<Atomlike> for Bondable {
    fn from(atomlike: Atomlike) -> Self {
        match atomlike {
            Atomlike::Atom(id) => Bondable::Atom(id),
            Atomlike::Pseudoatom(id) => Bondable::Pseudoatom(id),
        }
    }
}

/// An ID of an entity that an `Object` can be attached to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Anchor {
    Atom(AtomId),
    Pseudoatom(PseudoatomId),
    Bond(BondId),
    Substituent(SubstituentId),
    Molecule(MoleculeId),
}

impl From<AtomId> for Anchor {
    fn from(id: AtomId) -> Self {
        Anchor::Atom(id)
    }
}

impl From<PseudoatomId> for Anchor {
    fn from(id: PseudoatomId) -> Self {
        Anchor::Pseudoatom(id)
    }
}

impl From<BondId> for Anchor {
    fn from(id: BondId) -> Self {
        Anchor::Bond(id)
    }
}

impl From<SubstituentId> for Anchor {
    fn from(id: SubstituentId) -> Self {
        Anchor::Substituent(id)
    }
}

impl From<MoleculeId> for Anchor {
    fn from(id: MoleculeId) -> Self {
        Anchor::Molecule(id)
    }
}

/// An ID of any kind of entity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Entity {
    Atom(AtomId),
    Pseudoatom(PseudoatomId),
    Bond(BondId),
    Substituent(SubstituentId),
    Molecule(MoleculeId),
}

impl From<AtomId> for Entity {
    fn from(id: AtomId) -> Self {
        Entity::Atom(id)
    }
}

impl From<PseudoatomId> for Entity {
    fn from(id: PseudoatomId) -> Self {
        Entity::Pseudoatom(id)
    }
}

impl From<BondId> for Entity {
    fn from(id: BondId) -> Self {
        Entity::Bond(id)
    }
}

impl From<SubstituentId> for Entity {
    fn from(id: SubstituentId) -> Self {
        Entity::Substituent(id)
    }
}

impl From<MoleculeId> for Entity {
    fn from(id: MoleculeId) -> Self {
        Entity::Molecule(id)
    }
}

impl From<Atomlike> for Entity {
    fn from(atomlike: Atomlike) -> Self {
        match atomlike {
            Atomlike::Atom(id) => Entity::Atom(id),
            Atomlike::Pseudoatom(id) => Entity::Pseudoatom(id),
        }
    }
}

impl From<Fundamental> for Entity {
    fn from(fundamental: Fundamental) -> Self {
        match fundamental {
            Fundamental::Atom(id) => Entity::Atom(id),
            Fundamental::Pseudoatom(id) => Entity::Pseudoatom(id),
            Fundamental::Bond(id) => Entity::Bond(id),
        }
    }
}

impl From<Collection> for Entity {
    fn from(collection: Collection) -> Self {
        match collection {
            Collection::Substituent(id) => Entity::Substituent(id),
            Collection::Molecule(id) => Entity::Molecule(id),
        }
    }
}

impl From<Bondable> for Entity {
    fn from(bondable: Bondable) -> Self {
        match bondable {
            Bondable::Atom(id) => Entity::Atom(id),
            Bondable::Pseudoatom(id) => Entity::Pseudoatom(id),
            //Bondable::Bond(id) => Entity::Bond(id),
            Bondable::Substituent(id) => Entity::Substituent(id),
        }
    }
}

//impl From<BondingPartner> for Entity {
//    fn from(partner: BondingPartner) -> Self {
//        match partner {
//            BondingPartner::Atom(id) => Entity::Atom(id),
//            BondingPartner::Pseudoatom(id) => Entity::Pseudoatom(id),
//            BondingPartner::AmbiguouslyBondingSubstituent(id) => Entity::Substituent(id),
//        }
//    }
//}

impl From<Anchor> for Entity {
    fn from(anchor: Anchor) -> Self {
        match anchor {
            Anchor::Atom(id) => Entity::Atom(id),
            Anchor::Pseudoatom(id) => Entity::Pseudoatom(id),
            Anchor::Bond(id) => Entity::Bond(id),
            Anchor::Substituent(id) => Entity::Substituent(id),
            Anchor::Molecule(id) => Entity::Molecule(id),
        }
    }
}

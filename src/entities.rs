// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// The entities module contains a submodule for each entity kind.
// Each entity's submodule defines all types relating to the entity in one place:
// it has the implementation of the entity struct itself, the implementations of
// the corresponding View and ViewMut types, the corresponding ID type, and any
// other supporting functions and types specific to that entity.
// This is done for ease of development.
//
// However, the contents of this module are not exposed as part of the public
// API in the same way, and neither this module nor its submodules are public.
// The view types are all made available under the `views` module, and the ID
// types under the `ids` module (which also defines composite IDs).

pub(crate) mod atom;
pub(crate) mod bond;
pub(crate) mod definition;
pub(crate) mod molecule;
pub(crate) mod pseudoatom;
pub(crate) mod substituent;

// Re-export just the entities themselves for easy glob import within crate
pub(crate) use atom::Atom;
pub(crate) use bond::Bond;
pub(crate) use molecule::Molecule;
pub(crate) use pseudoatom::Pseudoatom;
pub(crate) use substituent::Substituent;

use crate::{MolMapError, MolMapResult};

/// The kind of an entity.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
//#[non_exhaustive]
pub enum EntityKind {
    Atom = 0x00,
    Pseudoatom = 0x01,
    Bond = 0x02,
    Substituent = 0x10,
    Molecule = 0x1F,
}

impl From<EntityKind> for u8 {
    fn from(kind: EntityKind) -> Self {
        kind as u8
    }
}

impl TryFrom<u8> for EntityKind {
    type Error = MolMapError;

    fn try_from(value: u8) -> MolMapResult<Self> {
        match value {
            0x00 => Ok(Self::Atom),
            0x01 => Ok(Self::Pseudoatom),
            0x02 => Ok(Self::Bond),
            0x10 => Ok(Self::Substituent),
            0x1F => Ok(Self::Molecule),
            _ => Err(MolMapError::UnknownEntityKind(value)),
        }
    }
}

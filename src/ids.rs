// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::num::{IntErrorKind, NonZeroU16};

use slotmap::Key;

use crate::entities::EntityKind;
use crate::{MolMapError, MolMapResult};

// We use composite IDs, not traits, to classify entities and narrow
// functionality. The strategy originally pursued was to wrap the basic ID types
// in enums, but this makes the composite ID types 12 bytes vs 8, and as they
// are widely used, that just means a lot of unnecessary memory use.
//
// Instead, we convert the SlotMap keys to the raw `u64` using the `as_ffi`
// method and use the most significant 8 bits to store a discriminant, and have
// the whole thing be the ID.
//
// This means that the maximum attainable version of the underlying SlotMap keys
// before issues arise is reduced from ~2^31 to ~2^23, as versions above that
// will not survive a round trip. However, this still allows over 8 million
// deletion-addition cycles before overflow, which should be plenty for chemical
// applications. Taking the bits from the index field would reduce the maximum
// possible number of atoms, which is much more likely to be a limiting factor.

/// The ID of any kind of entity.
///
/// This is equivalent to [`SlotMap::KeyData`] but with the version limited to
/// 24 bits.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct EntityId {
    pub(crate) kind: EntityKind,
    version_high: u8,
    version_low: u16,
    idx: u32,
}

impl EntityId {
    /// Wraps a key to create an ID.
    ///
    /// # Panics
    ///
    /// When the `overflow-checks` compiler setting is active (e.g. with the `dev`
    /// profile, but not `release`), panic occurs when the key version has overflowed.
    ///
    /// Normally, overflow occurs for `SlotMap` [after 2^31 deletions and insertions](https://docs.rs/slotmap/latest/slotmap/#performance-characteristics-and-implementation-details)
    /// to the same slot. For `molmap`'s ID types this is considerably lower: 2^23.
    /// At over 8 million cycles, this is still comfortably high for chemical
    /// applications.
    pub(crate) fn from_key<K: slotmap::Key>(kind: EntityKind, key: K) -> Self {
        // Runtime validation check should only take place if overflow checks are
        // enabled, otherwise prioritize performance and just drop the byte, which
        // invalidates the ID/key by causing it to hold a stale version number
        // However, `cfg(overflow_checks)` is currently unstable
        //if cfg!(overflow_checks) {
        //    assert_eq!(
        //        ffi & (0xFF << 56),
        //        0,
        //        "Key version overflow – maximum deletion/insertion cycles exceeded!"
        //    )
        //} else {
        if cfg!(debug_assertions) {
            let ffi = key.data().as_ffi();
            debug_assert_eq!(
                ffi & (0xFF << 56),
                0,
                "Key version overflow – maximum deletion/insertion cycles exceeded!"
            );
        };
        let ffi = key.data().as_ffi();
        EntityId {
            kind,
            version_high: (ffi >> 48) as u8,
            version_low: (ffi >> 32) as u16,
            idx: ffi as u32,
        }
    }

    /// Returns the equivalent key without confirming that the key is for the correct
    /// kind of entity.
    pub(crate) fn to_key_unchecked<K: slotmap::Key>(self) -> K {
        let ffi =
            (self.version_high as u64) << 48 | (self.version_low as u64) << 32 | self.idx as u64;
        K::from(slotmap::KeyData::from_ffi(ffi))
    }
}

/// An ID for a single kind of entity or a set of kinds of entity.
///
/// Implementors are required to ensure that they convert to the correct variant of
/// [`EntityId`], and that only the correct variants convert to the `Id` type.
pub trait Id: Into<EntityId> + TryFrom<EntityId> {
    /// The tagged key form of the ID that wraps an actual key in an enum.
    ///
    /// The tagged key form is inherently larger than the ID form but allows convenient
    /// matching on the kind while holding the underlying key.
    type Tagged;

    /// Returns the tagged key form of the ID that wraps the actual key in an enum.
    ///
    /// The tagged key form is inherently larger than the ID form but allows convenient
    /// matching on the kind while holding the underlying key.
    fn to_tagged(self) -> Self::Tagged;

    /// Returns the kind of entity that the ID represents, amongst all possible
    /// kinds of entity.
    fn kind(&self) -> EntityKind;
}

/// A key ID as a tagged enum.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(u8)]
//#[non_exhaustive]
pub enum TaggedEntity {
    Atom(AtomId) = 0x00,
    Pseudoatom(PseudoatomId) = 0x01,
    Bond(BondId) = 0x02,
    Substituent(SubstituentId) = 0x10,
    Molecule(MoleculeId) = 0x1F,
}

impl Id for EntityId {
    type Tagged = TaggedEntity;

    fn kind(&self) -> EntityKind {
        self.kind
    }

    fn to_tagged(self) -> TaggedEntity {
        match self.kind {
            EntityKind::Atom => TaggedEntity::Atom(self.to_key_unchecked()),
            EntityKind::Pseudoatom => TaggedEntity::Pseudoatom(self.to_key_unchecked()),
            EntityKind::Bond => TaggedEntity::Bond(self.to_key_unchecked()),
            EntityKind::Substituent => TaggedEntity::Substituent(self.to_key_unchecked()),
            EntityKind::Molecule => TaggedEntity::Molecule(self.to_key_unchecked()),
        }
    }
}

/// An ID for a kind of entity that is also a key for the corresponding `SlotMap`.
///
/// While category ID types wrap an [`EntityId`] with the discriminant already
/// included, key ID types are simply the actual [`SlotMap::Key`], as lookup using
/// the key is the most common use of the ID and so having to convert every time
/// would be inefficient.
///
/// `From`, `TryFrom` and `Id` must be implemented manually for each `KeyId` (but as
/// they are defined using a macro, this is not a big deal).
pub(crate) trait KeyId: slotmap::Key {
    /// The kind of entity that the ID represents.
    const KIND: EntityKind;
}

// First we define the different key IDs, which are all just SlotMap keys.

/// Defines a key ID type: a [`slotmap::Key`] that implements [`Id`] and [`KeyId`].
///
/// There must already exist:
/// - A matching `EntityKind::$name` variant
///
/// As well as the key ID type itself, named `$nameId`, an enum named
/// `$nameKind` gets defined with a single variant, `$name`.
///
/// The following trait implementations are then defined:
/// - `KeyId for $nameId`
/// - `Id for $nameId`
/// - `From<$nameId> for EntityId` and `TryFrom<EntityId> for $nameId`
///
/// Requires ident concatenation, currently done using the `paste` crate (which,
/// though now marked as unmaintained, is at least stable, mature, and popular).
macro_rules! define_key_id {
    (
        $(#[$doc:meta])*
        $name:ident;
    ) => {
        paste::paste! {
            slotmap::new_key_type! {
                $(#[$doc])*
                pub struct [<$name Id>];
            }

            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            #[repr(u8)]
            pub enum [<Tagged $name>] {
                $name = EntityKind::$name as u8,
            }

            impl KeyId for [<$name Id>] {
                const KIND: EntityKind = EntityKind::$name;
            }

            impl Id for [<$name Id>] {
                type Tagged = [<Tagged $name>];

                fn kind(&self) -> EntityKind {
                    EntityKind::$name
                }

                fn to_tagged(self) -> [<Tagged $name>] {
                    [<Tagged $name>]::$name
                }
            }

            impl From<[<$name Id>]> for EntityId {
                fn from(key: [<$name Id>]) -> EntityId {
                    EntityId::from_key(<[<$name Id>] as KeyId>::KIND, key)
                }
            }

            impl TryFrom<EntityId> for [<$name Id>] {
                type Error = MolMapError;

                fn try_from(id: EntityId) -> MolMapResult<Self> {
                    match id.kind {
                        EntityKind::$name => Ok(id.to_key_unchecked()),
                        _ => Err(MolMapError::IncorrectEntityKind(id.kind, id)),
                    }
                }
            }
        }
    };
}

define_key_id! {
    /// An ID corresponding to a specific atom entity in a `MolMap`.
    Atom;
}

define_key_id! {
    /// An ID corresponding to a specific pseudoatom entity in a `MolMap`.
    Pseudoatom;
}

define_key_id! {
    /// An ID corresponding to a specific bond entity in a `MolMap`.
    Bond;
}

define_key_id! {
    /// An ID corresponding to a specific substituent entity in a `MolMap`.
    Substituent;
}

define_key_id! {
    /// An ID corresponding to a specific molecule entity in a `MolMap`.
    Molecule;
}

// Now we define the different category IDs, which all just wrap an `EntityId`.
// It is important to note that any key ID can be converted into an `EntityId`
// trivially, and that any resulting `EntityId` can be wrapped to create any
// category ID type without an error occurring.
// Which kinds of entities can be converted to what is strictly controlled by
// the `From` and `TryFrom` implementations, so these should be implemented with
// thought and care.

/// Defines a category ID type wrapping [`EntityId`], covering the specified subset
/// of entity kinds.
///
/// For each `$variant` given, there must already exist:
/// - A matching `EntityKind::$variant` variant
/// - A key ID type named `$variantId` (e.g. `AtomId`) that is
///   `Into<EntityId>`
///
/// As well as the category ID type itself, named `$nameId`, an enum named
/// `$nameKind` gets defined with the provided variants and the same discriminant
/// values as for [`EntityKind`].
///
/// The following trait implementations are then defined:
/// - `Id for $nameId`
/// - `From<$nameId> for EntityId` and `TryFrom<EntityId> for $nameId`
/// - `From<$variantId> for $nameId` and `TryFrom<$nameId> for $variantId` for each `$variant`
///
/// Requires ident concatenation, currently done using the `paste` crate (which,
/// though now marked as unmaintained, is at least stable, mature, and popular).
///
/// # Usage
///
/// ```
/// define_category_id! {
///     /// An ID of a category.
///     Category {
///         Atom,
///         Pseudoatom,
///         Bond,
///     }
/// }
/// ```
///
/// will define:
///
/// ```
/// /// An ID of a category.
/// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// pub struct CategoryId(EntityId);
///
/// /// The kinds of entity covered by [`CategoryId`].
/// #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// pub enum CategoryKind {
///     Atom = EntityId::Atom as u8,
///     Pseudoatom = EntityId::Pseudoatom as u8,
///     Bond = EntityId::Bond as u8,
/// }
/// ```
macro_rules! define_category_id {
    (
        $(#[$doc:meta])*
        $name:ident {
            $($variant:ident),+ $(,)?
        }
    ) => {
        #[allow(non_snake_case)]
        paste::paste! {
            $(#[$doc])*
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            pub struct [<$name Id>](EntityId);

            #[doc = concat!(
                "The kinds of entity covered by [`", stringify!([<$name Id>]), "`]."
            )]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
            #[repr(u8)]
            pub enum [<Tagged $name>] {
                $($variant([<$variant Id>]) = EntityKind::$variant as u8,)+
            }

            impl Id for [<$name Id>] {
                type Tagged = [<Tagged $name>];

                fn kind(&self) -> EntityKind {
                    self.0.kind
                }

                fn to_tagged(self) -> [<Tagged $name>] {
                    match self.0.kind {
                        $(EntityKind::$variant => [<Tagged $name>]::$variant(self.0.to_key_unchecked()),)+
                        _ => unreachable!()
                    }
                }
            }

            impl From<[<$name Id>]> for EntityId {
                fn from(id: [<$name Id>]) -> EntityId {
                    id.0
                }
            }

            impl TryFrom<EntityId> for [<$name Id>] {
                type Error = MolMapError;

                fn try_from(id: EntityId) -> MolMapResult<Self> {
                    match id.kind {
                        $(EntityKind::$variant => Ok(Self(id)),)+
                        _ => Err(MolMapError::IncorrectEntityKind(id.kind, id)),
                    }
                }
            }

            $(
                impl From<[<$variant Id>]> for [<$name Id>] {
                    fn from(id: [<$variant Id>]) -> Self {
                        Self(id.into())
                    }
                }

                impl TryFrom<[<$name Id>]> for [<$variant Id>] {
                    type Error = MolMapError;

                    fn try_from(id: [<$name Id>]) -> MolMapResult<[<$variant Id>]> {
                        match id.0.kind {
                            EntityKind::$variant => Ok(id.0.to_key_unchecked()),
                            _ => Err(MolMapError::IncorrectEntityKind(id.0.kind, id.0)),
                        }
                    }
                }
            )+
        }
    };
}

define_category_id! {
    /// An ID of an atom or something that behaves like one (a pseudoatom).
    ///
    /// Atomlikes are the true nodes of the molecular graph.
    Atomlike {
        Atom,
        Pseudoatom,
    }
}

define_category_id! {
    /// An ID of a fundamental entity, an entity that does not group other entities.
    ///
    /// Fundamentals are the basic building blocks of a [`MolMap`].
    ///
    /// Atoms, pseudoatoms, and bonds are fundamentals.
    Fundamental {
        Atom,
        Pseudoatom,
        Bond,
    }
}

define_category_id! {
    /// An ID of a collection, an aggregation of fundamental entities.
    Collection {
        Substituent,
        Molecule,
    }
}

define_category_id! {
    /// An ID of an entity that can form bonds.
    Bondable {
        Atom,
        Pseudoatom,
        //Bond,
    }
}

define_category_id! {
    /// An ID of an entity that an `Object` can be attached to.
    Anchor {
        Atom,
        Pseudoatom,
        Bond,
        Substituent,
        Molecule,
    }
}

// Finally, some additional conversions

impl From<AtomlikeId> for FundamentalId {
    fn from(id: AtomlikeId) -> FundamentalId {
        FundamentalId(id.0)
    }
}

impl From<AtomlikeId> for BondableId {
    fn from(id: AtomlikeId) -> BondableId {
        BondableId(id.0)
    }
}

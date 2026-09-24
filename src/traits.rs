// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! General traits implemented by the different graphs and maps at each level.
//!
//! Traits specific to each layer are defined in the appropriate module.

use nalgebra as na;
use slotmap::SlotMap;

use crate::{
    entities::{Entity, Kind},
    error::*,
    view::*,
};

pub trait Store<E: Kind> {
    /// Returns a reference to the `SlotMap` that holds the entity.
    fn slotmap(&self) -> &SlotMap<E::Key, E::Data>;

    /// Returns a mutable reference to the `SlotMap` that holds the entity.
    fn slotmap_mut(&mut self) -> &mut SlotMap<E::Key, E::Data>;

    /// Returns a reference to the entity's data struct, or `None` if `entity` is invalid.
    fn data(&self, entity: E) -> Option<&E::Data> {
        self.slotmap().get(entity.to_key())
    }

    /// Returns a mutable reference to the entity's data struct, or `None` if `entity` is invalid.
    fn data_mut(&mut self, entity: E) -> Option<&mut E::Data> {
        self.slotmap_mut().get_mut(entity.to_key())
    }

    /// Returns an iterator over all the keys of a given kind of entity in the map.
    fn keys(&'_ self) -> slotmap::basic::Keys<'_, E::Key, E::Data> {
        self.slotmap().keys()
    }
}

/// A private struct providing core storage for a [`Map`] type.
///
/// Each `Map` type holds a corresponding core `Graph` type, but they are not
/// meant for downstream use. Generally, a zero-dimensional variant is provided
/// to provide a pure-graph form for users e.g. [`AtomMap0`], [`MolMap0`].
///
/// In general, the methods of each `Graph` type should be small in scope and
/// efficient so that the higher maps can combine them to create a nice public API.
/// The methods should do relatively little checking and validation, with the
/// higher maps responsible for careful usage. In particular, all IDs should be
/// assumed to be valid.
pub trait Graph {
    /// Checks if the graph currently contains the given entity.
    ///
    /// This method must be flexible and able to do the check for any Entity type,
    /// not just ones actually in the map, and including category types.
    fn contains<E: Entity>(&self, entity: E) -> bool;
}

/// An arena-like data structure to represent a set of chemical entities, their
/// properties, and the relationships between them, with or without spatial positions.
///
/// This trait provides methods for:
/// 1. obtaining an immutable or mutable view of an entity from its ID
/// 2. verifying an ID
/// 3. iterating over views of all of a given kind of entity
/// 4. iterating over all IDs of a given kind of entity
///
/// All implementors of `Map` should also always provide methods for adding new
/// entities, but the signature for these will vary according to the needs of the
/// concrete map type.
///
/// This trait is sealed and is not intended for implementation outside of `molmap`.
pub trait Map: Sized {
    // Graph access
    // ------------

    type Graph: Graph;

    /// Provides access to the core graph.
    fn graph(&self) -> &Self::Graph;

    /// Provides mutable access to the core graph.
    fn graph_mut(&mut self) -> &mut Self::Graph;

    // Constructors
    // ------------

    /// Creates an empty `MolMap`.
    ///
    /// As the constituent `SlotMap`s are created with an initial capacity of 0,
    /// reallocations will occur frequently if many entities are subsequently inserted.
    /// If you have an idea of approximately how large the `MolMap` needs to be, it is
    /// recommended to use `MolMap.with_capacity` or `with_capacities` instead.
    fn new() -> Self;

    ///// Creates a new `MolMap` with the specified initial capacities for each kind of entity.
    //fn with_capacities(
    //    atoms: usize,
    //    pseudoatoms: usize,
    //    bonds: usize,
    //    substituents: usize,
    //    molecules: usize,
    //) -> Self;

    ///// Creates a new `MolMap` with initial capacity for approximately `n` atoms.
    /////
    ///// In the default implementation, this results in a map with capacity for:
    ///// - `n` atoms
    ///// - `n / 10` pseudoatoms
    ///// - `n` bonds
    ///// - `n / 3` substituents
    ///// - `(n / 100) + 1` molecules
    //fn with_capacity(n: usize) -> Self {
    //    Self::with_capacities(n, n / 10, n, n / 3, (n / 100) + 1)
    //}

    // ID-related methods
    // ------------------

    /// Checks if the map currently contains the given entity.
    ///
    /// This method is flexible and able to do the check for any [`Entity`] type,
    /// not just ones actually in the map, and including category types.
    fn contains<E: Entity>(&self, entity: E) -> bool {
        self.graph().contains(entity)
    }

    ///// Returns an iterator over all of a given kind of entity in the map.
    //fn all<E: Kind>(&'_ self) -> AllEntities<'_, E> {
    //    AllEntities::from_keys(self.core().keys::<E>())
    //}

    // Getters
    // -------
    // One method per entity kind (via monomorphization) for:
    // - getting a view
    // - getting a mutable view
    // - iterating over (immutable) views

    /// Constructs an immutable view of the given entity, returning `None` if the ID is invalid.
    fn view<E: Kind>(&'_ self, entity: E) -> Option<View<'_, Self, E>> {
        self.contains(entity).then_some(View {
            map: self,
            id: entity,
        })
    }

    /// Constructs a mutable view of the given entity, returning `None` if the ID is invalid.
    fn view_mut<E: Kind>(&'_ mut self, entity: E) -> Option<ViewMut<'_, Self, E>> {
        self.contains(entity).then_some(ViewMut {
            map: self,
            id: entity,
        })
    }

    /// Returns an iterator over views of the given entities, returning an error
    /// if any ID is invalid.
    ///
    /// This method is most useful for situations where it is important to have
    /// ensured all IDs are valid before beginning some operation involving them.
    ///
    /// As the IDs are validated eagerly and then stored on the heap, in other
    /// situations it is probably more sensible to use `map` on the ID iterator
    /// to get an iterator that returns views for each entity in turn, lazily.
    ///
    /// # Errors
    ///
    /// Fails if any ID is invalid, in which case the error includes the first
    /// invalid ID encountered.
    fn views<E, I>(&'_ self, entities: I) -> MolMapResult<Views<'_, Self, E>>
    where
        E: Entity,
        I: IntoIterator<Item = E>,
    {
        let validated: Vec<E> = entities
            .into_iter()
            .map(|e| {
                if self.contains(e) {
                    Ok(e)
                } else {
                    Err(MolMapError::InvalidId(e.as_entity()))
                }
            })
            // An iterator of Result<T, U> can be collected into Result<Vec<T>, U>
            .collect::<MolMapResult<Vec<E>>>()?;
        Ok(Views {
            map: self,
            ids: validated.into_iter(),
        })
    }

    ///// Returns an iterator over views of all of a given kind of entity in the map.
    //fn all_views<E: Kind>(&'_ self) -> Views<'_, Self, E> {
    //    let all: Vec<E> = self.all().collect();
    //    Views {
    //        map: self,
    //        ids: all.into_iter(),
    //    }
    //}
}

// Any Map type automatically implements Store for the same set of entity types
// that the inner graph implements it for
impl<E, M> Store<E> for M
where
    E: Kind,
    M: Map,
    M::Graph: Store<E>,
{
    fn slotmap(&self) -> &SlotMap<<E>::Key, <E>::Data> {
        self.graph().slotmap()
    }

    fn slotmap_mut(&mut self) -> &mut SlotMap<<E>::Key, <E>::Data> {
        self.graph_mut().slotmap_mut()
    }
}

pub trait Spatial {
    const DIM: usize;

    type Dim: na::DimName;

    type Point: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Default
        + std::ops::Add<Self::Vector, Output = Self::Point>
        + std::ops::Sub<Self::Point, Output = Self::Vector>;

    type Vector: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Default
        + std::ops::Add<Output = Self::Vector>
        + std::ops::Sub<Output = Self::Vector>
        + std::ops::Mul<f64, Output = Self::Vector>
        + std::ops::Div<f64, Output = Self::Vector>
        + std::ops::Neg<Output = Self::Vector>;
}

pub trait TwoDimensional:
    Spatial<Dim = na::U2, Point = na::Point2<f64>, Vector = na::Vector2<f64>>
{
}

pub trait ThreeDimensional:
    Spatial<Dim = na::U3, Point = na::Point3<f64>, Vector = na::Vector3<f64>>
{
}

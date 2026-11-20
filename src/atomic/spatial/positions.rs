// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nalgebra as na;

use crate::{
    entities::*,
    error::Error,
    geometry::{self, Point, Vector},
    traits::{Spatial, Stores},
    units::Unit,
    view::View,
};

use super::super::Atomic;
use super::SpatialAtomMap;

/// Methods common to all spatial maps.
///
/// The purpose of this trait is to allow internal methods common to several spatial
/// map types to be written only once. The trait and its methods are not intended to
/// be exposed publicly. Functionality that should be available as part of the
/// public API is generally implemented via view methods, or in some cases via
/// inherent wrapper methods on the map type itself.
///
/// The current way to express that a map is both "spatial" and "atomic" via bounds
/// is to use both the respective traits as bounds. However, that does not allow
/// generic use of this trait's methods. Should a need for that arise in future,
/// `pub trait SpatialAtomic<const D: usize>: Spatial<D> + Atomic + SpatialAtomicCommon<D>`
/// should be introduced and blanket implemented for all maps that fulfil the
/// bounds -- this trait should *not* simply be made public.
pub(crate) trait AtomicPositions<const D: usize>: Spatial<D> + Atomic {
    /// Returns the position of the atom, or `None` if it is invalid.
    fn atom_position(&self, atom: Atom) -> Option<&Point<D>>;

    /// Sets the position of the atom.
    ///
    /// Can silently fail and return `None` if the atom was removed.
    ///
    /// Returns `None` if the atom did not previously have a position, or the
    /// old value otherwise.
    ///
    /// As such, this method does not panic – if the atom is not in the map,
    /// nothing happens, and `None` is returned.
    fn set_atom_position(&mut self, atom: Atom, new: Point<D>) -> Option<Point<D>>;

    /// Returns the position of the pseudoatom, or `None` if it is invalid.
    fn pseudoatom_position(&self, pseudoatom: Pseudoatom) -> Option<&Point<D>>;

    /// Sets the position of the pseudoatom.
    ///
    /// Can silently fail and return `None` if the pseudoatom was removed.
    ///
    /// Returns `None` if the pseudoatom did not previously have a position, or the
    /// old value otherwise.
    ///
    /// As such, this method does not panic – if the pseudoatom is not in the map,
    /// nothing happens, and `None` is returned.
    fn set_pseudoatom_position(
        &mut self,
        pseudoatom: Pseudoatom,
        new: Point<D>,
    ) -> Option<Point<D>>;

    /// Returns the position of the atom or pseudoatom, or `None` if it is invalid.
    fn atomlike_position(&self, atomlike: impl Atomlike) -> Option<&Point<D>> {
        match Atomlike::to_resolved(atomlike) {
            ResolvedAtomlike::Atom(atom) => self.atom_position(atom),
            ResolvedAtomlike::Pseudoatom(pseudoatom) => self.pseudoatom_position(pseudoatom),
        }
    }

    /// Sets the position of the atom or pseudoatom.
    ///
    /// Can silently fail and return `None` if the atomlike was removed.
    ///
    /// Returns `None` if the atomlike did not previously have a position, or the
    /// old value otherwise.
    ///
    /// As such, this method does not panic – if the atomlike is not in the map,
    /// nothing happens, and `None` is returned.
    #[expect(unused)]
    fn set_atomlike_position(
        &mut self,
        atomlike: impl Atomlike,
        new: Point<D>,
    ) -> Option<Point<D>> {
        match Atomlike::to_resolved(atomlike) {
            ResolvedAtomlike::Atom(atom) => self.set_atom_position(atom, new),
            ResolvedAtomlike::Pseudoatom(pseudoatom) => {
                self.set_pseudoatom_position(pseudoatom, new)
            }
        }
    }

    /// Returns the position of the bondable, or `None` if it is invalid.
    fn bondable_position(&self, bondable: impl Bondable) -> Option<&Point<D>> {
        match Bondable::to_resolved(bondable) {
            ResolvedBondable::Atom(atom) => self.atom_position(atom),
            ResolvedBondable::Pseudoatom(pseudoatom) => self.pseudoatom_position(pseudoatom),
        }
    }

    /// Calculates the vector of the line between two atomlikes, from `a` to `b`.
    ///
    /// # Errors
    ///
    /// Fails if either atomlike has no stored position (usually meaning that it is not
    /// in the map).
    #[expect(unused)]
    fn interatomlike_line(&self, a: impl Atomlike, b: impl Atomlike) -> Result<Vector<D>, Error> {
        let pos_a = self
            .atomlike_position(b)
            .ok_or(Error::InvalidId(b.as_entity()))?;
        let pos_b = self
            .atomlike_position(a)
            .ok_or(Error::InvalidId(a.as_entity()))?;
        Ok(pos_b - pos_a)
    }

    /// Returns the position of the [`Bondable`] from which the bond starts.
    ///
    /// # Errors
    ///
    /// Fails if the bond is not in the map.
    fn bond_origin(&self, bond: Bond) -> Result<&Point<D>, Error> {
        let bond_data = self
            .as_atomic()
            .data(bond)
            .ok_or(Error::InvalidId(bond.into()))?;
        let origin = self.bondable_position(bond_data.start).expect("If the bond exists, the bonding partners must exist, and any existing atom must have a position");
        Ok(origin)
    }

    /// Returns the position of the [`Bondable`] at which the bond ends.
    ///
    /// # Errors
    ///
    /// Fails if the bond is not in the map.
    fn bond_terminus(&self, bond: Bond) -> Result<&Point<D>, Error> {
        let bond_data = self
            .as_atomic()
            .data(bond)
            .ok_or(Error::InvalidId(bond.into()))?;
        let terminus = self.bondable_position(bond_data.end).expect("If the bond exists, the bonding partners must exist, and any existing atom must have a position");
        Ok(terminus)
    }

    /// Calculates the midpoint of the bond.
    ///
    /// # Errors
    ///
    /// Fails if the bond is not in the map.
    fn bond_midpoint(&self, bond: Bond) -> Result<Point<D>, Error> {
        let origin = self.bond_origin(bond)?;
        let terminus = self.bond_terminus(bond)?;
        let midpoint = na::center(origin, terminus);
        Ok(midpoint)
    }

    /// Calculates the vector that the bond follows from its origin to its terminus.
    ///
    /// # Errors
    ///
    /// Fails if the bond is not in the map.
    fn bond_vector(&self, bond: Bond) -> Result<Vector<D>, Error> {
        let origin = self.bond_origin(bond)?;
        let terminus = self.bond_terminus(bond)?;
        Ok(terminus - origin)
    }

    /// Calculates the unweighted geometric centre of the specified atoms and
    /// pseudoatoms.
    ///
    /// # Errors
    ///
    /// Fails (returning [`molmap::Error::InvalidId`]) if any of the atomlikes are not in
    /// the map.
    ///
    /// Fails (returning [`molmap::Error::EmptyIterator`]) if the provided iterator is
    /// empty.
    fn centroid(
        &self,
        atomlikes: impl IntoIterator<Item = AnyAtomlike>,
    ) -> Result<Point<D>, Error> {
        let positions = atomlikes
            .into_iter()
            .map(|x| {
                self.atomlike_position(x)
                    .ok_or(Error::InvalidId(x.as_entity()))
            })
            .collect::<Result<Vec<&Point<D>>, Error>>()?;
        geometry::mean_point(positions).ok_or(Error::EmptyIterator)
    }
}

impl<const D: usize, U: Unit> AtomicPositions<D> for SpatialAtomMap<D, U> {
    fn atom_position(&self, atom: Atom) -> Option<&Point<D>> {
        // When an entity is removed from the map, any spatial data is not deleted.
        // If the slot has not since been overwritten, it is possible to obtain a stale
        // position from the SecondaryMap using the stale ID, so we must first confirm
        // the ID's validity.
        if self.contains(atom) {
            self.atom_positions.get(atom.into())
        } else {
            None
        }
    }

    fn set_atom_position(&mut self, atom: Atom, new: Point<D>) -> Option<Point<D>> {
        // Here, slotmap's insert method takes care of the checking for us
        // The promised behaviour of set_position matches that of slotmap::insert
        self.atom_positions.insert(atom.into(), new)
    }

    fn pseudoatom_position(&self, pseudoatom: Pseudoatom) -> Option<&Point<D>> {
        if self.contains(pseudoatom) {
            self.pseudoatom_positions.get(pseudoatom.into())
        } else {
            None
        }
    }

    fn set_pseudoatom_position(
        &mut self,
        pseudoatom: Pseudoatom,
        new: Point<D>,
    ) -> Option<Point<D>> {
        self.pseudoatom_positions.insert(pseudoatom.into(), new)
    }
}

impl<'m, const D: usize, U: Unit> View<'m, SpatialAtomMap<D, U>, Atom> {
    /// Returns the position of the atom.
    pub fn position(&self) -> &Point<D> {
        self.map.atom_position(self.id).unwrap()
    }
}

impl<'m, const D: usize, U: Unit> View<'m, SpatialAtomMap<D, U>, Pseudoatom> {
    /// Returns the position of the pseudoatom.
    pub fn position(&self) -> &Point<D> {
        self.map.pseudoatom_position(self.id).unwrap()
    }
}

impl<'m, const D: usize, U: Unit> View<'m, SpatialAtomMap<D, U>, Bond> {
    /// Returns the position of the [`Bondable`] from which the bond starts.
    pub fn origin(&self) -> &Point<D> {
        self.map.bond_origin(self.id).unwrap()
    }

    /// Returns the position of the [`Bondable`] at which the bond ends.
    pub fn terminus(&self) -> &Point<D> {
        self.map.bond_terminus(self.id).unwrap()
    }

    /// Calculates the midpoint of the bond.
    pub fn midpoint(&self) -> Point<D> {
        self.map.bond_midpoint(self.id).unwrap()
    }

    /// Calculates the vector that the bond follows from its origin to its terminus.
    pub fn vector(&self) -> Vector<D> {
        self.map.bond_vector(self.id).unwrap()
    }

    /// Calculates the distance from the bond's origin to its terminus.
    pub fn length(&self) -> f64 {
        self.vector().magnitude()
    }
}

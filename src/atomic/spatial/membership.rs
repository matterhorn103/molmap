// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{entities::*, traits::Map, units::Unit, view::ViewMut};

use super::SpatialAtomMap;

// For now the deletion methods are identical to the equivalents for AtomMap.
// They may well diverge in future e.g. if we cache emergent information like
// the bond vectors or the centroids or bounding boxes of the collections.
//
// It is not necessary to delete the positions from the SecondaryMap; they will
// be overwritten.

impl<const D: usize, U> SpatialAtomMap<D, U> {
    pub fn contains<E: Entity>(&self, entity: E) -> bool {
        self.graph.contains(entity)
    }
}

impl<'m, const D: usize, U: Unit> ViewMut<'m, SpatialAtomMap<D, U>, Atom> {
    /// Removes the atom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.graph.delete_atom(self.id);
    }

    // /// Sets the position of the atom to the provided value.
    // pub fn set_position(self, position: Point<D>) {
    //     self.map.set_position(self.id, position);
    // }
}

impl<'m, const D: usize, U: Unit> ViewMut<'m, SpatialAtomMap<D, U>, Pseudoatom> {
    /// Removes the pseudoatom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.graph.delete_pseudoatom(self.id);
    }

    // /// Sets the position of the pseudoatom to the provided value.
    // pub fn set_position(self, position: Point<D>) {
    //     self.map.set_position(self.id, position);
    // }
}

impl<'m, const D: usize, U: Unit> ViewMut<'m, SpatialAtomMap<D, U>, Bond> {
    /// Removes the bond from the map (but not its bonding partners).
    pub fn delete(self) {
        self.map.graph.delete_bond(self.id);
    }
}

#[cfg(test)]
mod tests {
    use crate::{elements::Element, geometry::Point3, units::Relative};

    use super::*;

    #[test]
    fn delete_atom() {
        let mut am: SpatialAtomMap<3, Relative> = SpatialAtomMap::new();
        let h1 = am.add_atom(Element::H, Point3::new(1.0, 2.0, 3.0));
        assert_eq!(am.iter::<Atom>().count(), 1);
        assert_eq!(am.atom_positions.len(), 1);
        am.view_mut(h1).unwrap().delete();
        assert_eq!(am.iter::<Atom>().count(), 0);
        // Note that stale position is still present, but can't be accessed
        assert!(am.atom_positions.contains_key(h1.to_key()));
        assert!(am.view(h1).is_none());
    }
}

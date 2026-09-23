// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The data held by a `MolGraph` for the basic collections and the view methods
//! to access it.

use std::collections::HashSet;

use crate::{entities::*, error::MolMapResult, view::*};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SubstituentCentre {
    None,
    Single(AnyAtomlike),
    // Use Box around Vec so that SubstituentCentre takes up less space
    // Clippy doesn't like this but since the vast majority of substituents will
    // be single centre the extra redirection in the multi-centre case is worth it
    #[allow(clippy::box_collection)]
    Multiple(Box<Vec<AnyAtomlike>>),
}

/// The core data of a substituent entity.
#[derive(Clone, Debug)]
pub struct SubstituentData {
    pub(crate) centre: SubstituentCentre,
    pub(crate) members: Vec<AnyFundamental>,
}

impl SubstituentData {
    pub(crate) fn new(centre: AnyAtomlike, members: &[AnyFundamental]) -> Self {
        Self {
            centre: SubstituentCentre::Single(centre),
            members: members.to_vec(),
        }
    }
}

impl Stored for Substituent {
    type Data = SubstituentData;
}

//impl<'m, M> View<'m, M, Substituent> {
//    /// Returns details of the centre(s) of the substituent.
//    pub fn centre(&self) -> &SubstituentCentre {
//        &self.data().centre
//    }
//
//    /// Returns an iterator over the IDs of all constituent atoms, pseudoatoms, and bonds.
//    pub fn members(&self) -> impl Iterator<Item = AnyFundamental> {
//        self.data().members.iter().copied()
//    }
//
//    /// Checks if the substituent contains the given atom, pseudoatom, or bond.
//    pub fn contains(&self, fundamental: impl Fundamental) -> bool {
//        self.data().members.contains(&fundamental.as_fundamental())
//    }
//}
//
//impl<'m, M> ViewMut<'m, M, Substituent> {
//    /// Attempts to change the centre of the substituent to the one requested.
//    ///
//    /// # Errors
//    ///
//    /// Fails if the requested centre is not already a member of the substituent,
//    /// or if there are already bonds to the current centre(s).
//    pub fn set_centre(self, new_centre: impl Atomlike) -> MolMapResult<()> {
//        self.map
//            .core_mut()
//            .set_substituent_centre(self.id, new_centre)
//    }
//
//    /// Makes the requested atomlike a centre of the substituent, in addition to any
//    /// already existing centres.
//    ///
//    /// # Errors
//    ///
//    /// Fails if the requested centre is not already a member of the substituent.
//    pub fn add_centre(self, new_centre: impl Atomlike) -> MolMapResult<()> {
//        self.map
//            .core_mut()
//            .add_substituent_centre(self.id, new_centre)
//    }
//}

/// The core data of a molecule entity.
#[derive(Clone, Debug)]
pub struct MoleculeData {
    pub(crate) members: HashSet<AnyFundamental>,
}

impl MoleculeData {
    pub(crate) fn new() -> Self {
        Self {
            members: HashSet::new(),
        }
    }
}

impl Stored for Molecule {
    type Data = MoleculeData;
}

//impl<'m, M> View<'m, M, Molecule> {
//    /// Returns an iterator over the IDs of all constituent atoms, pseudoatoms, and bonds.
//    pub fn members(&self) -> impl Iterator<Item = AnyFundamental> {
//        self.data().members.iter().copied()
//    }
//
//    /// Checks if the molecule contains the given atom, pseudoatom, or bond.
//    pub fn contains(&self, fundamental: impl Fundamental) -> bool {
//        self.data().members.contains(&fundamental.as_fundamental())
//    }
//}

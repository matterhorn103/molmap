// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use slotmap::new_key_type;

use crate::{
    ids::{AtomlikeId, BondId, FundamentalId},
    traits::MolMap,
};

new_key_type! {
    /// An ID corresponding to a specific substituent entity in a `MolMap`.
    pub struct SubstituentId;
}

#[derive(Debug)]
pub(crate) enum SubstituentCentre {
    Ambiguous(Vec<BondId>),
    Single(AtomlikeId),
    Multiple(Vec<AtomlikeId>),
}

impl Default for SubstituentCentre {
    /// Creates an ambiguous centre with an empty vector of bonds.
    fn default() -> Self {
        SubstituentCentre::Ambiguous(Vec::new())
    }
}

/// The core data of a substituent entity.
///
/// Substituents are the smallest grouping in a MolMap
/// Substituents are conceptually equivalent to a non-hydrogen atom and "its" implicit
/// hydrogen atoms in SMILES or in packages that work that way,
/// or to the groups drawn together without explicit bonds in a skeletal formula
/// e.g. –OH, –COOH, –CH3
/// Substituents have an internal structure of Atoms, Pseudoatoms, and Bonds
/// Substituents generally indicate one or more centres to which bonds can be made,
/// but occasionally bonds are made to a substituent as a whole.
#[derive(Debug)]
pub(crate) struct Substituent {
    pub(crate) centre: SubstituentCentre,
    pub(crate) members: Vec<FundamentalId>,
}

impl Substituent {
    pub(crate) fn new(members: &[FundamentalId]) -> Self {
        Self {
            centre: SubstituentCentre::default(),
            members: members.to_vec(),
        }
    }
}

/// An immutable view over a specific substituent entity in a specific `MolMap`.
#[derive(Clone, Copy)]
pub struct SubstituentView<'a, M: MolMap> {
    pub molmap: &'a M,
    pub id: SubstituentId,
}

impl<'a, M: MolMap> From<SubstituentView<'a, M>> for SubstituentId {
    fn from(view: SubstituentView<'a, M>) -> Self {
        view.id
    }
}

impl<'a, M: MolMap> SubstituentView<'a, M> {
    fn core(&self) -> &'a Substituent {
        self.molmap.core().substituents.get(self.id).unwrap()
    }
}

/// A mutable view over a specific substituent entity in a specific `MolMap`.
pub struct SubstituentViewMut<'a, M: MolMap> {
    pub molmap: &'a mut M,
    pub id: SubstituentId,
}

impl<'a, M: MolMap> From<SubstituentViewMut<'a, M>> for SubstituentId {
    fn from(view: SubstituentViewMut<'a, M>) -> Self {
        view.id
    }
}

impl<'a, M: MolMap> SubstituentViewMut<'a, M> {
    fn core(&mut self) -> &mut Substituent {
        self.molmap
            .core_mut()
            .substituents
            .get_mut(self.id)
            .unwrap()
    }
}

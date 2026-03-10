// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{AtomId, Atomlike, BondId, FragmentId, IdError, MolMap, PseudoatomId};

// Maybe better to use Nodes after all?
// Presumably getting the length of a Vec is almost zero cost
#[derive(Clone, Debug)]
pub enum FragmentBondingCentre {
    Ambiguous(Vec<BondId>),
    Single(Atomlike),
    Multiple(Vec<Atomlike>),
}

// Fragments are the smallest non-fundamental grouping in a MolMap
// Fragments are conceptually equivalent to a non-hydrogen atom and "its" implicit
// hydrogen atoms in SMILES or in packages that work that way,
// or to the groups drawn together without explicit bonds in a skeletal formula
// e.g. –OH, –COOH, –CH3
// Fragments have an internal structure of Atoms, Pseudoatoms, and Bonds
// Atoms and Pseudoatoms cannot exist other than inside a Fragment
// Fragments generally indicate one or more centres to which bonds can be made
#[derive(Debug)]
pub struct Fragment {
    pub id: FragmentId,
    pub centre: FragmentBondingCentre,
    pub atoms: Vec<AtomId>,
    pub pseudoatoms: Vec<PseudoatomId>,
    pub bonds: Vec<BondId>,
}

impl Fragment {
    pub fn new(id: FragmentId, centre: FragmentBondingCentre) -> Self {
        Self {
            id,
            centre,
            atoms: Vec::new(),
            pseudoatoms: Vec::new(),
            bonds: Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FragmentView<'a, E> {
    pub molmap: &'a MolMap<E>,
    pub id: FragmentId,
}

impl<'a, E> From<FragmentView<'a, E>> for FragmentId {
    fn from(view: FragmentView<'a, E>) -> Self {
        view.id
    }
}

impl<'a, E> FragmentView<'a, E> {
    fn inner(&self) -> &'a Fragment {
        self.molmap.fragments.get(self.id).unwrap()
    }
    
    pub fn centre(&self) -> &FragmentBondingCentre {
        &self.inner().centre
    }
}

pub struct FragmentViewMut<'a, E> {
    pub molmap: &'a mut MolMap<E>,
    pub id: FragmentId,
}

impl<'a, E> From<FragmentViewMut<'a, E>> for FragmentId {
    fn from(view: FragmentViewMut<'a, E>) -> Self {
        view.id
    }
}

impl<'a, E> FragmentViewMut<'a, E> {
    fn as_ref(&self) -> FragmentView<'_, E> {
        FragmentView {
            molmap: &*self.molmap,
            id: self.id,
        }
    }

    fn inner(&mut self) -> &mut Fragment {
        self.molmap.fragments.get_mut(self.id).unwrap()
    }
}

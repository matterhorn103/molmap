// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{entities::*, error::Error, traits::Stores, view::ViewMut};

use super::super::data::*;
use super::MolMap;

/// Methods for entity removal.
impl MolMap {
    /// Removes an atom from the map, as well as any bonds to it.
    ///
    /// Returns whether the atom was present in the map.
    ///
    /// This is infallible – if the atom is not in the map, nothing changes.
    #[expect(unused_variables)]
    pub(crate) fn delete_atom(&mut self, atom: Atom) -> bool {
        // if !self.contains(atom) {
        //     return false;
        // }
        // // Make sure we always remove bonds first
        // let bonds = self.data(atom).unwrap().bonds.clone();
        // for bond in bonds {
        //     self.delete_bond(bond);
        // }
        // // Remove from any collections
        // if let Some(sub) = self.parent_substituent(atom) {
        //     self.remove_from_substituent(sub, atom);
        // }
        // if let Some(mol) = self.parent_molecule(atom) {
        //     self.remove_from_molecule(mol, atom);
        // }
        // // Now we can safely remove the atom itself without leaving dangling bonds
        // self.atoms.remove(atom.into()).is_some() // Should always be `true`
        todo!()
    }

    /// Removes a pseudoatom from the map, as well as any bonds to it.
    ///
    /// Returns whether the pseudoatom was present in the map.
    ///
    /// This is infallible – if the pseudoatom is not in the map, nothing changes.
    #[expect(unused_variables)]
    pub(crate) fn delete_pseudoatom(&mut self, pseudoatom: Pseudoatom) -> bool {
        // if !self.contains(pseudoatom) {
        //     return false;
        // }
        // // Make sure we always remove bonds first
        // let bonds = self.data(pseudoatom).unwrap().bonds.clone();
        // for bond in bonds {
        //     self.delete_bond(bond);
        // }
        // // Remove from any collections
        // if let Some(sub) = self.parent_substituent(pseudoatom) {
        //     self.remove_from_substituent(sub, pseudoatom);
        // }
        // if let Some(mol) = self.parent_molecule(pseudoatom) {
        //     self.remove_from_molecule(mol, pseudoatom);
        // }
        // // Now we can safely remove the pseudoatom itself without leaving dangling bonds
        // self.pseudoatoms.remove(pseudoatom.into()).is_some()
        todo!()
    }

    /// Removes a bond from the map (but not its bonding partners).
    ///
    /// Returns whether the bond was present in the map.
    ///
    /// If the bond is not in the map, nothing changes.
    ///
    /// # Panics
    ///
    /// Panics if either of the bond's bonding partners does not exist (which
    /// should never be the case – bonds are last in, first out).
    #[expect(unused_variables)]
    pub(crate) fn delete_bond(&mut self, bond: Bond) -> bool {
        // if let Some(bond_data) = self.bonds.remove(bond.into()) {
        //     for bonding_partner in [bond_data.start, bond_data.end] {
        //         match bonding_partner.resolve() {
        //             ResolvedBondable::Atom(atom) => {
        //                 let atom_data = self
        //                     .data_mut(atom)
        //                     .expect("Bonds are always removed before their bonding partners");
        //                 let pos = atom_data.bonds.iter().position(|x| *x == bond).expect(
        //                     "Bond should be listed in the bonding partner's bonds until deletion",
        //                 );
        //                 atom_data.bonds.remove(pos);
        //             }
        //             ResolvedBondable::Pseudoatom(pseudoatom) => {
        //                 let pseudoatom_data = self
        //                     .data_mut(pseudoatom)
        //                     .expect("Bonds are always removed before their bonding partners");
        //                 let pos = pseudoatom_data.bonds.iter().position(|x| *x == bond).expect(
        //                     "Bond should be listed in the bonding partner's bonds until deletion",
        //                 );
        //                 pseudoatom_data.bonds.remove(pos);
        //             }
        //         }
        //     }
        //     // Remove from any collections
        //     if let Some(sub) = self.parent_substituent(bond) {
        //         self.remove_from_substituent(sub, bond);
        //     }
        //     if let Some(mol) = self.parent_molecule(bond) {
        //         self.remove_from_molecule(mol, bond);
        //     }
        //     true
        // } else {
        //     false
        // }
        todo!()
    }

    /// Removes a substituent from the map, as well as all of its members.
    ///
    /// Returns whether the substituent was present in the map.
    ///
    /// This is infallible – if the substituent is not in the map, nothing changes.
    #[expect(unused_variables)]
    pub(crate) fn delete_substituent(&mut self, substituent: Substituent) -> bool {
        // if !self.contains(substituent) {
        //     return false;
        // };
        // let members = self.data(substituent).unwrap().members.clone();
        // for member in members {
        //     match member.resolve() {
        //         ResolvedFundamental::Atom(atom) => {
        //             self.delete_atom(atom);
        //         }
        //         ResolvedFundamental::Pseudoatom(pseudoatom) => {
        //             self.delete_pseudoatom(pseudoatom);
        //         }
        //         ResolvedFundamental::Bond(bond) => {
        //             self.delete_bond(bond);
        //         }
        //     }
        // }
        // self.substituents.remove(substituent.into()).is_some()
        todo!()
    }

    /// Removes a molecule from the map, as well as all of its members.
    ///
    /// Returns whether the molecule was present in the map.
    ///
    /// This is infallible – if the molecule is not in the map, nothing changes.
    #[expect(unused_variables)]
    pub(crate) fn delete_molecule(&mut self, molecule: Molecule) -> bool {
        // if !self.contains(molecule) {
        //     return false;
        // };
        // let members = self.data(molecule).unwrap().members.clone();
        // for member in members {
        //     match member.resolve() {
        //         ResolvedFundamental::Atom(id) => {
        //             self.delete_atom(id);
        //         }
        //         ResolvedFundamental::Pseudoatom(id) => {
        //             self.delete_pseudoatom(id);
        //         }
        //         ResolvedFundamental::Bond(id) => {
        //             self.delete_bond(id);
        //         }
        //     }
        // }
        // self.molecules.remove(molecule.into()).is_some()
        todo!()
    }
}

/// Methods to query or ascertain membership.
impl MolMap {
    pub fn contains<E: Entity>(&self, entity: E) -> bool {
        match entity.to_resolved() {
            ResolvedEntity::Atom(atom) => self.inner.contains(atom),
            ResolvedEntity::Bond(bond) => self.inner.contains(bond),
            ResolvedEntity::Pseudoatom(pseudoatom) => self.inner.contains(pseudoatom),
            ResolvedEntity::Substituent(substituent) => {
                <MolMap as Stores<Substituent>>::slotmap(self).contains_key(substituent.to_key())
            }
            ResolvedEntity::Molecule(molecule) => {
                <MolMap as Stores<Molecule>>::slotmap(self).contains_key(molecule.to_key())
            }
            #[expect(unreachable_patterns)] // Unreachable for now but will be needed later
            _ => false,
        }
    }

    /// Checks if the substituent contains the given fundamental.
    ///
    /// # Errors
    ///
    /// Fails if the substituent does not exist.
    pub(crate) fn substituent_contains(
        &self,
        substituent: Substituent,
        fundamental: impl Fundamental,
    ) -> Result<bool, Error> {
        let sub_data = self
            .data(substituent)
            .ok_or(Error::InvalidId(substituent.into()))?;
        let is_member = match Fundamental::to_resolved(fundamental) {
            ResolvedFundamental::Atom(atom) => sub_data.atomlikes.contains(&atom.as_atomlike()),
            ResolvedFundamental::Pseudoatom(pseudoatom) => {
                sub_data.atomlikes.contains(&pseudoatom.as_atomlike())
            }
            ResolvedFundamental::Bond(bond) => sub_data.bonds.contains(&bond),
        };
        Ok(is_member)
    }

    /// Checks if the molecule contains the given fundamental.
    ///
    /// # Errors
    ///
    /// Fails if the molecule does not exist.
    pub(crate) fn molecule_contains(
        &self,
        molecule: Molecule,
        fundamental: impl Fundamental,
    ) -> Result<bool, Error> {
        let mol_data = self
            .data(molecule)
            .ok_or(Error::InvalidId(molecule.into()))?;
        let is_member = match Fundamental::to_resolved(fundamental) {
            ResolvedFundamental::Atom(atom) => mol_data.substituents.iter().any(|sub| {
                self.data(*sub)
                    .expect("Any substituent listed as a member should be valid")
                    .atomlikes
                    .contains(&atom.as_atomlike())
            }),
            ResolvedFundamental::Pseudoatom(pseudoatom) => {
                mol_data.substituents.iter().any(|sub| {
                    self.data(*sub)
                        .expect("Any substituent listed as a member should be valid")
                        .atomlikes
                        .contains(&pseudoatom.as_atomlike())
                })
            }
            ResolvedFundamental::Bond(bond) => {
                mol_data.bonds.contains(&bond)
                    || mol_data.substituents.iter().any(|sub| {
                        self.data(*sub)
                            .expect("Any substituent listed as a member should be valid")
                            .bonds
                            .contains(&bond)
                    })
            }
        };
        Ok(is_member)
    }

    /// Returns an iterator over the IDs of all a substituent's constituent fundamentals.
    ///
    /// # Errors
    ///
    /// Fails if the substituent does not exist.
    pub(crate) fn substituent_members(
        &self,
        substituent: Substituent,
    ) -> Result<impl Iterator<Item = AnyFundamental>, Error> {
        let sub_data = self
            .data(substituent)
            .ok_or(Error::InvalidId(substituent.into()))?;
        let mut members: Vec<AnyFundamental> = sub_data
            .atomlikes
            .iter()
            .map(|x| x.as_fundamental())
            .collect();
        members.extend(sub_data.bonds.iter().map(|x| x.as_fundamental()));
        Ok(members.into_iter())
    }

    /// Returns an iterator over the IDs of all constituent atoms, pseudoatoms, and bonds.
    ///
    /// # Errors
    ///
    /// Fails if the molecule does not exist.
    pub(crate) fn molecule_members(
        &self,
        molecule: Molecule,
    ) -> Result<impl Iterator<Item = AnyFundamental>, Error> {
        let mol_data = self
            .data(molecule)
            .ok_or(Error::InvalidId(molecule.into()))?;
        let mut members: Vec<AnyFundamental> =
            mol_data.bonds.iter().map(|x| x.as_fundamental()).collect();
        for sub in &mol_data.substituents {
            members.extend(
                self.data(*sub)
                    .expect("Any substituent listed as a member should be valid")
                    .atomlikes
                    .iter()
                    .map(|x| x.as_fundamental()),
            );
            members.extend(
                self.data(*sub)
                    .expect("Any substituent listed as a member should be valid")
                    .bonds
                    .iter()
                    .map(|x| x.as_fundamental()),
            );
        }
        Ok(members.into_iter())
    }

    // /// Determines the substituent that contains the atom, pseudoatom, or bond, if any.
    // pub(crate) fn parent_substituent(&self, fundamental: impl Fundamental) -> Option<Substituent> {
    //     for (sub, sub_data) in self.substituents.iter() {
    //         if sub_data.members.contains(&fundamental.as_fundamental()) {
    //             return Some(sub.into());
    //         }
    //     }
    //     None
    // }

    // /// Determines the molecule that contains the atom, pseudoatom, or bond, if any.
    // pub(crate) fn parent_molecule(&self, fundamental: impl Fundamental) -> Option<Molecule> {
    //     for (mol, mol_data) in self.molecules.iter() {
    //         if mol_data.members.contains(&fundamental.as_fundamental()) {
    //             return Some(mol.into());
    //         }
    //     }
    //     None
    // }
}

/// Methods to change collection membership.
impl MolMap {
    // /// Adds an atom, pseudoatom, or bond to a substituent.
    // ///
    // /// Returns whether the fundamental was newly inserted.
    // ///
    // /// If the fundamental is already a member of another substituent, it is removed
    // /// from it before it is inserted into this one.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `substituent` is invalid, but is unaffected if `fundamental` is
    // /// invalid.
    // pub(crate) fn insert_into_substituent(
    //     &mut self,
    //     substituent: Substituent,
    //     fundamental: impl Fundamental,
    // ) -> bool {
    //     if let Some(parent) = self.parent_substituent(fundamental) {
    //         if parent == substituent {
    //             // Already a member of this substituent
    //             return false;
    //         } else {
    //             self.remove_from_substituent(parent, fundamental);
    //         }
    //     }
    //     self.insert_into_substituent_unchecked(substituent, fundamental)
    // }

    // /// Adds an atom, pseudoatom, or bond to a substituent.
    // ///
    // /// Returns whether the fundamental was newly inserted.
    // ///
    // /// This method should only ever be used with fundamentals that do not already
    // /// belong to a substituent.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `substituent` is invalid, but is unaffected if `fundamental` is
    // /// invalid.
    // pub(crate) fn insert_into_substituent_unchecked(
    //     &mut self,
    //     substituent: Substituent,
    //     fundamental: impl Fundamental,
    // ) -> bool {
    //     let sub = self.substituents.get_mut(substituent.into()).unwrap();
    //     let fund = fundamental.as_fundamental();
    //     // members is just a Vec, so have to manually make sure we don't end up with
    //     // any ID in the substituent twice
    //     if !sub.members.contains(&fund) {
    //         sub.members.push(fund);
    //         true
    //     } else {
    //         false
    //     }
    // }

    // /// Adds an atom, pseudoatom, or bond to a molecule.
    // ///
    // /// Returns whether the fundamental was newly inserted.
    // ///
    // /// If the fundamental is already a member of another molecule, it is removed
    // /// from it before it is inserted into this one.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `molecule` is invalid, but is unaffected if `fundamental` is
    // /// invalid.
    // pub(crate) fn insert_into_molecule(
    //     &mut self,
    //     molecule: Molecule,
    //     fundamental: impl Fundamental,
    // ) -> bool {
    //     if let Some(parent) = self.parent_molecule(fundamental) {
    //         if parent == molecule {
    //             // Already a member of this molecule
    //             return false;
    //         } else {
    //             self.remove_from_molecule(parent, fundamental);
    //         }
    //     }
    //     self.insert_into_molecule_unchecked(molecule, fundamental)
    // }

    // /// Adds an atom, pseudoatom, or bond to a molecule.
    // ///
    // /// Returns whether the fundamental was newly inserted.
    // ///
    // /// This method should only ever be used with fundamentals that do not already
    // /// belong to a molecule.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `molecule` is invalid, but is unaffected if `fundamental` is
    // /// invalid.
    // pub(crate) fn insert_into_molecule_unchecked(
    //     &mut self,
    //     molecule: Molecule,
    //     fundamental: impl Fundamental,
    // ) -> bool {
    //     let mol = self.molecules.get_mut(molecule.into()).unwrap();
    //     mol.members.insert(fundamental.as_fundamental())
    // }

    // /// Adds atoms, pseudoatoms, or bonds from an iterator to a substituent.
    // ///
    // /// This method should only ever be used with fundamentals that do not already
    // /// belong to a substituent.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `substituent` is invalid, but is unaffected if any of the fundamental
    // /// IDs are invalid.
    // #[allow(unused)]
    // pub(crate) fn extend_substituent_unchecked<I, E>(
    //     &mut self,
    //     substituent: Substituent,
    //     fundamentals: I,
    // ) where
    //     I: IntoIterator<Item = E>,
    //     E: Fundamental,
    // {
    //     let sub = self.substituents.get_mut(substituent.into()).unwrap();
    //     sub.members
    //         .extend(fundamentals.into_iter().map(|e| e.as_fundamental()))
    // }

    // /// Adds atoms, pseudoatoms, or bonds from an iterator to a molecule.
    // ///
    // /// This method should only ever be used with fundamentals that do not already
    // /// belong to a molecule.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `molecule` is invalid, but is unaffected if any of the fundamental
    // /// IDs are invalid.
    // #[allow(unused)]
    // pub(crate) fn extend_molecule_unchecked<I, E>(&mut self, molecule: Molecule, fundamentals: I)
    // where
    //     I: IntoIterator<Item = E>,
    //     E: Fundamental,
    // {
    //     let mol = self.molecules.get_mut(molecule.into()).unwrap();
    //     mol.members
    //         .extend(fundamentals.into_iter().map(|e| e.as_fundamental()))
    // }

    // /// Removes an atom, pseudoatom, or bond from a substituent.
    // ///
    // /// Returns whether the fundamental was a member of the substituent.
    // ///
    // /// If the fundamental is an atomlike and is the centre of the substituent,
    // /// the centre is adjusted accordingly; if it is the lone centre, the
    // /// substituent becomes centreless. If it is one of two centres,
    // /// however, the centre remains `SubstituentCentre::Multiple` rather than
    // /// becoming `Single`.
    // ///
    // /// The substituent continues to exist, even if empty, as does the removed
    // /// fundamental.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `substituent` is invalid.
    // pub(crate) fn remove_from_substituent(
    //     &mut self,
    //     substituent: Substituent,
    //     fundamental: impl Fundamental,
    // ) -> bool {
    //     let sub = self.substituents.get_mut(substituent.into()).unwrap();
    //     let fund = fundamental.as_fundamental();
    //     if let Some(index) = sub.members.iter().position(|x| *x == fund) {
    //         sub.members.swap_remove(index);
    //     } else {
    //         return false;
    //     }
    //     // If fundamental is an atomlike, we might be removing the centre of the
    //     // substituent (or one of them).
    //     // Is so, adjust the centres of the substituent accordingly
    //     match &mut sub.centre {
    //         SubstituentCentre::None => (),
    //         SubstituentCentre::Single(atomlike) => {
    //             if atomlike.into_inner() == fundamental.into_inner() {
    //                 // No longer has a centre
    //                 sub.centre = SubstituentCentre::None
    //             }
    //         }
    //         SubstituentCentre::Multiple(atomlikes) => {
    //             if let Some(atomlike) = match Fundamental::to_resolved(fundamental) {
    //                 ResolvedFundamental::Bond(_) => None,
    //                 ResolvedFundamental::Atom(atom) => Some(atom.into()),
    //                 ResolvedFundamental::Pseudoatom(pseudoatom) => Some(pseudoatom.into()),
    //             } && let Some(index) = atomlikes.iter().position(|x| *x == atomlike)
    //             {
    //                 // We want to preserve order
    //                 atomlikes.remove(index);
    //             }
    //         }
    //     }
    //     true
    // }

    // /// Removes an atom, pseudoatom, or bond from a molecule.
    // ///
    // /// Returns whether the fundamental was a member of the molecule.
    // ///
    // /// The molecule continues to exist even if empty, as does the removed
    // /// fundamental.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `molecule` is invalid.
    // pub(crate) fn remove_from_molecule(
    //     &mut self,
    //     molecule: Molecule,
    //     fundamental: impl Fundamental,
    // ) -> bool {
    //     let mol = self.molecules.get_mut(molecule.into()).unwrap();
    //     mol.members.remove(&fundamental.as_fundamental())
    // }

    // /// Empties a substituent by removing all its members, returning an iterator over
    // /// the IDs of the former members.
    // ///
    // /// The substituent and all removed fundamentals continue to exist.
    // ///
    // /// After this operation, the substituent will be centreless.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the substituent is not in the map.
    // pub(crate) fn drain_substituent(
    //     &mut self,
    //     substituent: Substituent,
    // ) -> impl Iterator<Item = AnyFundamental> {
    //     let sub = self
    //         .substituents
    //         .get_mut(substituent.into())
    //         .expect("Caller is required to ensure that the substituent is valid");
    //     sub.centre = SubstituentCentre::None;
    //     sub.members.drain(..)
    // }

    // /// Empties a molecule by removing all its members, returning an iterator over
    // /// the IDs of the former members.
    // ///
    // /// The molecule and all removed fundamentals continue to exist.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the molecule is not in the map.
    // pub(crate) fn drain_molecule(
    //     &mut self,
    //     molecule: Molecule,
    // ) -> impl Iterator<Item = AnyFundamental> {
    //     let mol = self
    //         .molecules
    //         .get_mut(molecule.into())
    //         .expect("Caller is required to ensure that the molecule is valid");
    //     mol.members.drain()
    // }

    // /// Empties a substituent by deleting all its members.
    // ///
    // /// The substituent itself continues to exist, and will be centreless.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the substituent is not in the map.
    // pub(crate) fn clear_substituent(&mut self, substituent: Substituent) {
    //     let sub = self
    //         .substituents
    //         .get_mut(substituent.into())
    //         .expect("Caller is required to ensure that the substituent is valid");
    //     sub.centre = SubstituentCentre::None;
    //     let former_members: Vec<AnyFundamental> = std::mem::take(&mut sub.members);
    //     // It's fine to delete in any order as if something isn't in the map any more
    //     // (e.g. because it's a bond and one of its bonding partners was already deleted
    //     // and thus it too was already deleted) then nothing changes when the deletion
    //     // is attempted
    //     for member in former_members {
    //         match member.resolve() {
    //             ResolvedFundamental::Atom(atom) => self.delete_atom(atom),
    //             ResolvedFundamental::Pseudoatom(pseudoatom) => self.delete_pseudoatom(pseudoatom),
    //             ResolvedFundamental::Bond(bond) => self.delete_bond(bond),
    //         };
    //     }
    // }

    // /// Empties a molecule by deleting all its members.
    // ///
    // /// The molecule itself continues to exist.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the molecule is not in the map.
    // pub(crate) fn clear_molecule(&mut self, molecule: Molecule) {
    //     let mol = self
    //         .molecules
    //         .get_mut(molecule.into())
    //         .expect("Caller is required to ensure that the Molecule is valid");
    //     let former_members: Vec<AnyFundamental> = mol.members.drain().collect();
    //     // It's fine to delete in any order as if something isn't in the map any more
    //     // (e.g. because it's a bond and one of its bonding partners was already deleted
    //     // and thus it too was already deleted) then nothing changes when the deletion
    //     // is attempted
    //     for member in former_members {
    //         match member.resolve() {
    //             ResolvedFundamental::Atom(atom) => self.delete_atom(atom),
    //             ResolvedFundamental::Pseudoatom(pseudoatom) => self.delete_pseudoatom(pseudoatom),
    //             ResolvedFundamental::Bond(bond) => self.delete_bond(bond),
    //         };
    //     }
    // }

    // /// Empties a substituent and then removes it from the map, returning the IDs of the
    // /// former members.
    // ///
    // /// All removed fundamentals continue to exist.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the substituent is not in the map.
    // pub(crate) fn dissolve_substituent(
    //     &mut self,
    //     substituent: Substituent,
    // ) -> impl Iterator<Item = AnyFundamental> {
    //     let sub = self
    //         .substituents
    //         .remove(substituent.into())
    //         .expect("Caller is required to ensure that the Molecule is valid");
    //     sub.members.into_iter()
    // }

    // /// Empties a molecule and then removes it from the map, returning the IDs of the
    // /// former members.
    // ///
    // /// All removed fundamentals continue to exist.
    // ///
    // /// # Panics
    // ///
    // /// Panics if the molecule is not in the map.
    // pub(crate) fn dissolve_molecule(
    //     &mut self,
    //     molecule: Molecule,
    // ) -> impl Iterator<Item = AnyFundamental> {
    //     let mol = self
    //         .molecules
    //         .remove(molecule.into())
    //         .expect("Caller is required to ensure that the Molecule is valid");
    //     mol.members.into_iter()
    // }
}

/// Methods relating to substituent centres.
impl MolMap {
    /// Returns details of the centre(s) of a substituent.
    ///
    /// # Errors
    ///
    /// Fails if the substituent does not exist.
    pub(crate) fn substituent_centres(
        &self,
        substituent: Substituent,
    ) -> Result<SubstituentCentre<'_>, Error> {
        let sub_data = self
            .data(substituent)
            .ok_or(Error::InvalidId(substituent.into()))?;
        let centre = match sub_data.centres {
            0 => SubstituentCentre::None,
            1 => SubstituentCentre::Single(
                sub_data
                    .atomlikes
                    .first()
                    .expect("atomlikes.len() should always be >= centres")
                    .to_owned(),
            ),
            2.. => SubstituentCentre::Multiple(&sub_data.atomlikes[..sub_data.centres]),
        };
        Ok(centre)
    }

    // /// Attempts to change the centre of the substituent to the one requested.
    // ///
    // /// # Errors
    // ///
    // /// Fails if the requested centre is not already a member of the substituent,
    // /// or if there are already bonds to the current centre(s).
    // ///
    // /// # Panics
    // ///
    // /// Panics if `substituent` is invalid.
    // pub(crate) fn set_substituent_centre(
    //     &mut self,
    //     substituent: Substituent,
    //     new_centre: impl Atomlike,
    // ) -> Result<()> {
    //     // First confirm that the new centre is actually a member of the substituent
    //     let sub_data = self
    //         .data(substituent)
    //         .expect("Caller is required to ensure that the molecule is valid");
    //     if !sub_data
    //         .members
    //         .contains(&new_centre.as_atomlike().as_fundamental())
    //     {
    //         return Err(Membership(
    //             new_centre.as_atomlike().as_fundamental(),
    //         ));
    //     }
    //     // A closure that determines if an atom or pseudoatom has bonds already
    //     let atomlike_has_bonds = |id: AnyAtomlike| -> bool {
    //         let bonds = match id.resolve() {
    //             ResolvedAtomlike::Atom(atom) => {
    //                 &self
    //                     .data(atom)
    //                     .expect("Wouldn't be listed as the centre if it had been removed")
    //                     .bonds
    //             }
    //             ResolvedAtomlike::Pseudoatom(pseudoatom) => {
    //                 &self
    //                     .data(pseudoatom)
    //                     .expect("Wouldn't be listed as the centre if it had been removed")
    //                     .bonds
    //             }
    //         };
    //         !bonds.is_empty()
    //     };
    //     // Check that there aren't already bonds to the current centre
    //     let already_bonded = match &sub_data.centre {
    //         SubstituentCentre::None => false,
    //         SubstituentCentre::Single(atomlike_id) => atomlike_has_bonds(*atomlike_id),
    //         SubstituentCentre::Multiple(atomlike_ids) => {
    //             atomlike_ids.iter().copied().any(atomlike_has_bonds)
    //         }
    //     };
    //     if already_bonded {
    //         Err(Disallowed(String::from(
    //             "Substituent already has at least one bond to its centre(s)",
    //         )))
    //     } else {
    //         self.data_mut(substituent)
    //             .expect("Already validated")
    //             .centre = SubstituentCentre::Single(new_centre.as_atomlike());
    //         Ok(())
    //     }
    // }

    // /// Makes the requested atomlike a centre of the substituent, in addition to any
    // /// already existing centres.
    // ///
    // /// # Errors
    // ///
    // /// Fails if the requested centre is not already a member of the substituent.
    // ///
    // /// # Panics
    // ///
    // /// Panics if `substituent` is invalid.
    // pub(crate) fn add_substituent_centre(
    //     &mut self,
    //     substituent: Substituent,
    //     new_centre: impl Atomlike,
    // ) -> Result<()> {
    //     // First confirm that the new centre is actually a member of the substituent
    //     let sub_data = self
    //         .data_mut(substituent)
    //         .expect("Caller is required to ensure that the molecule is valid");
    //     if !sub_data
    //         .members
    //         .contains(&new_centre.as_atomlike().as_fundamental())
    //     {
    //         return Err(Membership(
    //             new_centre.as_atomlike().as_fundamental(),
    //         ));
    //     }
    //     match &mut sub_data.centre {
    //         SubstituentCentre::None => {
    //             self.data_mut(substituent)
    //                 .expect("Already validated")
    //                 .centre = SubstituentCentre::Single(new_centre.as_atomlike())
    //         }
    //         SubstituentCentre::Single(existing_centre) => {
    //             self.data_mut(substituent)
    //                 .expect("Already validated")
    //                 .centre = SubstituentCentre::Multiple(Box::new(vec![
    //                 *existing_centre,
    //                 new_centre.as_atomlike(),
    //             ]))
    //         }
    //         SubstituentCentre::Multiple(existing_centres) => {
    //             existing_centres.push(new_centre.as_atomlike())
    //         }
    //     }
    //     Ok(())
    // }
}

impl<'m> ViewMut<'m, MolMap, Atom> {
    /// Removes the atom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.delete_atom(self.id);
    }
}

impl<'m> ViewMut<'m, MolMap, Pseudoatom> {
    /// Removes the pseudoatom from the map, as well as any bonds to it.
    pub fn delete(self) {
        self.map.delete_pseudoatom(self.id);
    }
}

impl<'m> ViewMut<'m, MolMap, Bond> {
    /// Removes the bond from the map (but not its bonding partners).
    pub fn delete(self) {
        self.map.delete_bond(self.id);
    }
}

impl<'m> ViewMut<'m, MolMap, Substituent> {
    /// Removes the substituent from the map, as well as all of its members.
    pub fn delete(self) {
        self.map.delete_substituent(self.id);
    }

    // /// Adds an atom, pseudoatom, or bond to the substituent.
    // ///
    // /// Returns whether the fundamental was newly inserted.
    // ///
    // /// If the fundamental is already a member of another substituent, it is removed
    // /// from it before it is inserted into this one.
    // ///
    // /// # Errors
    // ///
    // /// Returns an error if the fundamental is invalid.
    // pub fn insert(self, fundamental: impl Fundamental) -> Result<bool> {
    //     if !self.map.contains(fundamental) {
    //         return Err(InvalidId(fundamental.as_entity()));
    //     };
    //     Ok(self
    //         .map
    //         .core_mut()
    //         .insert_into_substituent(self.id, fundamental))
    // }

    // /// Adds atoms, pseudoatoms, or bonds from an iterator to the substituent.
    // ///
    // /// If any of the fundamentals are already members of other substituents, they are
    // /// removed before they are inserted into this one.
    // ///
    // /// # Errors
    // ///
    // /// Returns an error if any of the fundamentals are invalid, in which case the
    // /// map will remain unchanged; the returned error will hold the ID of the first
    // /// invalid fundamental encountered.
    // pub fn extend<E, I>(self, fundamentals: I) -> Result<()>
    // where
    //     E: Entity + Into<AnyFundamental>,
    //     I: IntoIterator<Item = E>,
    //     I::IntoIter: Clone,
    // {
    //     let fundamentals = fundamentals.into_iter();
    //     for f in fundamentals.clone() {
    //         if !self.map.contains(f) {
    //             return Err(InvalidId(f.as_entity()));
    //         }
    //     }
    //     for f in fundamentals {
    //         self.map
    //             .core_mut()
    //             .insert_into_substituent(self.id, f.into());
    //     }
    //     Ok(())
    // }

    // /// Removes an atom, pseudoatom, or bond from the substituent.
    // ///
    // /// Returns whether the fundamental was a member of the substituent.
    // ///
    // /// If the fundamental is an atomlike and is the centre of the substituent,
    // /// the centre is adjusted accordingly; if it is the lone centre, the
    // /// substituent becomes centreless. If it is one of two centres,
    // /// however, the centre remains `SubstituentCentre::Multiple` rather than
    // /// becoming `Single`.
    // ///
    // /// The substituent continues to exist, even if empty, as does the removed
    // /// fundamental.
    // pub fn remove(self, fundamental: impl Fundamental) -> bool {
    //     self.map
    //         .core_mut()
    //         .remove_from_substituent(self.id, fundamental)
    // }

    // /// Empties the substituent by removing all its members, returning an iterator
    // /// over the IDs of the former members.
    // ///
    // /// The substituent and all removed fundamentals continue to exist.
    // ///
    // /// After this operation, the substituent will be centreless.
    // pub fn drain(self) -> impl Iterator<Item = AnyFundamental> {
    //     self.map.core_mut().drain_substituent(self.id)
    // }

    // /// Empties the substituent by deleting all its members.
    // ///
    // /// The substituent itself continues to exist, and will be centreless.
    // pub fn clear(self) {
    //     self.map.core_mut().clear_substituent(self.id);
    // }

    // /// Empties the substituent and then removes it from the map, returning the IDs of
    // /// the former members.
    // ///
    // /// All removed fundamentals continue to exist.
    // pub fn dissolve(self) -> impl Iterator<Item = AnyFundamental> {
    //     self.map.core_mut().dissolve_substituent(self.id)
    // }
}

impl<'m> ViewMut<'m, MolMap, Molecule> {
    /// Removes the molecule from the map, as well as all of its members.
    pub fn delete(self) {
        self.map.delete_molecule(self.id);
    }

    // /// Adds an atom, pseudoatom, or bond to the molecule.
    // ///
    // /// Returns whether the fundamental was newly inserted.
    // ///
    // /// If the fundamental is already a member of another molecule, it is removed
    // /// from it before it is inserted into this one.
    // ///
    // /// # Errors
    // ///
    // /// Returns an error if the fundamental is invalid.
    // pub fn insert(self, fundamental: impl Fundamental) -> Result<bool> {
    //     if !self.map.contains(fundamental) {
    //         return Err(InvalidId(fundamental.as_entity()));
    //     };
    //     Ok(self
    //         .map
    //         .core_mut()
    //         .insert_into_molecule(self.id, fundamental))
    // }

    // /// Adds atoms, pseudoatoms, or bonds from an iterator to the molecule.
    // ///
    // /// If any of the fundamentals are already members of other molecules, they are
    // /// removed before they are inserted into this one.
    // ///
    // /// # Errors
    // ///
    // /// Returns an error if any of the fundamentals are invalid, in which case the
    // /// map will remain unchanged; the returned error will hold the ID of the first
    // /// invalid fundamental encountered.
    // pub fn extend<E, I>(self, fundamentals: I) -> Result<()>
    // where
    //     E: Entity + Into<AnyFundamental>,
    //     I: IntoIterator<Item = E>,
    //     I::IntoIter: Clone,
    // {
    //     let fundamentals = fundamentals.into_iter();
    //     for f in fundamentals.clone() {
    //         if !self.map.contains(f) {
    //             return Err(InvalidId(f.as_entity()));
    //         }
    //     }
    //     for f in fundamentals {
    //         self.map.core_mut().insert_into_molecule(self.id, f.into());
    //     }
    //     Ok(())
    // }

    // /// Removes an atom, pseudoatom, or bond from the molecule.
    // ///
    // /// Returns whether the fundamental was a member of the molecule.
    // ///
    // /// The molecule continues to exist even if empty, as does the removed
    // /// fundamental.
    // pub fn remove(self, fundamental: impl Fundamental) -> bool {
    //     self.map
    //         .core_mut()
    //         .remove_from_molecule(self.id, fundamental)
    // }

    // /// Empties the molecule by removing all its members, returning an iterator
    // /// over the IDs of the former members.
    // ///
    // /// The molecule and all removed fundamentals continue to exist.
    // pub fn drain(self) -> impl Iterator<Item = AnyFundamental> {
    //     self.map.core_mut().drain_molecule(self.id)
    // }

    // /// Empties the molecule by deleting all its members.
    // ///
    // /// The molecule itself continues to exist.
    // pub fn clear(self) {
    //     self.map.core_mut().clear_molecule(self.id);
    // }

    // /// Empties the molecule and then removes it from the map, returning the IDs of the
    // /// former members.
    // ///
    // /// All removed fundamentals continue to exist.
    // pub fn dissolve(self) -> impl Iterator<Item = AnyFundamental> {
    //     self.map.core_mut().dissolve_molecule(self.id)
    // }
}

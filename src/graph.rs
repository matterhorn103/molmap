// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use slotmap::SlotMap;

use crate::{
    BondType, Element,
    entities::{bond::BondingPartner, substituent::SubstituentCentre, *},
    ids::*,
};

/// An arena-like data structure to represent a set of chemical entities,
/// their properties, and the relationships between them, as a molecular graph.
///
/// This forms the core of all `MolMap` types, but is not public.
/// [`MolMap0`] is the `MolMap` type that provides a molecular graph for users.
///
/// In general, the methods of `MolGraph` should be small in scope and efficient
/// so that the higher maps can combine them to create a nice public API.
/// The methods should also do as little checking and validation as possible, with
/// panicking preferred - the higher maps are then responsible for careful usage.
#[derive(Debug, Default)]
pub(crate) struct MolGraph {
    pub(crate) atoms: SlotMap<AtomId, Atom>,
    pub(crate) pseudoatoms: SlotMap<PseudoatomId, Pseudoatom>,
    pub(crate) bonds: SlotMap<BondId, Bond>,
    pub(crate) substituents: SlotMap<SubstituentId, Substituent>,
    pub(crate) molecules: SlotMap<MoleculeId, Molecule>,
}

impl MolGraph {
    pub(crate) fn new() -> Self {
        Self {
            atoms: SlotMap::with_key(),
            pseudoatoms: SlotMap::with_key(),
            bonds: SlotMap::with_key(),
            substituents: SlotMap::with_key(),
            molecules: SlotMap::with_key(),
        }
    }

    pub(crate) fn with_capacity(n: usize) -> Self {
        todo!()
    }
}

impl MolGraph {
    // Methods to iterate over IDs

    /// Returns an iterator over all the IDs of all atoms in the map.
    pub(crate) fn atom_ids(&'_ self) -> impl Iterator<Item = AtomId> + '_ {
        self.atoms.keys()
    }

    /// Returns an iterator over all the IDs of all pseudoatoms in the map.
    pub(crate) fn pseudoatom_ids(&'_ self) -> impl Iterator<Item = PseudoatomId> + '_ {
        self.pseudoatoms.keys()
    }

    /// Returns an iterator over all the IDs of all bonds in the map.
    pub(crate) fn bond_ids(&'_ self) -> impl Iterator<Item = BondId> + '_ {
        self.bonds.keys()
    }

    /// Returns an iterator over all the IDs of all substituents in the map.
    pub(crate) fn substituent_ids(&'_ self) -> impl Iterator<Item = SubstituentId> + '_ {
        self.substituents.keys()
    }

    /// Returns an iterator over all the IDs of all molecules in the map.
    pub(crate) fn molecule_ids(&'_ self) -> impl Iterator<Item = MoleculeId> + '_ {
        self.molecules.keys()
    }

    // Methods to check the validity of IDs and entity enums

    /// Checks if the given ID is valid.
    pub(crate) fn contains_atom(&self, id: AtomId) -> bool {
        self.atoms.contains_key(id)
    }

    /// Checks if the given ID is valid.
    pub(crate) fn contains_pseudoatom(&self, id: PseudoatomId) -> bool {
        self.pseudoatoms.contains_key(id)
    }

    /// Checks if the given ID is valid.
    pub(crate) fn contains_bond(&self, id: BondId) -> bool {
        self.bonds.contains_key(id)
    }

    /// Checks if the given ID is valid.
    pub(crate) fn contains_substituent(&self, id: SubstituentId) -> bool {
        self.substituents.contains_key(id)
    }

    /// Checks if the given ID is valid.
    pub(crate) fn contains_molecule(&self, id: MoleculeId) -> bool {
        self.molecules.contains_key(id)
    }

    /// Checks if the given enum wraps a valid ID.
    pub(crate) fn contains_atomlike(&self, atomlike: AtomlikeId) -> bool {
        match atomlike {
            AtomlikeId::Atom(id) => self.contains_atom(id),
            AtomlikeId::Pseudoatom(id) => self.contains_pseudoatom(id),
        }
    }

    /// Checks if the given enum wraps a valid ID.
    pub(crate) fn contains_fundamental(&self, fundamental: FundamentalId) -> bool {
        match fundamental {
            FundamentalId::Atom(id) => self.contains_atom(id),
            FundamentalId::Pseudoatom(id) => self.contains_pseudoatom(id),
            FundamentalId::Bond(id) => self.contains_bond(id),
        }
    }

    /// Checks if the given enum wraps a valid ID.
    pub(crate) fn contains_bondable(&self, bondable: BondableId) -> bool {
        match bondable {
            BondableId::Atom(id) => self.contains_atom(id),
            BondableId::Pseudoatom(id) => self.contains_pseudoatom(id),
            BondableId::Substituent(id) => self.contains_substituent(id),
        }
    }

    /// Checks if the map contains the entity with the wrapped ID.
    pub(crate) fn contains_entity(&self, entity: EntityId) -> bool {
        match entity {
            EntityId::Atom(id) => self.contains_atom(id),
            EntityId::Pseudoatom(id) => self.contains_pseudoatom(id),
            EntityId::Bond(id) => self.contains_bond(id),
            EntityId::Substituent(id) => self.contains_substituent(id),
            EntityId::Molecule(id) => self.contains_molecule(id),
        }
    }

    // Methods to add entities

    /// Adds an atom to the map.
    pub(crate) fn add_atom(&mut self, element: Element) -> AtomId {
        self.atoms.insert(Atom::new(element))
    }

    /// Adds a pseudoatom to the map.
    pub(crate) fn add_pseudoatom(&mut self, symbol: &str) -> PseudoatomId {
        self.pseudoatoms.insert(Pseudoatom::new(symbol.to_owned()))
    }

    /// Creates a new (single covalent) bond between two bondable entities.
    ///
    /// # Panics
    ///
    /// Panics if either of `start` and `end` are invalid.
    pub(crate) fn add_bond(&mut self, start: BondableId, end: BondableId) -> BondId {
        let start = self.convert_bondable(start);
        let end = self.convert_bondable(end);
        let bond_id = self
            .bonds
            .insert(Bond::new(BondType::Covalent, 1.0, start, end));
        for partner in [start, end] {
            match partner {
                BondingPartner::Atom(id) => self.atoms.get_mut(id).unwrap().bonds.push(bond_id),
                BondingPartner::Pseudoatom(id) => {
                    self.pseudoatoms.get_mut(id).unwrap().bonds.push(bond_id)
                }
                BondingPartner::AmbiguouslyBondingSubstituent(id) => {
                    let SubstituentCentre::Ambiguous(bonds) =
                        &mut self.substituents.get_mut(id).unwrap().centre
                    else {
                        unreachable!("Already know it's ambiguous")
                    };
                    bonds.push(bond_id);
                }
            }
        }
        bond_id
    }

    /// Adds an empty substituent to the map.
    ///
    /// The centre of the new substituent is ambiguous, so if it will have a
    /// centre added to it subsequently, prefer `add_substituent_with_centre()`.
    pub(crate) fn add_substituent(&mut self) -> SubstituentId {
        self.substituents.insert(Substituent {
            centre: SubstituentCentre::Ambiguous(Vec::new()),
            members: Vec::new(),
        })
    }

    /// Adds a substituent to the map with the given atomlike as its centre.
    ///
    /// Note that this method will not fail, even if `centre` is an invalid ID.
    pub(crate) fn add_substituent_with_centre(&mut self, centre: AtomlikeId) -> SubstituentId {
        self.substituents.insert(Substituent {
            centre: SubstituentCentre::Single(centre),
            members: vec![centre.into()],
        })
    }

    /// Adds an empty molecule to the map.
    pub(crate) fn add_molecule(&mut self) -> MoleculeId {
        self.molecules.insert(Molecule::new())
    }

    // Methods to add entities to collections

    /// Adds the atom, pseudoatom, or bond to the substituent.
    ///
    /// This method should only ever be used with fundamentals that do not already
    /// belong to a substituent.
    ///
    /// # Panics
    ///
    /// Panics if `substituent` is invalid, but is unaffected if `fundamental` is
    /// invalid.
    pub(crate) fn add_to_substituent(
        &mut self,
        substituent: SubstituentId,
        fundamental: FundamentalId,
    ) {
        let sub = self.substituents.get_mut(substituent).unwrap();
        sub.members.push(fundamental);
    }

    /// Adds the atom, pseudoatom, or bond to the molecule.
    ///
    /// This method should only ever be used with fundamentals that do not already
    /// belong to a molecule.
    ///
    /// # Panics
    ///
    /// Panics if `molecule` is invalid, but is unaffected if `fundamental` is
    /// invalid.
    pub(crate) fn add_to_molecule(&mut self, molecule: MoleculeId, fundamental: FundamentalId) {
        let mol = self.molecules.get_mut(molecule).unwrap();
        mol.members.push(fundamental);
    }

    // Methods to remove entities from collections

    /// Removes the atom, pseudoatom, or bond from the substituent.
    ///
    /// If the fundamental is an atomlike and is the centre of the substituent,
    /// the centre is adjusted accordingly; if it is the lone centre, the
    /// substituent's centre becomes ambiguous. If it is one of two centres,
    /// however, the centre remains `SubstituentCentre::Multiple` rather than
    /// becoming `Single`.
    ///
    /// The substituent continues to exist even if empty.
    ///
    /// If the fundamental is invalid or not a member of the substituent, this
    /// simply has no effect.
    ///
    /// # Panics
    ///
    /// Panics if `substituent` is invalid.
    pub(crate) fn remove_from_substituent(
        &mut self,
        substituent: SubstituentId,
        fundamental: FundamentalId,
    ) {
        let sub = self.substituents.get_mut(substituent).unwrap();
        if let Some(index) = sub.members.iter().position(|x| *x == fundamental) {
            sub.members.swap_remove(index);
        }
        // If fundamental is an atomlike, we might be removing the centre of the
        // substituent (or one of them).
        // Is so, adjust the centres of the substituent accordingly
        match &mut sub.centre {
            SubstituentCentre::Ambiguous(_) => (),
            SubstituentCentre::Single(atomlike) => {
                if FundamentalId::from(*atomlike) == fundamental {
                    // Becomes an ambiguous centre
                    sub.centre = SubstituentCentre::default()
                }
            }
            SubstituentCentre::Multiple(atomlikes) => {
                if let Some(atomlike) = match fundamental {
                    FundamentalId::Bond(_) => None,
                    FundamentalId::Atom(id) => Some(id.into()),
                    FundamentalId::Pseudoatom(id) => Some(id.into()),
                } && let Some(index) = atomlikes.iter().position(|x| *x == atomlike)
                {
                    // We want to preserve order
                    atomlikes.remove(index);
                }
            }
        }
    }

    /// Removes the atom, pseudoatom, or bond from the molecule.
    ///
    /// If the fundamental is invalid or not a member of the molecule, this simply
    /// has no effect.
    ///
    /// The molecule continues to exist even if empty.
    ///
    /// # Panics
    ///
    /// Panics if `molecule` is invalid.
    pub(crate) fn remove_from_molecule(
        &mut self,
        molecule: MoleculeId,
        fundamental: FundamentalId,
    ) {
        let mol = self.molecules.get_mut(molecule).unwrap();
        if let Some(index) = mol.members.iter().position(|x| *x == fundamental) {
            mol.members.swap_remove(index);
        }
    }

    // Methods to remove entities entirely

    /// Removes an atom from the map, as well as any bonds to it.
    ///
    /// This is infallible – if the atom is not in the map, nothing happens.
    pub(crate) fn remove_atom(&mut self, id: AtomId) {
        if !self.contains_atom(id) {
            return;
        }
        // Make sure we always remove bonds first
        let bonds = self.atoms.get(id).unwrap().bonds.clone();
        for bond_id in bonds {
            self.remove_bond(bond_id);
        }
        // Remove from any collections
        if let Some(frag_id) = self.parent_substituent(id.into()) {
            self.remove_from_substituent(frag_id, id.into())
        }
        if let Some(mol_id) = self.parent_molecule(id.into()) {
            self.remove_from_molecule(mol_id, id.into())
        }
        // Now we can safely remove the atom itself without leaving dangling bonds
        self.atoms.remove(id);
    }

    /// Removes a pseudoatom from the map, as well as any bonds to it.
    ///
    /// This is infallible – if the pseudoatom is not in the map, nothing happens.
    pub(crate) fn remove_pseudoatom(&mut self, id: PseudoatomId) {
        if !self.contains_pseudoatom(id) {
            return;
        }
        // Make sure we always remove bonds first
        let bonds = self.pseudoatoms.get(id).unwrap().bonds.clone();
        for bond_id in bonds {
            self.remove_bond(bond_id);
        }
        // Remove from any collections
        if let Some(frag_id) = self.parent_substituent(id.into()) {
            self.remove_from_substituent(frag_id, id.into())
        }
        if let Some(mol_id) = self.parent_molecule(id.into()) {
            self.remove_from_molecule(mol_id, id.into())
        }
        // Now we can safely remove the pseudoatom itself without leaving dangling bonds
        self.pseudoatoms.remove(id);
    }

    /// Removes a bond from the map (but not its bonding partners).
    ///
    /// If the bond is not in the map, nothing happens.
    ///
    /// # Panics
    ///
    /// Panics if either of the bond's bonding partners does not exist (which
    /// should never be the case – bonds are last in, first out).
    pub(crate) fn remove_bond(&mut self, id: BondId) {
        if let Some(bond) = self.bonds.remove(id) {
            for bonding_partner in [bond.start, bond.end] {
                match bonding_partner {
                    BondingPartner::Atom(atom_id) => {
                        let mut atom = self
                            .atoms
                            .get_mut(atom_id)
                            .expect("Bonds are always removed before their bonding partners");
                        let pos = atom.bonds.iter().position(|x| *x == id).expect(
                            "Bond should be listed in the bonding partner's bonds until deletion",
                        );
                        atom.bonds.remove(pos);
                    }
                    BondingPartner::Pseudoatom(pseudoatom_id) => {
                        let mut pseudoatom = self
                            .pseudoatoms
                            .get_mut(pseudoatom_id)
                            .expect("Bonds are always removed before their bonding partners");
                        let pos = pseudoatom.bonds.iter().position(|x| *x == id).expect(
                            "Bond should be listed in the bonding partner's bonds until deletion",
                        );
                        pseudoatom.bonds.remove(pos);
                    }
                    BondingPartner::AmbiguouslyBondingSubstituent(substituent_id) => todo!(),
                }
            }
            // Remove from any collections
            if let Some(frag_id) = self.parent_substituent(id.into()) {
                self.remove_from_substituent(frag_id, id.into())
            }
            if let Some(mol_id) = self.parent_molecule(id.into()) {
                self.remove_from_molecule(mol_id, id.into())
            }
        }
    }

    /// Removes a substituent from the map, as well as all of its members.
    ///
    /// This is infallible – if the substituent is not in the map, nothing happens.
    pub(crate) fn remove_substituent(&mut self, id: SubstituentId) {
        let members = self.substituents.get(id).unwrap().members.clone();
        for member in members {
            match member {
                FundamentalId::Atom(id) => self.remove_atom(id),
                FundamentalId::Pseudoatom(id) => self.remove_pseudoatom(id),
                FundamentalId::Bond(id) => self.remove_bond(id),
            }
        }
        self.substituents.remove(id);
    }

    /// Removes a molecule from the map, as well as all of its members.
    ///
    /// This is infallible – if the molecule is not in the map, nothing happens.
    pub(crate) fn remove_molecule(&mut self, id: MoleculeId) {
        let members = self.molecules.get(id).unwrap().members.clone();
        for member in members {
            match member {
                FundamentalId::Atom(id) => self.remove_atom(id),
                FundamentalId::Pseudoatom(id) => self.remove_pseudoatom(id),
                FundamentalId::Bond(id) => self.remove_bond(id),
            }
        }
        self.molecules.remove(id);
    }

    /// Gets the actual bonding partner that a `Bondable` refers to.
    ///
    /// Substituents generally form bonds from a central atom or pseudoatom, but they might have no
    /// specified centre, or they might have multiple centres.
    /// In the first case, the bond goes to the substituent as a whole; in the second case the first
    /// centre in `centres` is used for the new bond.
    ///
    /// # Panics
    ///
    /// Panics if `bondable` is an invalid `Substituent`.
    ///
    /// For an `Atom` or `Pseudoatom` the ID is simply wrapped in `BondingPartner`
    /// with no checking of validity, so it will never fail.
    fn convert_bondable(&self, bondable: BondableId) -> BondingPartner {
        match bondable {
            BondableId::Atom(id) => BondingPartner::Atom(id),
            BondableId::Pseudoatom(id) => BondingPartner::Pseudoatom(id),
            BondableId::Substituent(id) => {
                // Get the substituent's data, panics if ID invalid
                let substituent = self.substituents.get(id).unwrap();
                // Use the first centre if any specified, the entire substituent if not
                match &substituent.centre {
                    substituent::SubstituentCentre::Ambiguous(_) => {
                        BondingPartner::AmbiguouslyBondingSubstituent(id)
                    }
                    substituent::SubstituentCentre::Single(centre) => (*centre).into(),
                    substituent::SubstituentCentre::Multiple(centres) => (*centres
                        .first()
                        .expect("Will always have at least one centre"))
                    .into(),
                }
            }
        }
    }

    /// Determines the substituent that contains the atom, pseudoatom, or bond, if any.
    fn parent_substituent(&self, fundamental: FundamentalId) -> Option<SubstituentId> {
        for (substituent_id, substituent) in self.substituents.iter() {
            if substituent.members.contains(&fundamental) {
                return Some(substituent_id);
            }
        }
        None
    }

    /// Determines the molecule that contains the atom, pseudoatom, or bond, if any.
    fn parent_molecule(&self, fundamental: FundamentalId) -> Option<MoleculeId> {
        for (mol_id, mol) in self.molecules.iter() {
            if mol.members.contains(&fundamental) {
                return Some(mol_id);
            }
        }
        None
    }
}

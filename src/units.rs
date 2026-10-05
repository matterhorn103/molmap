// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nalgebra as na;

/// A unit of measure.
pub trait Unit {
    /// The proportionality factor of the value of the unit when expressed in metres.
    const FACTOR: f64;

    /// Expresses the unit in terms of the requested unit.
    fn in_terms_of<U: Unit>(other: U) -> f64 {
        Self::FACTOR / U::FACTOR
    }

    /// Converts a quantity with this unit and the provided number to a quantity in
    /// terms of the requested unit and returns its number.
    fn convert_to<U: Unit>(number: f64) -> f64 {
        (number * Self::FACTOR) / U::FACTOR
    }

    /// Converts a quantity with the provided number and unit to a quantity in
    /// terms of this unit and returns its number.
    fn convert_from<U: Unit>(number: f64) -> f64 {
        (number * U::FACTOR) / Self::FACTOR
    }
}

/// The meter (m), the SI unit of length.
pub struct Metre;

impl Unit for Metre {
    const FACTOR: f64 = 1.0;
}

pub type Meter = Metre;

/// The angstrom (Å), equal to 1 × 10<sup>−10</sup> m.
pub struct Angstrom;

impl Unit for Angstrom {
    const FACTOR: f64 = 1e-10;
}

/// The nanometre (nm), equal to 1 × 10<sup>−9</sup> m.
pub struct Nanometre;

impl Unit for Nanometre {
    const FACTOR: f64 = 1e-9;
}

pub type Nanometer = Nanometre;

/// The picometre (pm), equal to 1 × 10<sup>−12</sup> m.
pub struct Picometre;

impl Unit for Picometre {
    const FACTOR: f64 = 1e-10;
}

pub type Picometer = Picometre;

/// `molmap`'s "standard bond length", equivalent to 1.5 Å (the rough length of a
/// carbon-carbon single covalent bond).
pub struct StdBond;

impl Unit for StdBond {
    const FACTOR: f64 = 1.5e-10;
}

/// A length unit relative to the approximate length of a carbon-carbon single
/// covalent bond (1.5 Å).
pub type Relative = StdBond;

/// The Bohr radius (_a_<sub>0</sub>).
///
/// The value used is the 2022 CODATA recommended value of 5.291 772 105 44 × 10<sup>−11</sup> m.
pub struct BohrRadius;

impl Unit for BohrRadius {
    const FACTOR: f64 = 5.29177210544e-11;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_terms_of() {
        assert_eq!(Angstrom::in_terms_of(Metre), 1e-10);
        assert_eq!(Angstrom::in_terms_of(Nanometre), 0.1);
        assert_eq!(Angstrom::in_terms_of(Picometre), 100.0);
    }

    fn convert() {
        assert_eq!(Angstrom::convert_to::<Nanometre>(3.21), 0.321);
        assert_eq!(Nanometre::convert_to::<Picometre>(3.21), 3210.0);
        assert_eq!(Angstrom::convert_from::<Nanometre>(3.21), 32.1);
        assert_eq!(Nanometre::convert_from::<Picometre>(3.21), 0.00321);
    }
}

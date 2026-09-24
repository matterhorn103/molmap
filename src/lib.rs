// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(unused)]

// Private modules
// ---------------
mod element;
mod pseudoelement;
pub(crate) mod traits;

// ----------
// Public API
// ----------

// Publicly accessible modules
// ---------------------------
pub mod atomic;
//pub mod crystalline;
pub mod entities;
pub mod error;
pub mod graph;
pub mod molecular;
pub mod parse;
//pub mod reaction;
pub mod view;

// Top-level items
// ---------------
pub use atomic::entities::BondType;
pub use element::Element;
//pub use molecular::{MolMap, MolMap0, MolMap2, MolMap3};
pub use pseudoelement::Pseudoelement;
pub use traits::{Map, Spatial, ThreeDimensional, TwoDimensional};

// Foreign re-exports
// ------------------
// Foreign crates or things from them
// Re-exporting nalgebra makes it easier for others to use
pub use nalgebra;
pub use nalgebra::{Point2, Point3, Vector2, Vector3};

// SPDX-FileCopyrightText: 2026 Matthew Milner <matterhorn103@proton.me>
//
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::AtomGraph;

pub trait AtomMap {
    /// Returns the core graph.
    fn graph(&self) -> &AtomGraph;

    /// Returns the core graph in mutable form.
    fn graph_mut(&mut self) -> &mut AtomGraph;
}

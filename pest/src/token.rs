// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use position::Position;

/// An `enum` representing tokens generated by a `Parser`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Token<'i, R> {
    /// The starting bit of the `Token`
    Start {
        rule: R,
        pos: Position<'i>
    },
    /// The ending bit of the `Token`
    End {
        rule: R,
        pos: Position<'i>
    }
}

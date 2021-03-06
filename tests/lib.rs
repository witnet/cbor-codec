// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate cbor;
extern crate quickcheck;
#[macro_use]
extern crate json;
extern crate rand;
extern crate rustc_serialize;

mod properties;
mod unit;
mod util;

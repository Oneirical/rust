//@ edition:2018
//@ aux-build:aux-52489.rs
//@ compile-flags:--extern issue_52489

use issue_52489;
//~^ ERROR use of unstable library feature `issue_52489_unstable`

fn main() {}
// https://github.com/rust-lang/rust/issues/52489

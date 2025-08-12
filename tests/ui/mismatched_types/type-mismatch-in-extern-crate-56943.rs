//@ aux-build:aux-56943.rs

extern crate issue_56943;

fn main() {
    let _: issue_56943::S = issue_56943::S2;
    //~^ ERROR mismatched types [E0308]
}
// https://github.com/rust-lang/rust/issues/56943

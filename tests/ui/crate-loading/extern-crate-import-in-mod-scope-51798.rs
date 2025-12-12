//@ edition:2018
//@ aux-build:aux-51798.rs
//@ check-pass

extern crate issue_51798;

mod server {
    fn f() {
        let mut v = issue_51798::vec();
        v.clear();
    }
}

fn main() {}
// https://github.com/rust-lang/rust/issues/51798

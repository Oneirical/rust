//@ run-pass

struct A(#[allow(dead_code)] bool);

pub fn main() {
    let f = A;
    f(true);
}
// https://github.com/rust-lang/rust/issues/5315

//@ build-pass

#![crate_type = "lib"]
#![allow(unconditional_panic)]
struct S(u8);

pub fn ice() {
    S([][0]);
}
// https://github.com/rust-lang/rust/issues/53275

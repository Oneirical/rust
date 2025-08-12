//@ run-pass

mod m {
    pub fn r#for() {}
}

fn main() {
    m::r#for();
}
// https://github.com/rust-lang/rust/issues/57198-

//@ build-pass

fn main() {
    println!("{}", [(); usize::MAX].len());
}
// https://github.com/rust-lang/rust/issues/68010-

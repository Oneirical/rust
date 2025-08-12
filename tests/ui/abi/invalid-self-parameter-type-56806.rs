pub trait Trait {
    fn dyn_instead_of_self(self: Box<dyn Trait>);
    //~^ ERROR invalid `self` parameter type
}

pub fn main() {}
// https://github.com/rust-lang/rust/issues/56806

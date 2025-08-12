// Issue 69130: character indexing bug in rustc_errors::CodeSuggestion::splice_lines().

enum F {
M (§& u8)}
//~^ ERROR unknown start of token
//~| ERROR missing lifetime specifier
fn main() {}
// https://github.com/rust-lang/rust/issues/69130

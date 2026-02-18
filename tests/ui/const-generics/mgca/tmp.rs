//! Regression test for <https://github.com/rust-lang/rust/issues/152653>
#![expect(incomplete_features)]
#![feature(min_generic_const_args)]
type const CONST: usize = 1i32;
//~^ ERROR the constant `1` is not of type `usize`
//~| ERROR mismatched types

fn uses_const() {
    CONST;
}

fn main() {}

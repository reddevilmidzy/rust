//! Regression test for <https://github.com/rust-lang/rust/issues/94846>
//!                     <https://github.com/rust-lang/rust/issues/106473>
//!                     <https://github.com/rust-lang/rust/issues/136416>
#![expect(incomplete_features)]
#![feature(generic_const_exprs)]

const DEFAULT: u32 = 1;

struct V<const U: usize = DEFAULT> //~ ERROR: mismatched types
where
    [(); U]:;


struct S<const C: () = {}>()
where
    S<{}>:;
    //~^ ERROR: overflow evaluating the requirement `S<{}> well-formed`


struct State<const S: usize = {}> //~ ERROR: mismatched types
where
    [(); S]:;

struct Foo;
struct State2<const S: usize = Foo> //~ ERROR: mismatched types
where
    [(); S]:;

fn main() {}

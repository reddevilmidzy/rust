//! Regression test for <https://github.com/rust-lang/rust/issues/116554>
#![expect(incomplete_features)]
#![feature(generic_const_exprs)]

const fn t<const N: usize>() -> u8 {
    N as u8
}

#[repr(u8)]
enum T<const N: u8 = { T::<0>::A as u8 + T::<0>::B as u8 }>
//~^ ERROR: no variant or associated item named `B` found for enum `T<N>` in the current scope
where
    [(); N as usize]:,
    T: ?Sized, //~ ERROR: this relaxed bound is not permitted here
{
    A,
}

fn main() {
    A = t::<N>() as u8,
    //~^ ERROR: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `,`
    B,
}

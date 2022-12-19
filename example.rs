//! Example of use of `grouptrait`.
// Bart Massey 2022

use grouptrait::*;

/// Demonstrate basic usage via addition.
fn eg_add<'a, T>(a: &'a T, b: &'a T) -> T
where
    &'a T: Group<&'a T, T>,
{
    a + b
}

/// Demonstrate use of `zero()` via addition. Should probably
/// provide a macro in the crate or something.
fn eg_id<T>(a: &T) -> T
where
    for<'a> &'a T: Group<&'a T, T>,
{
    &<&T>::gzero() + a
}

/// Demonstration that lifetimes work as expected.  Would be
/// better with a non-copy type.
fn eg_add_borrow() -> i8 {
    let two = 2i8;
    eg_id(&two)
}

fn main() {
    let three: i8 = eg_add(&1i8, &2i8);
    println!("{three}");
    let one: i8 = eg_id(&1i8);
    println!("{one}");
    let two: i8 = eg_add_borrow();
    println!("{two}");
}

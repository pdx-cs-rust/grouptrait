//! Demo of using some traits with standard operators to do a thing.
// Bart Massey 2022

use std::ops::*;

use num_traits::Zero;

/// Trait to provide a zero at an output type while having
/// `Self` a different type.
pub trait GZero<Output = Self> {
    type Output;

    /// Return a zero. The rest of the methods from
    /// `num_trait::Zero` should probably also be provided.
    fn gzero() -> Self::Output;
}

/// Trait to provide some arithmetic operations.
pub trait Group<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output> + Neg<Output = Output> + GZero<Output = Output> + Sized
{
}

/// Implementation for getting a zero of base type out of a
/// reference type.
impl<T> GZero for &'_ T
where
    T: Zero,
{
    type Output = T;

    fn gzero() -> Self::Output {
        Zero::zero()
    }
}

/// Implementation for doing arithmetic `&T` × `&T` → `T`.
impl<'b, T> Group<&'b T, T> for &'_ T where
    Self: Add<&'b T, Output = T> + Neg<Output = T> + GZero<Output = T>
{
}

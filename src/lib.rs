use std::{
    iter,
    ops::{
        AddAssign,
        DivAssign,
        MulAssign,
        RemAssign,
        SubAssign,
    },
};

/// Floating point number abstraction
pub trait Float:
    num::Float
    + Sync
    + Send
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
    + iter::Sum
{
}

impl Float for f32 {}
impl Float for f64 {}

mod qubit;
pub use qubit::{
    Bit,
    Qubit,
};

mod system;
pub use system::System;

mod tensor_iter;
pub use tensor_iter::TensorIter;

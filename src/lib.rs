use std::ops::{
    AddAssign,
    DivAssign,
    MulAssign,
    RemAssign,
    SubAssign,
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

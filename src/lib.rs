use std::ops::{
    AddAssign,
    DivAssign,
    MulAssign,
    RemAssign,
    SubAssign,
};

use num::Float;

pub trait QFloat:
    Float
    + Sync
    + Send
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>
{
}

impl QFloat for f32 {}
impl QFloat for f64 {}

mod qubit;
pub use qubit::{
    Bit,
    Qubit,
};

mod qureg;
pub use qureg::Qureg;

use std::{
    f64::consts::SQRT_2,
    num::NonZeroU16,
    thread,
};

use num::{
    Complex,
    Zero,
};
use qn::*;

trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&'_ mut self) -> Option<Self::Item<'_>>;
}

struct BellPairs {
    stm: System<f64>,
}

impl BellPairs {
    fn new(seed: u64) -> Self {
        let num_qubits = NonZeroU16::new(2).unwrap();
        let stm = System::new(num_qubits, seed);

        Self {
            stm,
        }
    }
}

impl LendingIterator for BellPairs {
    type Item<'a> = (Qubit<'a, f64>, Qubit<'a, f64>)
    where
        Self: 'a;

    fn next(&'_ mut self) -> Option<Self::Item<'_>> {
        for amp in self.stm.as_mut_slice() {
            *amp = Complex::zero();
        }

        self.stm.as_mut_slice()[0] = Complex::from(SQRT_2.recip());
        self.stm.as_mut_slice()[3] = Complex::from(SQRT_2.recip());

        Some(self.stm.qubit_pair(0, 1).unwrap())
    }
}

fn main() {
    let mut bell_pairs = BellPairs::new(123);

    let mut outcome1 = None;
    let mut outcome2 = None;

    let (mut qb1, mut qb2) = bell_pairs.next().unwrap();
    thread::scope(|s| {
        s.spawn(|| {
            outcome1 = Some(qb1.measure());
        });

        outcome2 = Some(qb2.measure());
    });

    assert_eq!(outcome1, outcome2);
}

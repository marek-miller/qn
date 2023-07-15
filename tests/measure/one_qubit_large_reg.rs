use std::f64::consts::SQRT_2;

use num::{
    Complex,
    Zero,
};
use qn::Bit;

use crate::measure::gen_qureg;

#[test]
fn zero_state() {
    let mut qureg = gen_qureg(10, 12);
    for i in 0..10 {
        let mut qubit = qureg.qubit(i).unwrap();
        assert_eq!(qubit.measure(), Bit::ZERO);
    }
}

#[test]
fn one_state() {
    for i in 0..10 {
        let mut qureg = gen_qureg(10, 123);
        qureg.as_mut_slice()[1 << i] = Complex::from(1.);
        let mut qubit = qureg.qubit(i).unwrap();
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

#[test]
fn state_persistent() {
    for i in 0..10 {
        let mut qureg = gen_qureg(10, 34983);
        for _ in 0..100 {
            qureg.as_mut_slice()[0] = Complex::from(SQRT_2.recip());
            qureg.as_mut_slice()[1 << i] = Complex::from(SQRT_2.recip());
            let mut qubit = qureg.qubit(i).unwrap();

            let outcome = qubit.measure();
            assert_eq!(qubit.measure(), outcome);
            assert_eq!(qubit.measure(), outcome);
        }
    }
}

#[test]
fn binary_state() {
    const SIZE: u16 = 10;

    let mut qureg = gen_qureg(SIZE, 349812);

    for i in 0..1 << SIZE {
        (0..1 << SIZE).for_each(|j| qureg.as_mut_slice()[j] = Complex::zero());
        qureg.as_mut_slice()[i] = Complex::from(1.);

        for k in 0..SIZE {
            let outcome = qureg.qubit(k).unwrap().measure();
            if (i >> k & 1) == 0 {
                assert_eq!(outcome, Bit::ZERO);
            } else {
                assert_eq!(outcome, Bit::ONE);
            }
        }
    }
}

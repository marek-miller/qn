use std::f64::consts::SQRT_2;

use num::{
    Complex,
    Zero,
};
use qn::Bit;

use crate::measure::gen_stm;

#[test]
fn zero_state() {
    let mut stm = gen_stm(10, 12);
    for i in 0..10 {
        let mut qubit = stm.qubit(i).unwrap();
        assert_eq!(qubit.measure(), Bit::ZERO);
    }
}

#[test]
fn one_state() {
    for i in 0..10 {
        let mut stm = gen_stm(10, 123);
        stm.as_mut_slice()[0] = Complex::zero();
        stm.as_mut_slice()[1 << i] = Complex::from(1.);
        let mut qubit = stm.qubit(i).unwrap();
        println!("{i}");
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

#[test]
fn state_persistent() {
    for i in 0..10 {
        let mut stm = gen_stm(10, 34983);
        for _ in 0..100 {
            stm.as_mut_slice()[0] = Complex::from(SQRT_2.recip());
            stm.as_mut_slice()[1 << i] = Complex::from(SQRT_2.recip());
            let mut qubit = stm.qubit(i).unwrap();

            let outcome = qubit.measure();
            assert_eq!(qubit.measure(), outcome);
            assert_eq!(qubit.measure(), outcome);
        }
    }
}

#[test]
fn binary_state() {
    const SIZE: u16 = 8;

    let mut stm = gen_stm(SIZE, 349_812);

    for i in 0..1 << SIZE {
        (0..1 << SIZE).for_each(|j| stm.as_mut_slice()[j] = Complex::zero());
        stm.as_mut_slice()[i] = Complex::from(1.);

        for k in 0..SIZE {
            let outcome = stm.qubit(k).unwrap().measure();
            if (i >> k & 1) == 0 {
                assert_eq!(outcome, Bit::ZERO);
            } else {
                assert_eq!(outcome, Bit::ONE);
            }
        }
    }
}

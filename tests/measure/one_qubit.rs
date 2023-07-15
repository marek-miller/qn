use std::f64::consts::SQRT_2;

use qn::Bit;

use crate::measure::{
    gen_stm,
    stm_set_real,
};

#[test]
fn zero_state() {
    let mut stm = gen_stm(1, 12);
    let mut qubit = stm.qubit(0).unwrap();
    assert_eq!(qubit.measure(), Bit::ZERO);
}

#[test]
fn one_state() {
    let mut stm = gen_stm(1, 123);
    stm_set_real(&mut stm, &[0., 1.]);
    let mut qubit = stm.qubit(0).unwrap();
    assert_eq!(qubit.measure(), Bit::ONE);
}

#[test]
fn m100_zero_state() {
    let mut stm = gen_stm(1, 111);
    let mut qubit = stm.qubit(0).unwrap();

    for _ in 0..100 {
        assert_eq!(qubit.measure(), Bit::ZERO);
    }
}

#[test]
fn m100_one_state() {
    let mut stm = gen_stm(1, 1231);
    stm_set_real(&mut stm, &[0., 1.]);
    let mut qubit = stm.qubit(0).unwrap();

    for _ in 0..100 {
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

#[test]
fn m100_alternate() {
    let mut stm = gen_stm(1, 1234);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[1., 0.]);
        let mut qubit = stm.qubit(0).unwrap();
        assert_eq!(qubit.measure(), Bit::ZERO);

        stm_set_real(&mut stm, &[0., 1.]);
        let mut qubit = stm.qubit(0).unwrap();
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

#[test]
fn state_persistent() {
    let mut stm = gen_stm(1, 34983);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[SQRT_2.recip(), SQRT_2.recip()]);
        let mut qubit = stm.qubit(0).unwrap();

        let outcome = qubit.measure();
        assert_eq!(qubit.measure(), outcome);
        assert_eq!(qubit.measure(), outcome);
    }
}

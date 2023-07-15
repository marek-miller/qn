use qn::Bit;

use crate::measure::{
    gen_qureg,
    qureg_set_real,
};

#[test]
fn zero_state() {
    let mut qureg = gen_qureg(1, 12);
    let mut qubit = qureg.qubit(0).unwrap();
    assert_eq!(qubit.measure(), Bit::ZERO);
}

#[test]
fn one_state() {
    let mut qureg = gen_qureg(1, 123);
    qureg_set_real(&mut qureg, &[0., 1.]);
    let mut qubit = qureg.qubit(0).unwrap();
    assert_eq!(qubit.measure(), Bit::ONE);
}

#[test]
fn m100_zero_state() {
    let mut qureg = gen_qureg(1, 111);
    let mut qubit = qureg.qubit(0).unwrap();

    for _ in 0..100 {
        assert_eq!(qubit.measure(), Bit::ZERO);
    }
}

#[test]
fn m100_one_state() {
    let mut qureg = gen_qureg(1, 1231);
    qureg_set_real(&mut qureg, &[0., 1.]);
    let mut qubit = qureg.qubit(0).unwrap();

    for _ in 0..100 {
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

#[test]
fn m100_alternate() {
    let mut qureg = gen_qureg(1, 1234);

    for _ in 0..100 {
        qureg_set_real(&mut qureg, &[1., 0.]);
        let mut qubit = qureg.qubit(0).unwrap();
        assert_eq!(qubit.measure(), Bit::ZERO);

        qureg_set_real(&mut qureg, &[0., 1.]);
        let mut qubit = qureg.qubit(0).unwrap();
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

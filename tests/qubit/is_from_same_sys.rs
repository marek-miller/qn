use std::{
    num::NonZeroU16,
    thread,
};

use qn::System;

#[test]
fn is_from_same_stm_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qb = stm.qubit_pair(0, 1).unwrap();
    assert!(qb.0.is_from_same_stm(&qb.0));
    assert!(qb.0.is_from_same_stm(&qb.1));
    assert!(qb.1.is_from_same_stm(&qb.0));
    assert!(qb.1.is_from_same_stm(&qb.1));
}

#[test]
fn is_from_same_stm_02() {
    let num_qubits = NonZeroU16::new(2).unwrap();

    let mut stm = System::<f32>::new(num_qubits, 1);
    let mut other_stm = System::<f32>::new(num_qubits, 1);
    let qb = stm.qubit_pair(0, 1).unwrap();
    let other_qb = other_stm.qubit_pair(0, 1).unwrap();

    assert!(!qb.0.is_from_same_stm(&other_qb.0));
    assert!(!qb.0.is_from_same_stm(&other_qb.1));
    assert!(!qb.1.is_from_same_stm(&other_qb.0));
    assert!(!qb.0.is_from_same_stm(&other_qb.0));
}

#[test]
fn is_from_same_stm_03() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qb = stm.qubit_pair(0, 1).unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            assert!(qb.0.is_from_same_stm(&qb.0));
            assert!(qb.0.is_from_same_stm(&qb.1));
            assert!(qb.1.is_from_same_stm(&qb.0));
            assert!(qb.1.is_from_same_stm(&qb.1));
        });
        assert!(qb.0.is_from_same_stm(&qb.0));
        assert!(qb.0.is_from_same_stm(&qb.1));
        assert!(qb.1.is_from_same_stm(&qb.0));
        assert!(qb.1.is_from_same_stm(&qb.1));
    });
}

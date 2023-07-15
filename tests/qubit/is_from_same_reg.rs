use std::{
    num::NonZeroU16,
    thread,
};

use qn::{
    Qubit,
    Register,
};

#[test]
fn is_from_same_qureg_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Register::<f32>::new(num_qubits, 1);

    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();
    assert!(qb.0.is_from_same_reg(&qb.0));
    assert!(qb.0.is_from_same_reg(&qb.1));
    assert!(qb.1.is_from_same_reg(&qb.0));
    assert!(qb.1.is_from_same_reg(&qb.1));
}

#[test]
fn is_from_same_qureg_02() {
    let num_qubits = NonZeroU16::new(2).unwrap();

    let mut qureg = Register::<f32>::new(num_qubits, 1);
    let mut other_qureg = Register::<f32>::new(num_qubits, 1);
    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();
    let other_qb = Qubit::new_pair(&mut other_qureg, 0, 1).unwrap();

    assert!(!qb.0.is_from_same_reg(&other_qb.0));
    assert!(!qb.0.is_from_same_reg(&other_qb.1));
    assert!(!qb.1.is_from_same_reg(&other_qb.0));
    assert!(!qb.0.is_from_same_reg(&other_qb.0));
}

#[test]
fn is_from_same_qureg_03() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Register::<f32>::new(num_qubits, 1);

    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            assert!(qb.0.is_from_same_reg(&qb.0));
            assert!(qb.0.is_from_same_reg(&qb.1));
            assert!(qb.1.is_from_same_reg(&qb.0));
            assert!(qb.1.is_from_same_reg(&qb.1));
        });
        assert!(qb.0.is_from_same_reg(&qb.0));
        assert!(qb.0.is_from_same_reg(&qb.1));
        assert!(qb.1.is_from_same_reg(&qb.0));
        assert!(qb.1.is_from_same_reg(&qb.1));
    });
}

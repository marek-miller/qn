use std::{
    num::NonZeroU16,
    thread,
};

use qn::{
    Qubit,
    Qureg,
};

#[test]
fn init_01() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let qubit = Qubit::new(&mut qureg, 0);
    assert!(qubit.is_some());

    let qubit = Qubit::new(&mut qureg, 1);
    assert!(qubit.is_none());
}

#[test]
fn init_02() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let qubit = Qubit::new(&mut qureg, 0);
    assert!(qubit.is_some());

    let qubit = Qubit::new(&mut qureg, 1);
    assert!(qubit.is_some());

    let qubit = Qubit::new(&mut qureg, 2);
    assert!(qubit.is_none());
}

#[test]
fn new_pair_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let qb = Qubit::new_pair(&mut qureg, 0, 1);
    assert!(qb.is_some());

    let qb = Qubit::new_pair(&mut qureg, 1, 0);
    assert!(qb.is_some());

    let qb = Qubit::new_pair(&mut qureg, 0, 0);
    assert!(qb.is_none());

    let qb = Qubit::new_pair(&mut qureg, 1, 1);
    assert!(qb.is_none());

    let qb = Qubit::new_pair(&mut qureg, 0, 2);
    assert!(qb.is_none());

    let qb = Qubit::new_pair(&mut qureg, 2, 0);
    assert!(qb.is_none());
}

#[test]
fn get_index_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();
    assert_eq!(qb.0.index(), 0);
    assert_eq!(qb.1.index(), 1);

    let qb = Qubit::new_pair(&mut qureg, 1, 0).unwrap();
    assert_eq!(qb.0.index(), 1);
    assert_eq!(qb.1.index(), 0);
}

#[test]
fn is_from_same_qureg_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();
    assert!(qb.0.is_from_same_qureg(&qb.0));
    assert!(qb.0.is_from_same_qureg(&qb.1));
    assert!(qb.1.is_from_same_qureg(&qb.0));
    assert!(qb.1.is_from_same_qureg(&qb.1));
}

#[test]
fn is_from_same_qureg_02() {
    let num_qubits = NonZeroU16::new(2).unwrap();

    let mut qureg = Qureg::<f32>::new(num_qubits, 1);
    let mut other_qureg = Qureg::<f32>::new(num_qubits, 1);
    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();
    let other_qb = Qubit::new_pair(&mut other_qureg, 0, 1).unwrap();

    assert!(!qb.0.is_from_same_qureg(&other_qb.0));
    assert!(!qb.0.is_from_same_qureg(&other_qb.1));
    assert!(!qb.1.is_from_same_qureg(&other_qb.0));
    assert!(!qb.0.is_from_same_qureg(&other_qb.0));
}

#[test]
fn is_from_same_qureg_03() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let qb = Qubit::new_pair(&mut qureg, 0, 1).unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            assert!(qb.0.is_from_same_qureg(&qb.0));
            assert!(qb.0.is_from_same_qureg(&qb.1));
            assert!(qb.1.is_from_same_qureg(&qb.0));
            assert!(qb.1.is_from_same_qureg(&qb.1));
        });
        assert!(qb.0.is_from_same_qureg(&qb.0));
        assert!(qb.0.is_from_same_qureg(&qb.1));
        assert!(qb.1.is_from_same_qureg(&qb.0));
        assert!(qb.1.is_from_same_qureg(&qb.1));
    });
}

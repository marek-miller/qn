use std::num::NonZeroU16;

use qn::{
    Qubit,
    System,
};

#[test]
fn init_01() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qubit = Qubit::new(&mut stm, 0);
    assert!(qubit.is_some());

    let qubit = Qubit::new(&mut stm, 1);
    assert!(qubit.is_none());
}

#[test]
fn init_02() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qubit = Qubit::new(&mut stm, 0);
    assert!(qubit.is_some());

    let qubit = Qubit::new(&mut stm, 1);
    assert!(qubit.is_some());

    let qubit = Qubit::new(&mut stm, 2);
    assert!(qubit.is_none());
}

#[test]
fn new_pair_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qb = stm.qubit_pair(0, 1);
    assert!(qb.is_some());

    let qb = stm.qubit_pair(1, 0);
    assert!(qb.is_some());

    let qb = stm.qubit_pair(0, 0);
    assert!(qb.is_none());

    let qb = stm.qubit_pair(1, 1);
    assert!(qb.is_none());

    let qb = stm.qubit_pair(0, 2);
    assert!(qb.is_none());

    let qb = stm.qubit_pair(2, 0);
    assert!(qb.is_none());
}

#[test]
fn get_index_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qb = stm.qubit_pair(0, 1).unwrap();
    assert_eq!(qb.0.index(), 0);
    assert_eq!(qb.1.index(), 1);

    let qb = stm.qubit_pair(1, 0).unwrap();
    assert_eq!(qb.0.index(), 1);
    assert_eq!(qb.1.index(), 0);
}

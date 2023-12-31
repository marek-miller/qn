use std::{
    num::NonZeroU16,
    thread,
};

use num::{
    Complex,
    Zero,
};
use qn::{
    Bit,
    System,
};

#[test]
fn get_qubit_01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qubit = stm.qubit(0);
    assert!(qubit.is_some());

    let qubit = stm.qubit(1);
    assert!(qubit.is_some());

    let qubit = stm.qubit(2);
    assert!(qubit.is_none());
}

#[test]
fn get_qubit_pair01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qb = stm.qubit_pair(0, 1);
    assert!(qb.is_some());

    let qb = stm.qubit_pair(0, 0);
    assert!(qb.is_none());

    let qb = stm.qubit_pair(0, 2);
    assert!(qb.is_none());
}

#[test]
fn get_qubit_iter01() {
    const SIZE: u16 = 5;
    let num_qubits = NonZeroU16::try_from(SIZE).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    for mut qb in stm.qubit_iter() {
        assert_eq!(qb.measure(), Bit::ZERO);
    }

    stm.as_mut_slice()[0] = Complex::zero();
    stm.as_mut_slice()[(1 << SIZE) - 1] = Complex::from(1.);

    for mut qb in stm.qubit_iter() {
        assert_eq!(qb.measure(), Bit::ONE);
    }
}

#[test]
fn get_qubit_iter_collect() {
    const SIZE: u16 = 5;
    let num_qubits = NonZeroU16::try_from(SIZE).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qubits = stm.qubit_iter().collect::<Vec<_>>();
    for mut qb in qubits {
        assert_eq!(qb.measure(), Bit::ZERO);
    }

    stm.as_mut_slice()[0] = Complex::zero();
    stm.as_mut_slice()[(1 << SIZE) - 1] = Complex::from(1.);

    let qubits = stm.qubit_iter().collect::<Vec<_>>();
    for mut qb in qubits {
        assert_eq!(qb.measure(), Bit::ONE);
    }
}

#[test]
fn get_qubit_iter_collect_nonlocal() {
    const SIZE: u16 = 8;
    let num_qubits = NonZeroU16::try_from(SIZE).unwrap();
    let mut stm = System::<f32>::new(num_qubits, 1);

    let qubits = stm.qubit_iter().collect::<Vec<_>>();
    for mut qb in qubits {
        assert_eq!(qb.measure(), Bit::ZERO);
    }

    stm.as_mut_slice()[0] = Complex::zero();
    stm.as_mut_slice()[(1 << SIZE) - 1] = Complex::from(1.);

    let mut qubits = stm.qubit_iter().collect::<Vec<_>>();
    let (qubits_l, qubits_r) = qubits.split_at_mut(5);

    thread::scope(|s| {
        s.spawn(|| {
            for qb in qubits_l {
                assert_eq!(qb.measure(), Bit::ONE);
            }
        });
        for qb in qubits_r {
            assert_eq!(qb.measure(), Bit::ONE);
        }
    });
}

use std::num::NonZeroU16;

use num::Complex;
use qn::{
    Bit,
    Qureg,
};

#[test]
fn zero_state() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let mut qubit = qureg.qubit(0).unwrap();
    assert_eq!(qubit.measure(), Bit::ZERO);
}

#[test]
fn one_state() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    qureg.as_mut_slice()[0] = Complex::from(0.);
    qureg.as_mut_slice()[1] = Complex::from(1.);
    let mut qubit = qureg.qubit(0).unwrap();
    assert_eq!(qubit.measure(), Bit::ONE);
}

#[test]
fn m100_zero_state() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    let mut qubit = qureg.qubit(0).unwrap();

    for _ in 0..100 {
        assert_eq!(qubit.measure(), Bit::ZERO);
    }
}

#[test]
fn m100_one_state() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);
    qureg.as_mut_slice()[0] = Complex::from(0.);
    qureg.as_mut_slice()[1] = Complex::from(1.);

    let mut qubit = qureg.qubit(0).unwrap();

    for _ in 0..100 {
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

#[test]
fn m100_alternate() {
    let num_qubits = NonZeroU16::new(1).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits, 1);

    for _ in 0..100 {
        qureg.as_mut_slice()[0] = Complex::from(1.);
        qureg.as_mut_slice()[1] = Complex::from(0.);

        let mut qubit = qureg.qubit(0).unwrap();
        assert_eq!(qubit.measure(), Bit::ZERO);

        qureg.as_mut_slice()[0] = Complex::from(0.);
        qureg.as_mut_slice()[1] = Complex::from(1.);
        let mut qubit = qureg.qubit(0).unwrap();
        assert_eq!(qubit.measure(), Bit::ONE);
    }
}

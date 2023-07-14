use std::num::NonZeroU16;

use num::Complex;
use qn::Qureg;

#[test]
fn init_01() {
    let _ = Qureg::<f32>::new(NonZeroU16::try_from(1).unwrap());
    let _ = Qureg::<f32>::new(NonZeroU16::try_from(2).unwrap());
    let _ = Qureg::<f32>::new(NonZeroU16::try_from(3).unwrap());
}

#[test]
fn num_qubits_01() {
    let num_qubits = NonZeroU16::try_from(1).unwrap();
    let qureg = Qureg::<f32>::new(num_qubits);
    assert_eq!(qureg.num_qubits(), num_qubits);

    let num_qubits = NonZeroU16::try_from(3).unwrap();
    let qureg = Qureg::<f32>::new(num_qubits);
    assert_eq!(qureg.num_qubits(), num_qubits);
}

#[test]
fn as_slice_01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let qureg = Qureg::<f32>::new(num_qubits);

    assert_eq!(qureg.as_slice()[0], Complex::from(1.));
    assert_eq!(qureg.as_slice()[1], Complex::from(0.));
    assert_eq!(qureg.as_slice()[2], Complex::from(0.));
    assert_eq!(qureg.as_slice()[3], Complex::from(0.));
}

#[test]
fn as_mut_slice_01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits);

    let slice = qureg.as_mut_slice();
    slice[0] = Complex::from(4.);
    slice[1] = Complex::from(5.);
    slice[2] = Complex::from(6.);
    slice[3] = Complex::from(7.);

    assert_eq!(qureg.as_slice()[0], Complex::from(4.));
    assert_eq!(qureg.as_slice()[1], Complex::from(5.));
    assert_eq!(qureg.as_slice()[2], Complex::from(6.));
    assert_eq!(qureg.as_slice()[3], Complex::from(7.));
}

#[test]
fn get_qubit_01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits);

    let qubit = qureg.qubit(0);
    assert!(qubit.is_some());

    let qubit = qureg.qubit(1);
    assert!(qubit.is_some());

    let qubit = qureg.qubit(2);
    assert!(qubit.is_none());
}

#[test]
fn get_qubit_pair01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let mut qureg = Qureg::<f32>::new(num_qubits);

    let qb = qureg.qubit_pair(0, 1);
    assert!(qb.is_some());

    let qb = qureg.qubit_pair(0, 0);
    assert!(qb.is_none());

    let qb = qureg.qubit_pair(0, 2);
    assert!(qb.is_none());
}

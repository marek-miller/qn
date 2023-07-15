use std::num::NonZeroU16;

use num::Complex;
use qn::Register;

#[test]
fn init_01() {
    let _ = Register::<f32>::new(NonZeroU16::try_from(1).unwrap(), 1);
    let _ = Register::<f32>::new(NonZeroU16::try_from(2).unwrap(), 1);
    let _ = Register::<f32>::new(NonZeroU16::try_from(3).unwrap(), 1);
}

#[test]
fn num_qubits_01() {
    let num_qubits = NonZeroU16::try_from(1).unwrap();
    let qureg = Register::<f32>::new(num_qubits, 1);
    assert_eq!(qureg.num_qubits(), num_qubits);

    let num_qubits = NonZeroU16::try_from(3).unwrap();
    let qureg = Register::<f32>::new(num_qubits, 1);
    assert_eq!(qureg.num_qubits(), num_qubits);
}

#[test]
fn as_slice_01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let qureg = Register::<f32>::new(num_qubits, 1);

    assert_eq!(qureg.as_slice()[0], Complex::from(1.));
    assert_eq!(qureg.as_slice()[1], Complex::from(0.));
    assert_eq!(qureg.as_slice()[2], Complex::from(0.));
    assert_eq!(qureg.as_slice()[3], Complex::from(0.));
}

#[test]
fn as_mut_slice_01() {
    let num_qubits = NonZeroU16::try_from(2).unwrap();
    let mut qureg = Register::<f32>::new(num_qubits, 1);

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

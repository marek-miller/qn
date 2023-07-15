use std::{
    f64::{
        consts::SQRT_2,
        EPSILON,
    },
    num::NonZeroU16,
};

use qn::{
    Bit,
    System,
};

use crate::measure::{
    generate_stm_set_real,
    stm_set_real,
};

#[test]
fn q2_01() {
    let mut stm = generate_stm_set_real(2, 123, &[1., 0., 0., 0.]);
    let mut qubits = stm.qubit_pair(0, 1).unwrap();
    assert_eq!(qubits.0.measure(), Bit::ZERO);
    assert_eq!(qubits.1.measure(), Bit::ZERO);
}

#[test]
fn q2_02() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 1., 0., 0.]);
    let mut qubits = stm.qubit_pair(0, 1).unwrap();
    assert_eq!(qubits.0.measure(), Bit::ONE);
    assert_eq!(qubits.1.measure(), Bit::ZERO);
}

#[test]
fn q2_03() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 1., 0.]);
    let mut qubits = stm.qubit_pair(0, 1).unwrap();
    assert_eq!(qubits.0.measure(), Bit::ZERO);
    assert_eq!(qubits.1.measure(), Bit::ONE);
}

#[test]
fn q2_03_reversed() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 1., 0.]);
    let mut qubits = stm.qubit_pair(1, 0).unwrap();
    assert_eq!(qubits.0.measure(), Bit::ONE);
    assert_eq!(qubits.1.measure(), Bit::ZERO);
}

#[test]
fn q2_03_opposite_order() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 1., 0.]);
    let mut qubits = stm.qubit_pair(0, 1).unwrap();
    assert_eq!(qubits.1.measure(), Bit::ONE);
    assert_eq!(qubits.0.measure(), Bit::ZERO);
}

#[test]
fn q2_04() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 0., 1.]);
    let mut qubits = stm.qubit_pair(0, 1).unwrap();
    assert_eq!(qubits.0.measure(), Bit::ONE);
    assert_eq!(qubits.1.measure(), Bit::ONE);
}

#[test]
fn q2_correlated_01() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[SQRT_2.recip(), 0., 0., SQRT_2.recip()]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.0.measure();
        let outcome1 = qubits.1.measure();
        assert_eq!(outcome0, outcome1);

        if outcome0 == Bit::ZERO {
            for (x, y) in stm.as_slice().iter().zip(&[1., 0., 0., 0.]) {
                assert!((x.re - y).abs() < EPSILON);
                assert!(x.im.abs() < EPSILON);
            }
        } else {
            for (x, y) in stm.as_slice().iter().zip(&[0., 0., 0., 1.]) {
                assert!((x.re - y).abs() < EPSILON);
                assert!(x.im.abs() < EPSILON);
            }
        }
    }
}

#[test]
fn q2_correlated_01_one_measurement() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[SQRT_2.recip(), 0., 0., SQRT_2.recip()]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.0.measure();

        if outcome0 == Bit::ZERO {
            for (x, y) in stm.as_slice().iter().zip(&[1., 0., 0., 0.]) {
                assert!((x.re - y).abs() < EPSILON);
                assert!(x.im.abs() < EPSILON);
            }
        } else {
            for (x, y) in stm.as_slice().iter().zip(&[0., 0., 0., 1.]) {
                assert!((x.re - y).abs() < EPSILON);
                assert!(x.im.abs() < EPSILON);
            }
        }
    }
}

#[test]
fn q2_correlated_02() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[0., SQRT_2.recip(), SQRT_2.recip(), 0.]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.0.measure();
        let outcome1 = qubits.1.measure();
        assert_ne!(outcome0, outcome1);

        if outcome0 == Bit::ZERO {
            for (x, y) in stm.as_slice().iter().zip(&[0., 0., 1., 0.]) {
                assert!((x.re - y).abs() < EPSILON);
                assert!(x.im.abs() < EPSILON);
            }
        } else {
            for (x, y) in stm.as_slice().iter().zip(&[0., 1., 0., 0.]) {
                assert!((x.re - y).abs() < EPSILON);
                assert!(x.im.abs() < EPSILON);
            }
        }
    }
}

#[test]
fn q2_correlated_03() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        // |00> + |01> = |0>(|0> + |1>)
        stm_set_real(&mut stm, &[SQRT_2.recip(), 0., SQRT_2.recip(), 0.]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.0.measure();
        assert_eq!(outcome0, Bit::ZERO);

        for (x, y) in
            stm.as_slice()
                .iter()
                .zip(&[SQRT_2.recip(), 0., SQRT_2.recip(), 0.])
        {
            assert!((x.re - y).abs() < EPSILON, "{x:?}");
            assert!(x.im.abs() < EPSILON);
        }
    }
}

#[test]
fn q2_correlated_04() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        // |10> + |11> = |1>(|0> + |1>)
        stm_set_real(&mut stm, &[0., SQRT_2.recip(), 0., SQRT_2.recip()]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.0.measure();
        assert_eq!(outcome0, Bit::ONE);

        for (x, y) in
            stm.as_slice()
                .iter()
                .zip(&[0., SQRT_2.recip(), 0., SQRT_2.recip()])
        {
            assert!((x.re - y).abs() < EPSILON, "{x:?}");
            assert!(x.im.abs() < EPSILON);
        }
    }
}

#[test]
fn q2_correlated_05() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        // |00> + |10> = (|0> + |1>)|0>
        stm_set_real(&mut stm, &[SQRT_2.recip(), SQRT_2.recip(), 0., 0.]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.1.measure();
        assert_eq!(outcome0, Bit::ZERO);

        for (x, y) in
            stm.as_slice()
                .iter()
                .zip(&[SQRT_2.recip(), SQRT_2.recip(), 0., 0.])
        {
            assert!((x.re - y).abs() < EPSILON, "{x:?}");
            assert!(x.im.abs() < EPSILON);
        }
    }
}

#[test]
fn q2_correlated_06() {
    let num_qubits = NonZeroU16::new(2).unwrap();
    let mut stm = System::new(num_qubits, 123);

    for _ in 0..100 {
        // |01> + |11> = (|0> + |1>)|1>
        stm_set_real(&mut stm, &[0., 0., SQRT_2.recip(), SQRT_2.recip()]);
        let mut qubits = stm.qubit_pair(0, 1).unwrap();

        let outcome0 = qubits.1.measure();
        assert_eq!(outcome0, Bit::ONE);

        for (x, y) in
            stm.as_slice()
                .iter()
                .zip(&[0., 0., SQRT_2.recip(), SQRT_2.recip()])
        {
            assert!((x.re - y).abs() < EPSILON, "{x:?}");
            assert!(x.im.abs() < EPSILON);
        }
    }
}

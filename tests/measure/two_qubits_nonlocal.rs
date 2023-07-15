use std::{
    f64::consts::SQRT_2,
    thread,
};

use qn::{
    Bit,
    Qubit,
};

use crate::measure::{
    generate_stm_set_real,
    stm_set_real,
};

fn measure_qubits(
    qubits: (Qubit<'_, f64>, Qubit<'_, f64>)
) -> Option<(Bit, Bit)> {
    let (mut qb0, mut qb1) = qubits;
    let (mut out0, mut out1) = (None, None);
    thread::scope(|s| {
        s.spawn(|| out0 = Some(qb0.measure()));
        out1 = Some(qb1.measure());
    });
    Some((out0.unwrap(), out1.unwrap()))
}

#[test]
fn q2nloc_01() {
    let mut stm = generate_stm_set_real(2, 123, &[1., 0., 0., 0.]);
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ZERO, Bit::ZERO));
}

#[test]
fn q2nloc_02() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 1., 0., 0.]);
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ONE, Bit::ZERO));
}

#[test]
fn q2nloc_03() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 1., 0.]);
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ZERO, Bit::ONE));
}

#[test]
fn q2nloc_04() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 0., 1.]);
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ONE, Bit::ONE));
}

#[test]
fn q2nloc_01_reverse() {
    let mut stm = generate_stm_set_real(2, 123, &[1., 0., 0., 0.]);
    let qubits = stm.qubit_pair(1, 0).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ZERO, Bit::ZERO));
}

#[test]
fn q2nloc_02_reverse() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 1., 0., 0.]);
    let qubits = stm.qubit_pair(1, 0).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ZERO, Bit::ONE));
}

#[test]
fn q2nloc_03_reverse() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 1., 0.]);
    let qubits = stm.qubit_pair(1, 0).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ONE, Bit::ZERO));
}

#[test]
fn q2nloc_04_reverse() {
    let mut stm = generate_stm_set_real(2, 123, &[0., 0., 0., 1.]);
    let qubits = stm.qubit_pair(1, 0).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome, (Bit::ONE, Bit::ONE));
}

#[test]
fn q2nloc_entangled_01() {
    let mut stm = generate_stm_set_real(
        2,
        123,
        &[SQRT_2.recip(), 0., 0., SQRT_2.recip()],
    );
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome.0, outcome.1);
}

#[test]
fn q2nloc_entangled_02() {
    let mut stm = generate_stm_set_real(
        2,
        123,
        &[0., SQRT_2.recip(), SQRT_2.recip(), 0.],
    );
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_ne!(outcome.0, outcome.1);
}

#[test]
fn q2nloc_entangled_01_reps100() {
    let mut stm = generate_stm_set_real(
        2,
        123,
        &[SQRT_2.recip(), 0., 0., SQRT_2.recip()],
    );
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_eq!(outcome.0, outcome.1);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[SQRT_2.recip(), 0., 0., SQRT_2.recip()]);
        let qubits = stm.qubit_pair(0, 1).unwrap();
        let outcome = measure_qubits(qubits).unwrap();
        assert_eq!(outcome.0, outcome.1);
    }
}

#[test]
fn q2nloc_entangled_02_reps100() {
    let mut stm = generate_stm_set_real(
        2,
        123,
        &[0., SQRT_2.recip(), SQRT_2.recip(), 0.],
    );
    let qubits = stm.qubit_pair(0, 1).unwrap();
    let outcome = measure_qubits(qubits).unwrap();
    assert_ne!(outcome.0, outcome.1);

    for _ in 0..100 {
        stm_set_real(&mut stm, &[0., SQRT_2.recip(), SQRT_2.recip(), 0.]);
        let qubits = stm.qubit_pair(0, 1).unwrap();
        let outcome = measure_qubits(qubits).unwrap();
        assert_ne!(outcome.0, outcome.1);
    }
}

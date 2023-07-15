use std::num::NonZeroU16;

use num::Complex;
use qn::System;

fn gen_stm(
    num_qubits: u16,
    seed: u64,
) -> System<f64> {
    let num_qubits = NonZeroU16::new(num_qubits).unwrap();
    System::new(num_qubits, seed)
}

fn stm_set_real(
    stm: &mut System<f64>,
    amps: &[f64],
) {
    amps.iter()
        .map(Complex::from)
        .zip(stm.as_mut_slice())
        .for_each(|(x, a)| *a = x);
}

fn generate_stm_set_real(
    num_qubits: u16,
    seed: u64,
    amps: &[f64],
) -> System<f64> {
    let mut stm = gen_stm(num_qubits, seed);
    stm_set_real(&mut stm, amps);
    stm
}

mod one_qubit;
mod one_qubit_large_sys;
mod two_qubits;
mod two_qubits_nonlocal;

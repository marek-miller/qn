use std::num::NonZeroU16;

use num::Complex;
use qn::Register;

fn gen_qureg(
    num_qubits: u16,
    seed: u64,
) -> Register<f64> {
    let num_qubits = NonZeroU16::new(num_qubits).unwrap();
    Register::new(num_qubits, seed)
}

fn qureg_set_real(
    qureg: &mut Register<f64>,
    amps: &[f64],
) {
    amps.iter()
        .map(Complex::from)
        .zip(qureg.as_mut_slice())
        .for_each(|(x, a)| *a = x);
}

fn generate_qureg_set_real(
    num_qubits: u16,
    seed: u64,
    amps: &[f64],
) -> Register<f64> {
    let mut qureg = gen_qureg(num_qubits, seed);
    qureg_set_real(&mut qureg, amps);
    qureg
}

mod one_qubit;
mod two_qubits;
mod two_qubits_nonlocal;

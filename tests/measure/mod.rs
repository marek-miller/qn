use std::num::NonZeroU16;

use num::Complex;
use qn::Register;

fn generate_qureg_set_real(
    num_qubits: u16,
    seed: u64,
    amps: &[f64],
) -> Register<f64> {
    let num_qubits = NonZeroU16::new(num_qubits).unwrap();
    let mut qureg = Register::new(num_qubits, seed);
    qureg_set_real(&mut qureg, amps);
    qureg
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

mod one_qubit;
mod two_qubits;

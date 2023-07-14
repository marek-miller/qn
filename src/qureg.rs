use std::num::NonZeroU16;

use num::{
    Complex,
    Float,
};

use crate::Qubit;

/// Quantum register
pub struct Qureg<T: Float> {
    num_qubits: NonZeroU16,
    amp:        Vec<Complex<T>>,
}

impl<T: Float> Qureg<T> {
    /// Initialize a new quantum register of `n` qubits in a zero state.
    #[must_use]
    pub fn new(num_qubits: NonZeroU16) -> Self {
        let mut amp = vec![Complex::from(T::zero()); 1 << num_qubits.get()];
        amp[0] = Complex::from(T::one());
        Self {
            num_qubits,
            amp,
        }
    }

    /// Get the number of qubits in this `Qureg`
    #[must_use]
    pub fn num_qubits(&self) -> NonZeroU16 {
        self.num_qubits
    }

    /// Get complex amplitudes of the computational basis states
    #[must_use]
    pub fn as_slice(&self) -> &[Complex<T>] {
        &self.amp
    }

    /// Get mutable access to complex amplitudes of the computational basis
    /// states
    pub fn as_mut_slice(&mut self) -> &mut [Complex<T>] {
        &mut self.amp
    }

    /// Get a qubit.
    ///
    /// Returns `None` is index is larger or equal than `qureg.num_qubits()`
    pub fn qubit(
        &mut self,
        index: u16,
    ) -> Option<Qubit<'_, T>> {
        Qubit::new(self, index)
    }

    /// Get a pair of qubits.
    ///
    /// # Result
    ///
    /// Returns `None`
    /// - if any of indices is larger or equal than `qureg.num_qubits()`
    /// - if indices are equal
    pub fn qubit_pair(
        &mut self,
        index1: u16,
        index2: u16,
    ) -> Option<(Qubit<'_, T>, Qubit<'_, T>)> {
        Qubit::new_pair(self, index1, index2)
    }
}

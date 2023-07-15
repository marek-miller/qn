use std::num::NonZeroU16;

use num::{
    Complex,
    Zero,
};
use rand::{
    distributions::{
        Bernoulli,
        BernoulliError,
        Distribution,
    },
    SeedableRng,
};
use rand_chacha::ChaCha8Rng;

use crate::{
    Float,
    Qubit,
};

/// Quantum register
pub struct Register<T>
where
    T: Float,
{
    rng:        ChaCha8Rng,
    num_qubits: NonZeroU16,
    amp:        Vec<Complex<T>>,
}

impl<T> Register<T>
where
    T: Float,
{
    /// Initialize a new quantum register of `n` qubits in the zero state.
    ///
    /// Seed internal RNG with `seed`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::num::NonZeroU16;
    /// # use qn::{Bit, Register};
    /// let num_qubits = NonZeroU16::new(8).unwrap();
    /// let seed = 123;
    /// let mut qureg: Register<f64> = Register::new(num_qubits, seed);
    ///
    /// for mut qubit in qureg.qubit_iter() {
    ///     assert_eq!(qubit.measure(), Bit::ZERO);
    /// }
    /// ```
    #[must_use]
    pub fn new(
        num_qubits: NonZeroU16,
        seed: u64,
    ) -> Self {
        let mut amp = vec![Complex::zero(); 1 << num_qubits.get()];
        amp[0] = Complex::from(T::one());
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            num_qubits,
            amp,
        }
    }

    /// Draw from Bernoulli distribution with probability of success `p`.
    ///
    /// Uses internal RNG.
    pub(crate) fn bernoulli(
        &mut self,
        p: f64,
    ) -> Result<bool, BernoulliError> {
        Ok(Bernoulli::new(p)?.sample(&mut self.rng))
    }

    /// Get the number of qubits.
    #[must_use]
    pub fn num_qubits(&self) -> NonZeroU16 {
        self.num_qubits
    }

    /// Get complex amplitudes of the computational basis states.
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
    /// Returns `None` if index is larger or equal than `self.num_qubits()`
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
    /// - if any of indices is larger or equal than `self.num_qubits()`
    /// - if indices are equal
    pub fn qubit_pair(
        &mut self,
        index1: u16,
        index2: u16,
    ) -> Option<(Qubit<'_, T>, Qubit<'_, T>)> {
        Qubit::new_pair(self, index1, index2)
    }

    /// Create an iterator over all qubits in register.
    pub fn qubit_iter(&mut self) -> impl Iterator<Item = Qubit<'_, T>> {
        Qubit::new_iter(self)
    }
}

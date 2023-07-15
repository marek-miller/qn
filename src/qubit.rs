use std::sync::{
    Arc,
    Mutex,
};

use num::{
    Complex,
    Zero,
};

use crate::{
    Float,
    Register,
};

/// Classical bit with two possible values (ZERO and ONE)
#[derive(Debug, PartialEq)]
pub enum Bit {
    ZERO,
    ONE,
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value {
            Self::ONE
        } else {
            Self::ZERO
        }
    }
}

/// A representation of a qubit in a quantum register.
pub struct Qubit<'a, T>
where
    T: Float,
{
    qureg: Arc<Mutex<&'a mut Register<T>>>,
    index: u16,
}

impl<'a, T> Qubit<'a, T>
where
    T: Float,
{
    /// Derive a single qubit from a quantum register.
    ///
    /// Returns `None` is index is larger or equal than `qureg.num_qubits()`
    pub fn new(
        qureg: &'a mut Register<T>,
        index: u16,
    ) -> Option<Qubit<'_, T>> {
        if index >= qureg.num_qubits().get() {
            None
        } else {
            Some(Self {
                qureg: Arc::new(Mutex::new(qureg)),
                index,
            })
        }
    }

    /// Get a pair of qubits from the same register.
    ///
    /// # Result
    ///
    /// Returns `None`
    /// - if any of indices is larger or equal than `qureg.num_qubits()`
    /// - if indices are equal
    pub(crate) fn new_pair(
        qureg: &'a mut Register<T>,
        index1: u16,
        index2: u16,
    ) -> Option<(Qubit<'_, T>, Qubit<'_, T>)> {
        if index1 >= qureg.num_qubits().get()
            || index2 >= qureg.num_qubits().get()
            || index1 == index2
        {
            return None;
        }

        let lock = Arc::new(Mutex::new(qureg));
        let qb1 = Self {
            qureg: lock.clone(),
            index: index1,
        };
        let qb2 = Self {
            qureg: lock,
            index: index2,
        };
        Some((qb1, qb2))
    }

    /// Get iterator over all qubits in register
    pub(crate) fn new_iter(
        qureg: &'a mut Register<T>
    ) -> impl Iterator<Item = Qubit<'a, T>> {
        let num_qubits = qureg.num_qubits().get();
        let lock = Arc::new(Mutex::new(qureg));

        (0..num_qubits).map(move |i| Self {
            qureg: lock.clone(),
            index: i,
        })
    }

    /// Get index of this qubit in the underlying register
    #[must_use]
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Check if other qubit belongs to the same register
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::num::NonZeroU16;
    /// # use qn::Register;
    /// let num_qubits = NonZeroU16::new(2).unwrap();
    /// let seed = 123;
    /// let mut qureg: Register<f64> = Register::new(num_qubits, seed);
    /// let (qb0, qb1) = qureg.qubit_pair(0, 1).unwrap();
    ///
    /// assert!(qb0.is_from_same_reg(&qb1));
    /// ```
    #[must_use]
    pub fn is_from_same_reg(
        &self,
        other_qubit: &Qubit<'a, T>,
    ) -> bool {
        // Two qubits belong to the same register, if and only if they share
        // the same mutex guarding access to &mut Register.
        Arc::<_>::as_ptr(&self.qureg) == Arc::<_>::as_ptr(&other_qubit.qureg)
    }

    #[must_use]
    /// Measure the qubit.
    pub fn measure(&mut self) -> Bit {
        let mut qureg = self.qureg.lock().unwrap();

        let lower_bits = 1 << self.index;
        let upper_bits = 1 << (qureg.num_qubits().get() - self.index - 1);
        let amp_buf = qureg.as_mut_slice();

        // calculate sum of squares of prob. amplitudes for outcomes 0 and 1
        let mut amp_sq = [T::zero(); 2];
        for k in 0..upper_bits {
            for i in 0..lower_bits {
                amp_sq[0] += amp_buf[i + (2 * k) * lower_bits].norm_sqr();
                amp_sq[1] += amp_buf[i + (2 * k + 1) * lower_bits].norm_sqr();
            }
        }

        // project the state onto random outcome
        let p = T::to_f64(&amp_sq[1]).unwrap();
        let outcome = qureg.bernoulli(p).unwrap();

        // zero amplitudes corresponding to (1-outcome), normalize the rest
        let out_idx = usize::from(outcome);
        let amp_buf = qureg.as_mut_slice();
        let norm_factor = amp_sq[out_idx].sqrt();
        for k in 0..upper_bits {
            for i in 0..lower_bits {
                amp_buf[i + (2 * k + out_idx) * lower_bits] /= norm_factor;
                amp_buf[i + (2 * k + (1 - out_idx)) * lower_bits] =
                    Complex::zero();
            }
        }
        outcome.into()
    }
}

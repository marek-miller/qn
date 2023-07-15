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
    System,
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

/// A representation of a qubit in a quantum system.
pub struct Qubit<'a, T>
where
    T: Float,
{
    stm:   Arc<Mutex<&'a mut System<T>>>,
    index: u16,
}

impl<'a, T> Qubit<'a, T>
where
    T: Float,
{
    /// Derive a single qubit from a quantum system.
    ///
    /// Returns `None` is index is larger or equal than `stm.num_qubits()`
    pub fn new(
        stm: &'a mut System<T>,
        index: u16,
    ) -> Option<Qubit<'_, T>> {
        if index >= stm.num_qubits().get() {
            None
        } else {
            Some(Self {
                stm: Arc::new(Mutex::new(stm)),
                index,
            })
        }
    }

    /// Get a pair of qubits from the same system.
    ///
    /// # Result
    ///
    /// Returns `None`
    /// - if any of indices is larger or equal than `stm.num_qubits()`
    /// - if indices are equal
    pub(crate) fn new_pair(
        stm: &'a mut System<T>,
        index1: u16,
        index2: u16,
    ) -> Option<(Qubit<'_, T>, Qubit<'_, T>)> {
        if index1 >= stm.num_qubits().get()
            || index2 >= stm.num_qubits().get()
            || index1 == index2
        {
            return None;
        }

        let lock = Arc::new(Mutex::new(stm));
        let qb1 = Self {
            stm:   lock.clone(),
            index: index1,
        };
        let qb2 = Self {
            stm:   lock,
            index: index2,
        };
        Some((qb1, qb2))
    }

    /// Get iterator over all qubits in system
    pub(crate) fn new_iter(
        stm: &'a mut System<T>
    ) -> impl Iterator<Item = Qubit<'a, T>> {
        let num_qubits = stm.num_qubits().get();
        let lock = Arc::new(Mutex::new(stm));

        (0..num_qubits).map(move |i| Self {
            stm:   lock.clone(),
            index: i,
        })
    }

    /// Get index of this qubit in the underlying system
    #[must_use]
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Check if other qubit belongs to the same system
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::num::NonZeroU16;
    /// # use qn::System;
    /// let num_qubits = NonZeroU16::new(2).unwrap();
    /// let seed = 123;
    /// let mut stm: System<f64> = System::new(num_qubits, seed);
    /// let (qb0, qb1) = stm.qubit_pair(0, 1).unwrap();
    ///
    /// assert!(qb0.is_from_same_stm(&qb1));
    /// ```
    #[must_use]
    pub fn is_from_same_stm(
        &self,
        other_qubit: &Qubit<'a, T>,
    ) -> bool {
        // Two qubits belong to the same system, if and only if they share
        // the same mutex guarding access to &mut System.
        Arc::<_>::as_ptr(&self.stm) == Arc::<_>::as_ptr(&other_qubit.stm)
    }

    #[must_use]
    /// Measure the qubit.
    pub fn measure(&mut self) -> Bit {
        let mut stm = self.stm.lock().unwrap();

        let lower_bits = 1 << self.index;
        let upper_bits = 1 << (stm.num_qubits().get() - self.index - 1);
        let amp_buf = stm.as_mut_slice();

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
        let outcome = stm.bernoulli(p).unwrap();

        // zero amplitudes corresponding to (1-outcome), normalize the rest
        let out_idx = usize::from(outcome);
        let amp_buf = stm.as_mut_slice();
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

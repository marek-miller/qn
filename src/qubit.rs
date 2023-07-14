use std::sync::{
    Arc,
    Mutex,
};

use num::{
    Complex,
    Float,
    Zero,
};

use crate::Qureg;

pub enum Bit {
    ZERO,
    ONE,
}

/// A representation of a qubit in a quantum register.
pub struct Qubit<'a, T: Float> {
    qureg: Arc<Mutex<&'a mut Qureg<T>>>,
    index: u16,
}

impl<'a, T: Float> Qubit<'a, T> {
    /// Derive a single qubit from a quantum register.
    ///
    /// Returns `None` is index is larger or equal than `qureg.num_qubits()`
    pub fn new(
        qureg: &'a mut Qureg<T>,
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
    pub fn new_pair(
        qureg: &'a mut Qureg<T>,
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

    /// Get index of this qubit in the underlying register
    pub fn index(&self) -> u16 {
        self.index
    }

    /// Check if other qubit belongs to the same register
    pub fn is_from_same_qureg(
        &self,
        other_qubit: &Qubit<'_, T>,
    ) -> bool {
        // Note the new scope: other_qubit lock is dropped immediately
        let other_qureg_ptr =
            { *other_qubit.qureg.lock().unwrap() as *const _ };
        let qureg_ptr = *self.qureg.lock().unwrap() as *const _;

        qureg_ptr == other_qureg_ptr
    }

    /// Measure the qubit.
    pub fn measure(&mut self) -> Bit {
        let mut amp = [Complex::<T>::zero(); 2];

        let outcome = {
            let mut qureg = self.qureg.lock().unwrap();
            let lower_bits = 1 << self.index;
            let upper_bits = 1 << (qureg.num_qubits().get() - self.index - 1);
            let amp_buf = qureg.as_mut_slice();

            for k in 0..upper_bits {
                for j in 0..=1 {
                    for i in 0..lower_bits {
                        amp[j] = amp[j] + amp_buf[i + lower_bits * (j + 2 * k)];
                    }
                }
            }

            // TODO: impl. pseudo RNG in qureg
            let outcome = 0; // for now
            let outcome_amp = amp[outcome];

            // zero amplitudes from (1-outcome), normalize the rest
            for k in 0..upper_bits {
                for i in 0..lower_bits {
                    amp_buf[i + lower_bits * ((1 - outcome) + 2 * k)] =
                        Complex::zero();
                    amp_buf[i + lower_bits * (outcome + 2 * k)] = amp_buf
                        [i + lower_bits * (outcome + 2 * k)]
                        / outcome_amp;
                }
            }
            outcome
            // qureg is dropped here
        };

        match outcome {
            0 => Bit::ZERO,
            1 => Bit::ONE,
            _ => panic!("outcome should always be 0 or 1"),
        }
    }
}

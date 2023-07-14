use std::sync::{
    Arc,
    Mutex,
};

use num::Float;

use crate::Qureg;

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
}

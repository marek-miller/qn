# qn

A non-local system of qubits and a collection of thread-safe qubit
transformations.

- Emulate entangled physical systems in concurrent code: measurements in one
  thread have immediate effect on measurements of entangled qubits in other
  parts of the system.
- Synchronization primitives and borrow checker assures thread-safety and
  uniqueness of qubits.
- Tensor structure of the Hilbert space allows for straightforward
  parallelization: we use rayon's parallel iterators to squeeze maximum
  performance on CPU:

  - Simple benchmarking suggests measurement of a single qubit in a 20 qubit
    register is already faster than with
    [`quest_bind`](https://github.com/marek-miller/quest-bind.git): below 2ms on
    my laptop.

## TODO

- Write gates/operators, expand test suite.
- Write intro:
  - quantum info perspective
  - system programming perspective
- Write example: _A and B share a Bell state_
- Write documentation and tutorial in `README.md`

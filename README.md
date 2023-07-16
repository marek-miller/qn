# qn

[![Test](https://github.com/marek-miller/qn/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/marek-miller/qn/actions/workflows/test.yml)
[![Docs](https://github.com/marek-miller/qn/actions/workflows/docs.yml/badge.svg?branch=main)](https://github.com/marek-miller/qn/actions/workflows/docs.yml)

A non-local system of qubits and a collection of thread-safe qubit
transformations.

- Emulate entangled physical systems in concurrent code: measurements in one
  thread have immediate effect on measurements of entangled qubits in other
  parts of the system.
- Synchronization primitives and borrow checker assure thread-safety and
  uniqueness of qubits.
- Tensor structure of the Hilbert space allows for straightforward
  parallelization: we use rayon's parallel iterators to squeeze maximum
  performance out of CPU (what about GPU?):

  - Simple benchmarking suggests measurement of a single qubit in a 20 qubit
    register is already faster than with
    [`quest_bind`](https://github.com/marek-miller/quest-bind.git). What about
    raw QuEST?
  - Tensor iterators to turn single-qubit transformations in a multi-qubit register
    into serial operations on vector of amplitudes

## TODO

- Write gates/operators, expand test suite.
- Write intro:
  - quantum info perspective
  - system programming perspective
- Write example: _A and B share a Bell state_
- Write documentation and tutorial in `README.md`

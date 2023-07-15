#![feature(portable_simd)]
use std::num::NonZeroU16;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use qn::System;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample
    // size to improve precision and counteract the resulting noise.
    // group.significance_level(0.1).sample_size(10);

    const NUM_QUBITS: u16 = 23;
    let num_qubits = NonZeroU16::new(NUM_QUBITS).unwrap();
    let mut stm = System::<f64>::new(num_qubits, 349812);
    let mut qubit = stm.qubit(16).unwrap();

    group.bench_function("1. binary state", |b| b.iter(|| qubit.measure()));

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

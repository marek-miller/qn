#![feature(portable_simd)]
use std::num::NonZeroU16;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use qn::{
    hadamard,
    System,
};

fn bench(c: &mut Criterion) {
    const NUM_QUBITS: u16 = 20;
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample
    // size to improve precision and counteract the resulting noise.
    // group.significance_level(0.1).sample_size(10);

    let num_qubits = NonZeroU16::new(NUM_QUBITS).unwrap();
    let mut stm = System::<f64>::new(num_qubits, 349_812);
    let mut qubit = stm.qubit(15).unwrap();

    group.bench_function("measure", |b| b.iter(|| qubit.measure()));
    group.bench_function("hadamard", |b| b.iter(|| hadamard(&mut qubit)));

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

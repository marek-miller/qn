#![feature(portable_simd)]
use std::num::NonZeroU16;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use num::{
    Complex,
    Zero,
};
use qn::{
    Bit,
    System,
};

fn gen_stm(
    num_qubits: u16,
    seed: u64,
) -> System<f64> {
    let num_qubits = NonZeroU16::new(num_qubits).unwrap();
    System::new(num_qubits, seed)
}

fn binary_state() {
    const SIZE: u16 = 10;

    let mut stm = gen_stm(SIZE, 349812);

    for i in 0..1 << SIZE {
        (0..1 << SIZE).for_each(|j| stm.as_mut_slice()[j] = Complex::zero());
        stm.as_mut_slice()[i] = Complex::from(1.);

        for k in 0..SIZE {
            let outcome = stm.qubit(k).unwrap().measure();
            if (i >> k & 1) == 0 {
                assert_eq!(outcome, Bit::ZERO);
            } else {
                assert_eq!(outcome, Bit::ONE);
            }
        }
    }
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample
    // size to improve precision and counteract the resulting noise.
    // group.significance_level(0.1).sample_size(10);
    group.bench_function("1. binary state", |b| b.iter(binary_state));
    // group.bench_function("2. fwht4_simd", |b| {
    //     b.iter(|| fwht4_simd_02(&mut scratch_4))
    // });
    // group.bench_function("2. fwht8_simd", |b| {
    //     b.iter(|| fwht8_simd_02(&mut scratch_8))
    // });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);

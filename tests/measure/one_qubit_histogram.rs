use std::f64::consts::SQRT_2;

use num::Complex;
use qn::Bit;

use crate::measure::{
    gen_stm,
    stm_set_imag,
    stm_set_real,
};

const SAMPLES: usize = 10000;
const SAMPLES_RECIP: f64 = 0.0001;
const MARGIN: f64 = 0.01;

#[test]
fn plus_state_q1() {
    let mut stm = gen_stm(1, 93);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm_set_real(&mut stm, &[SQRT_2.recip(), SQRT_2.recip()]);
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q1_imag() {
    let mut stm = gen_stm(1, 93);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm_set_imag(&mut stm, &[SQRT_2.recip(), SQRT_2.recip()]);
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q1_complex_01() {
    let mut stm = gen_stm(1, 93);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm.as_mut_slice()[0] = Complex::new(SQRT_2.recip(), 0.);
        stm.as_mut_slice()[1] = Complex::new(0., SQRT_2.recip());
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q1_complex_02() {
    let mut stm = gen_stm(1, 93);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm.as_mut_slice()[0] = Complex::new(0.5, 0.5);
        stm.as_mut_slice()[1] = Complex::new(0.5, -1. * 0.5);
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q2_01() {
    let mut stm = gen_stm(2, 933);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm_set_real(&mut stm, &[SQRT_2.recip(), SQRT_2.recip(), 0., 0.]);
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q2_02() {
    let mut stm = gen_stm(2, 933);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm_set_real(&mut stm, &[SQRT_2.recip(), 0., SQRT_2.recip(), 0.]);
        let mut qubit = stm.qubit(1).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q2_03() {
    let mut stm = gen_stm(2, 933);

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm_set_real(&mut stm, &[SQRT_2.recip(), 0., 0., SQRT_2.recip()]);
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn plus_state_q8_01() {
    let mut stm = gen_stm(8, 49233);
    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm.as_mut_slice()[0] = Complex::from(SQRT_2.recip());
        stm.as_mut_slice()[1] = Complex::from(SQRT_2.recip());
        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - 0.5) < MARGIN);
    assert!((outcomes_count.1 - 0.5) < MARGIN);
}

#[test]
fn some_state_q8_01() {
    let mut stm = gen_stm(8, 2329);

    let p = 0.11f64;
    let alpha = 0.214f64;

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm.as_mut_slice()[0] = Complex::new(
            p.sqrt() * alpha.sqrt(),
            p.sqrt() * (1. - alpha).sqrt(),
        );
        stm.as_mut_slice()[1] = Complex::new(
            (1. - p).sqrt() * alpha.sqrt(),
            (1. - p).sqrt() * (1. - alpha).sqrt(),
        );

        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - p) < MARGIN);
    assert!((outcomes_count.1 - 1. + p) < MARGIN);
}

#[test]
fn some_state_q8_02() {
    let mut stm = gen_stm(8, 29);

    let p = 0.91f64;
    let alpha = 0.7214f64;

    let mut outcomes_count = (0., 0.);

    for _ in 0..SAMPLES {
        stm.as_mut_slice()[0] = Complex::new(
            p.sqrt() * alpha.sqrt(),
            p.sqrt() * (1. - alpha).sqrt(),
        );
        stm.as_mut_slice()[1] = Complex::new(
            (1. - p).sqrt() * alpha.sqrt(),
            (1. - p).sqrt() * (1. - alpha).sqrt(),
        );

        let mut qubit = stm.qubit(0).unwrap();
        match qubit.measure() {
            Bit::ZERO => outcomes_count.0 += SAMPLES_RECIP,
            Bit::ONE => outcomes_count.1 += SAMPLES_RECIP,
        }
    }

    assert!((outcomes_count.0 - p) < MARGIN);
    assert!((outcomes_count.1 - 1. + p) < MARGIN);
}

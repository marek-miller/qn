use qn::{
    TensorIter,
    TensorIterMut,
};

use crate::test_cases;

#[test]
fn tensor_iter_10() {
    const DIMENSION: usize = 1;
    const SITE: usize = 0;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE).collect::<Vec<_>>();

    assert_eq!(result, &[(&0b0, &0b1),]);
}

#[test]
fn tensor_iter_20() {
    const DIMENSION: usize = 2;
    const SITE: usize = 0;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE).collect::<Vec<_>>();

    assert_eq!(result, &[(&0b00, &0b01), (&0b10, &0b11)]);
}

#[test]
fn tensor_iter_21() {
    const DIMENSION: usize = 2;
    const SITE: usize = 1;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE).collect::<Vec<_>>();

    assert_eq!(result, &[(&0b00, &0b10), (&0b01, &0b11)]);
}

#[test]
fn tensor_iter_30() {
    const DIMENSION: usize = 3;
    const SITE: usize = 0;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&0b000, &0b001),
            (&0b010, &0b011),
            (&0b100, &0b101),
            (&0b110, &0b111)
        ]
    );
}

#[test]
fn tensor_iter_31() {
    const DIMENSION: usize = 3;
    const SITE: usize = 1;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&0b000, &0b010),
            (&0b001, &0b011),
            (&0b100, &0b110),
            (&0b101, &0b111)
        ]
    );
}

#[test]
fn tensor_iter_32() {
    const DIMENSION: usize = 3;
    const SITE: usize = 2;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&0b000, &0b100),
            (&0b001, &0b101),
            (&0b010, &0b110),
            (&0b011, &0b111)
        ]
    );
}

#[test]
fn tensor_iter_mut_10() {
    const DIMENSION: usize = 1;
    const SITE: usize = 0;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    assert_eq!(result, &[(&mut 0b0, &mut 0b1),]);
}

#[test]
fn tensor_iter_mut_20() {
    const DIMENSION: usize = 2;
    const SITE: usize = 0;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    assert_eq!(result, &[(&mut 0b00, &mut 0b01), (&mut 0b10, &mut 0b11)]);
}

#[test]
fn tensor_iter_mut_21() {
    const DIMENSION: usize = 2;
    const SITE: usize = 1;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    assert_eq!(result, &[(&mut 0b00, &mut 0b10), (&mut 0b01, &mut 0b11)]);
}

#[test]
fn tensor_iter_mut_30() {
    const DIMENSION: usize = 3;
    const SITE: usize = 0;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&mut 0b000, &mut 0b001),
            (&mut 0b010, &mut 0b011),
            (&mut 0b100, &mut 0b101),
            (&mut 0b110, &mut 0b111)
        ]
    );
}

#[test]
fn tensor_iter_mut_31() {
    const DIMENSION: usize = 3;
    const SITE: usize = 1;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&mut 0b000, &mut 0b010),
            (&mut 0b001, &mut 0b011),
            (&mut 0b100, &mut 0b110),
            (&mut 0b101, &mut 0b111)
        ]
    );
}

#[test]
fn tensor_iter_mut_32() {
    const DIMENSION: usize = 3;
    const SITE: usize = 2;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    assert_eq!(
        result,
        &[
            (&mut 0b000, &mut 0b100),
            (&mut 0b001, &mut 0b101),
            (&mut 0b010, &mut 0b110),
            (&mut 0b011, &mut 0b111)
        ]
    );
}

fn check_tensor_iter_nk(
    (dim, site): (usize, usize),
    _: (),
    _: (),
) {
    let mask: usize = !(1 << site);
    let reg = (0..1 << dim).collect::<Vec<_>>();

    let result = TensorIter::<'_, usize>::new(&reg, site).collect::<Vec<_>>();
    assert_eq!(result.len(), 1 << (dim - 1));

    let mut count_0 = 0;
    let mut count_1 = 0;

    for (&val_0, &val_1) in result {
        check_val_update_counter(val_0, mask, site, &mut count_0);
        check_val_update_counter(val_1, mask, site, &mut count_1);
    }
}

fn check_val_update_counter(
    val: usize,
    mask: usize,
    site: usize,
    count: &mut usize,
) {
    let val_0 = val & mask;
    let hi_0 = (val_0 >> (site + 1)) << (site + 1);
    let lo_0 = val_0 - hi_0;
    let hi_0 = hi_0 >> 1;
    let val_shift_0 = lo_0 + hi_0;

    assert_eq!(val_shift_0, *count);

    *count += 1;
}

test_cases! {
    tensor_iter_nk_10: check_tensor_iter_nk (1,0), (), ();
    tensor_iter_nk_20: check_tensor_iter_nk (2,0), (), ();
    tensor_iter_nk_21: check_tensor_iter_nk (2,1), (), ();
    tensor_iter_nk_30: check_tensor_iter_nk (3,0), (), ();
    tensor_iter_nk_31: check_tensor_iter_nk (3,1), (), ();
    tensor_iter_nk_32: check_tensor_iter_nk (3,2), (), ();
    tensor_iter_nk_80: check_tensor_iter_nk (8,0), (), ();
    tensor_iter_nk_83: check_tensor_iter_nk (8,3), (), ();
    tensor_iter_nk_87: check_tensor_iter_nk (8,7), (), ();
    tensor_iter_nk_2100: check_tensor_iter_nk (21,0), (), ();
    tensor_iter_nk_2101: check_tensor_iter_nk (21,1), (), ();
    tensor_iter_nk_2117: check_tensor_iter_nk (21,17), (), ();
    tensor_iter_nk_2124: check_tensor_iter_nk (21,20), (), ();
}

fn check_tensor_iter_mut_nk(
    (dim, site): (usize, usize),
    _: (),
    _: (),
) {
    let mask: usize = !(1 << site);
    let mut reg = (0..1 << dim).collect::<Vec<_>>();

    let result =
        TensorIterMut::<'_, usize>::new(&mut reg, site).collect::<Vec<_>>();
    assert_eq!(result.len(), 1 << (dim - 1));

    let mut count_0 = 0;
    let mut count_1 = 0;

    for (&mut val_0, &mut val_1) in result {
        check_val_update_counter(val_0, mask, site, &mut count_0);
        check_val_update_counter(val_1, mask, site, &mut count_1);
    }
}

test_cases! {
    tensor_iter_mut_nk_10: check_tensor_iter_mut_nk (1,0), (), ();
    tensor_iter_mut_nk_20: check_tensor_iter_mut_nk (2,0), (), ();
    tensor_iter_mut_nk_21: check_tensor_iter_mut_nk (2,1), (), ();
    tensor_iter_mut_nk_30: check_tensor_iter_mut_nk (3,0), (), ();
    tensor_iter_mut_nk_31: check_tensor_iter_mut_nk (3,1), (), ();
    tensor_iter_mut_nk_32: check_tensor_iter_mut_nk (3,2), (), ();
    tensor_iter_mut_nk_80: check_tensor_iter_mut_nk (8,0), (), ();
    tensor_iter_mut_nk_83: check_tensor_iter_mut_nk (8,3), (), ();
    tensor_iter_mut_nk_87: check_tensor_iter_mut_nk (8,7), (), ();
    tensor_iter_mut_nk_2100: check_tensor_iter_mut_nk (21,0), (), ();
    tensor_iter_mut_nk_2101: check_tensor_iter_mut_nk (21,1), (), ();
    tensor_iter_mut_nk_2117: check_tensor_iter_mut_nk (21,17), (), ();
    tensor_iter_mut_nk_2124: check_tensor_iter_mut_nk (21,20), (), ();
}

#[test]
fn iter_mut_mutability() {
    const DIMENSION: usize = 3;
    const SITE: usize = 2;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let mut result = TensorIterMut::new(&mut reg, SITE).collect::<Vec<_>>();

    let (val_0, val_1) = &mut result[3];
    **val_0 = 77;
    **val_1 = 99;

    assert_eq!(reg[3], 77);
    assert_eq!(reg[7], 99);
}

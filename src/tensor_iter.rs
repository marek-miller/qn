pub struct TensorIter<'a, T> {
    buf:         &'a [T],
    lower_bits:  usize,
    upper_bits:  usize,
    lower_index: usize,
    upper_index: usize,
}

impl<'a, T> TensorIter<'a, T> {
    pub fn new(
        buf: &'a [T],
        site: usize,
        dimension: usize,
    ) -> Self {
        assert!(dimension > site);
        Self {
            buf,
            lower_bits: 1usize << site,
            upper_bits: 1usize << (dimension - site - 1),
            lower_index: 0,
            upper_index: 0,
        }
    }
}

impl<'a, T> Iterator for TensorIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.upper_index >= self.upper_bits {
            return None;
        }

        let x = &self.buf
            [self.lower_index + self.lower_bits * (2 * self.upper_index)];
        let y = &self.buf
            [self.lower_index + self.lower_bits * (2 * self.upper_index + 1)];

        self.lower_index += 1;
        if self.lower_index >= self.lower_bits {
            self.upper_index += 1;
            self.lower_index = 0;
        }
        Some((x, y))
    }
}

pub struct TensorIterMut<'a, T> {
    buf:         &'a mut [T],
    lower_bits:  usize,
    upper_bits:  usize,
    lower_index: usize,
    upper_index: usize,
}

impl<'a, T> TensorIterMut<'a, T> {
    pub fn new(
        buf: &'a mut [T],
        site: usize,
        dimension: usize,
    ) -> Self {
        assert!(dimension > site);
        Self {
            buf,
            lower_bits: 1usize << site,
            upper_bits: 1usize << (dimension - site - 1),
            lower_index: 0,
            upper_index: 0,
        }
    }
}

impl<'a, T> Iterator for TensorIterMut<'a, T> {
    type Item = (&'a mut T, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.upper_index >= self.upper_bits {
            return None;
        }

        let index_lo = (self.lower_index
            + self.lower_bits * (2 * self.upper_index))
            as isize;
        let index_hi = (self.lower_index
            + self.lower_bits * (2 * self.upper_index + 1))
            as isize;

        let buf_ptr = self.buf.as_mut_ptr();
        let (x, y) = unsafe {
            let x = buf_ptr.offset(index_lo).as_mut().unwrap();
            let y = buf_ptr.offset(index_hi).as_mut().unwrap();
            (x, y)
        };

        self.lower_index += 1;

        if self.lower_index >= self.lower_bits {
            self.upper_index += 1;
            self.lower_index = 0;
        }
        Some((x, y))
    }
}

#[test]
fn tensor_iter_10() {
    const DIMENSION: usize = 1;
    const SITE: usize = 0;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE, DIMENSION).collect::<Vec<_>>();

    assert_eq!(result, &[(&0b0, &0b1),]);
}

#[test]
fn tensor_iter_20() {
    const DIMENSION: usize = 2;
    const SITE: usize = 0;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE, DIMENSION).collect::<Vec<_>>();

    assert_eq!(result, &[(&0b00, &0b01), (&0b10, &0b11)]);
}

#[test]
fn tensor_iter_21() {
    const DIMENSION: usize = 2;
    const SITE: usize = 1;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE, DIMENSION).collect::<Vec<_>>();

    assert_eq!(result, &[(&0b00, &0b10), (&0b01, &0b11)]);
}

#[test]
fn tensor_iter_30() {
    const DIMENSION: usize = 3;
    const SITE: usize = 0;
    let reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result = TensorIter::new(&reg, SITE, DIMENSION).collect::<Vec<_>>();

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

    let result = TensorIter::new(&reg, SITE, DIMENSION).collect::<Vec<_>>();

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

    let result = TensorIter::new(&reg, SITE, DIMENSION).collect::<Vec<_>>();

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

    let result =
        TensorIterMut::new(&mut reg, SITE, DIMENSION).collect::<Vec<_>>();

    assert_eq!(result, &[(&mut 0b0, &mut 0b1),]);
}

#[test]
fn tensor_iter_mut_20() {
    const DIMENSION: usize = 2;
    const SITE: usize = 0;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result =
        TensorIterMut::new(&mut reg, SITE, DIMENSION).collect::<Vec<_>>();

    assert_eq!(result, &[(&mut 0b00, &mut 0b01), (&mut 0b10, &mut 0b11)]);
}

#[test]
fn tensor_iter_mut_21() {
    const DIMENSION: usize = 2;
    const SITE: usize = 1;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result =
        TensorIterMut::new(&mut reg, SITE, DIMENSION).collect::<Vec<_>>();

    assert_eq!(result, &[(&mut 0b00, &mut 0b10), (&mut 0b01, &mut 0b11)]);
}

#[test]
fn tensor_iter_mut_30() {
    const DIMENSION: usize = 3;
    const SITE: usize = 0;
    let mut reg = (0..1 << DIMENSION).collect::<Vec<_>>();

    let result =
        TensorIterMut::new(&mut reg, SITE, DIMENSION).collect::<Vec<_>>();

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

    let result =
        TensorIterMut::new(&mut reg, SITE, DIMENSION).collect::<Vec<_>>();

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

    let result =
        TensorIterMut::new(&mut reg, SITE, DIMENSION).collect::<Vec<_>>();

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

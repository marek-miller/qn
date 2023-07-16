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
    ) -> Self {
        assert!(buf.len() > 1 << site);
        Self {
            buf,
            lower_bits: 1usize << site,
            upper_bits: buf.len() / (1usize << (site + 1)),
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
    ) -> Self {
        assert!(buf.len() > 1 << site);
        let upper_bits = buf.len() / (1usize << (site + 1));

        Self {
            buf,
            lower_bits: 1usize << site,
            upper_bits,
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

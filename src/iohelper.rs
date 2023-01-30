use core::iter::Iterator;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ReadAsIter<P, const PROBE_SIZE: usize>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: P,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    ptr: usize,
}

impl<P, const PROBE_SIZE: usize> ReadAsIter<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    pub fn new(probe_fn: P) -> Self {
        Self {
            probe_fn,
            buffer: [0; PROBE_SIZE],
            len: 0,
            ptr: 0,
        }
    }
}

impl<P, const PROBE_SIZE: usize> Iterator for ReadAsIter<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.ptr == self.len {
            match (self.probe_fn)(&mut self.buffer) {
                None => return None,
                Some(l) if l == 0 => return None,
                Some(l) => self.len = l,
            }

            self.ptr = 0;
        }

        let byte = self.buffer[self.ptr];

        self.ptr += 1;

        Some(byte)
    }
}

#[test]
fn read_as_iter_test() {
    use pretty_assertions::assert_eq;
    use std::io::{Cursor, Read};

    fn run<const PROBE_SIZE: usize>(src: &[u8]) -> Vec<u8> {
        let mut reader = Cursor::new(src);

        let r: ReadAsIter<_, PROBE_SIZE> = ReadAsIter::new(|mut buf| {
            reader.read(&mut buf).ok()
        });

        r.collect()
    }

    // zero-size input
    assert_eq!(run::<1>(b""), b"");

    // zero-size input
    assert_eq!(run::<16>(b""), b"");

    // non-buffered input
    assert_eq!(run::<1>(b"Hello!"), b"Hello!");

    // just-fit input
    assert_eq!(run::<6>(b"Hello!"), b"Hello!");

    // N input
    assert_eq!(run::<3>(b"Hello!"), b"Hello!");

    // less input
    assert_eq!(run::<16>(b"Hello!"), b"Hello!");

    // N+less input
    assert_eq!(run::<4>(b"Hello!"), b"Hello!");
}

/// A bit iterator cap for the [`Iterator`] of [`u8`].
pub trait BitIterCap<T>
where
    T: IntoIterator<Item = u8>,
{
    fn into_bit_iter(self) -> ByteIterAsBitIter<T>;
}

impl<T> BitIterCap<T> for T
where
    T: IntoIterator<Item = u8>,
{
    fn into_bit_iter(self) -> ByteIterAsBitIter<Self>
    where
        Self: Sized,
    {
        ByteIterAsBitIter::new(self)
    }
}

pub struct ByteIterAsBitIter<T>
where
    T: IntoIterator<Item = u8>,
{
    pub provider: T::IntoIter,
    pub ptr: u8,
    pub buffer: u8,
}

impl<T> ByteIterAsBitIter<T>
where
    T: IntoIterator<Item = u8>,
{
    fn new(provider: T) -> Self {
        Self {
            provider: provider.into_iter(),
            ptr: 8,
            buffer: 0,
        }
    }
}

impl<T> Iterator for ByteIterAsBitIter<T>
where
    T: IntoIterator<Item = u8>,
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == 8 {
            self.buffer = self.provider.next()?;
            self.ptr = 0;
        }

        let bit = (self.buffer & (1 << self.ptr)) != 0;
        self.ptr += 1;
        Some(bit)
    }
}

#[test]
fn byte_iter_as_bit_iter_test() {
    use pretty_assertions::assert_eq;

    let input = [0x55, 0xAA];
    let expected = [
        true, false, true, false, true, false, true, false, false, true, false, true, false, true,
        false, true,
    ];

    let run = |src: &[u8]| -> Vec<bool> { src.iter().copied().into_bit_iter().collect() };

    assert_eq!(run(&[]), &[]);
    assert_eq!(run(&input[..1]), &expected[..8]);
    assert_eq!(run(&input), &expected);
}

pub trait ByteIterCap<T>
where
    T: IntoIterator<Item = bool>,
{
    fn into_byte_iter(self) -> BitIterAsByteIter<T>;
}

impl<T> ByteIterCap<T> for T
where
    T: IntoIterator<Item = bool>,
{
    fn into_byte_iter(self) -> BitIterAsByteIter<Self>
    where
        Self: Sized,
    {
        BitIterAsByteIter::new(self)
    }
}

pub struct BitIterAsByteIter<T>
where
    T: IntoIterator<Item = bool>,
{
    pub provider: T::IntoIter,
}

impl<T> BitIterAsByteIter<T>
where
    T: IntoIterator<Item = bool>,
{
    pub fn new(provider: T) -> Self {
        Self {
            provider: provider.into_iter(),
        }
    }
}

impl<T> Iterator for BitIterAsByteIter<T>
where
    T: IntoIterator<Item = bool>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v: u8 = 0;

        for n in 0..8 {
            match self.provider.next() {
                Some(false) => continue,
                Some(true) => v |= 1 << n,
                None if n == 0 => return None,
                None => break,
            }
        }

        Some(v)
    }
}

#[test]
fn bit_iter_as_byte_iter_test() {
    use pretty_assertions::assert_eq;

    let input = [
        true, false, true, false, true, false, true, false, false, true, false, true, false, true,
        false, true,
    ];

    let run = |src: &[bool]| -> Vec<u8> { src.iter().copied().into_byte_iter().collect() };

    assert_eq!(run(&[]), &[]);
    assert_eq!(run(&input[..1]), &[0x01]);
    assert_eq!(run(&input[..4]), &[0x05]);
    assert_eq!(run(&input[..8]), &[0x55]);
    assert_eq!(run(&input[..12]), &[0x55, 0x0A]);
    assert_eq!(run(&input[..16]), &[0x55, 0xAA]);
}

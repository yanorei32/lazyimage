use core::iter::Iterator;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ByteIter<P, const PROBE_SIZE: usize>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: P,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    ptr: usize,
}

impl<P, const PROBE_SIZE: usize> ByteIter<P, PROBE_SIZE>
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

impl<P, const PROBE_SIZE: usize> Iterator for ByteIter<P, PROBE_SIZE>
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

pub trait BitCap<T>
where
    T: IntoIterator<Item = u8>,
{
    fn bits(self) -> BitIter<T>;
}

impl<T> BitCap<T> for T
where
    T: IntoIterator<Item = u8>,
{
    fn bits(self) -> BitIter<Self>
    where
        Self: Sized,
    {
        BitIter::new(self)
    }
}

pub struct BitIter<T>
where
    T: IntoIterator<Item = u8>,
{
    pub provider: T::IntoIter,
    pub ptr: u8,
    pub buffer: u8,
}

impl<T> BitIter<T>
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

impl<T> Iterator for BitIter<T>
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

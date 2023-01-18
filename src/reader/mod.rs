use core::iter::Iterator;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BufToByte<P, const PROBE_SIZE: usize>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: P,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    ptr: usize,
}

impl<P, const PROBE_SIZE: usize> BufToByte<P, PROBE_SIZE>
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

impl<P, const PROBE_SIZE: usize> Iterator for BufToByte<P, PROBE_SIZE>
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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BufToBit<P, const PROBE_SIZE: usize>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: P,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    bitptr: usize,
}

impl<P, const PROBE_SIZE: usize> BufToBit<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    pub fn new(probe_fn: P) -> Self {
        Self {
            probe_fn,
            buffer: [0; PROBE_SIZE],
            len: 0,
            bitptr: 0,
        }
    }
}

impl<P, const PROBE_SIZE: usize> Iterator for BufToBit<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Option<usize>,
{
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.bitptr * 8 == self.len {
            match (self.probe_fn)(&mut self.buffer) {
                None => return None,
                Some(l) if l == 0 => return None,
                Some(l) => self.len = l,
            }

            self.bitptr = 0;
        }

        let ptr = self.bitptr / 8;
        let bit_idx = self.bitptr % 8;
        let bit = (self.buffer[ptr] & (1 << bit_idx)) != 0;

        self.bitptr += 1;

        Some(bit)
    }
}

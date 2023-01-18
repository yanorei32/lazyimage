use crate::interface::{Error, ByteProvider, BitProvider};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BufToByte<P, const PROBE_SIZE: usize>
where
    P: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: P,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    ptr: usize,
}

impl<P, const PROBE_SIZE: usize> BufToByte<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Result<usize, Error>,
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

impl<P, const PROBE_SIZE: usize> ByteProvider for BufToByte<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    fn next_byte(&mut self) -> Result<u8, Error> {
        if self.ptr == self.len {
            self.len = (self.probe_fn)(&mut self.buffer)?;

            if self.len == 0 {
                return Err(Error::UnexpectedEOF);
            }

            self.ptr = 0;
        }

        let byte = self.buffer[self.ptr];

        self.ptr += 1;

        Ok(byte)
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct BufToBit<P, const PROBE_SIZE: usize>
where
    P: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: P,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    bitptr: usize,
}

impl<P, const PROBE_SIZE: usize> BufToBit<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Result<usize, Error>,
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

impl<P, const PROBE_SIZE: usize> BitProvider for BufToBit<P, PROBE_SIZE>
where
    P: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    fn next_bit(&mut self) -> Result<bool, Error> {
        if self.bitptr * 8 == self.len {
            self.len = (self.probe_fn)(&mut self.buffer)?;

            if self.len == 0 {
                return Err(Error::UnexpectedEOF);
            }

            self.bitptr = 0;
        }

        let ptr = self.bitptr / 8;
        let bit_idx = self.bitptr % 8;
        let bit = (self.buffer[ptr] & (1 << bit_idx)) != 0;

        self.bitptr += 1;

        Ok(bit)
    }
}

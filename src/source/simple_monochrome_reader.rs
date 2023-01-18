use crate::interface::{Color, Error, ImageProvider, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SimpleMonochromeReader<T, const PROBE_SIZE: usize>
where
    T: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    probe_fn: T,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    bitptr: usize,
}

impl<T, const PROBE_SIZE: usize> SimpleMonochromeReader<T, PROBE_SIZE>
where
    T: FnMut(&mut [u8]) -> Result<usize, Error>,
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

    pub fn new(size: Size, probe_fn: T) -> Self {
        Self {
            size,
            probe_fn,
            buffer: [0; PROBE_SIZE],
            len: 0,
            bitptr: 0,
        }
    }
}

impl<T, const PROBE_SIZE: usize> ImageProvider for SimpleMonochromeReader<T, PROBE_SIZE>
where
    T: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        Ok(match self.next_bit()? {
            true => Color::Black,
            false => Color::White,
        })
    }
}

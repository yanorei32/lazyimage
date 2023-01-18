use crate::interface::{Color, Error, ImageProvider, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SimpleTextReader<T, const PROBE_SIZE: usize>
where
    T: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    probe_fn: T,
    buffer: [u8; PROBE_SIZE],
    len: usize,
    ptr: usize,
}

impl<T, const PROBE_SIZE: usize> SimpleTextReader<T, PROBE_SIZE>
where
    T: FnMut(&mut [u8]) -> Result<usize, Error>,
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

    pub fn new(size: Size, probe_fn: T) -> Self {
        Self {
            size,
            probe_fn,
            buffer: [0; PROBE_SIZE],
            len: 0,
            ptr: 0,
        }
    }
}

impl<T, const PROBE_SIZE: usize> ImageProvider for SimpleTextReader<T, PROBE_SIZE>
where
    T: FnMut(&mut [u8]) -> Result<usize, Error>,
{
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        loop {
            match self.next_byte()? {
                b'B' => return Ok(Color::Black),
                b'W' => return Ok(Color::White),
                b'T' => return Ok(Color::Third),
                b' ' => return Ok(Color::Transpalent),
                _ => continue,
            }
        }
    }
}

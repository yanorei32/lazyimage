use crate::interface::{BufferProber, Error};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct CallbackBufferProber<T, const BUFSIZE: usize>
where
    T: FnMut(&mut [u8; BUFSIZE]) -> Result<usize, Error>,
{
    #[derivative(Debug = "ignore")]
    probe_fn: T,
    buffer: [u8; BUFSIZE],
}

impl<T, const BUFSIZE: usize> CallbackBufferProber<T, BUFSIZE>
where
    T: FnMut(&mut [u8; BUFSIZE]) -> Result<usize, Error>,
{
    pub fn new(probe_fn: T) -> Self {
        Self {
            probe_fn,
            buffer: [0u8; BUFSIZE],
        }
    }
}

impl<T, const BUFSIZE: usize> BufferProber for CallbackBufferProber<T, BUFSIZE>
where
    T: FnMut(&mut [u8; BUFSIZE]) -> Result<usize, Error>,
{
    fn probe(&mut self) -> Result<(&[u8], usize), Error> {
        let size = (self.probe_fn)(&mut self.buffer)?;
        Ok((&self.buffer, size))
    }
}

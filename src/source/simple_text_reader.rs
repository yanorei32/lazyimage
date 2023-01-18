use crate::interface::{Color, Error, ImageProvider, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SimpleTextReader<P>
where
    P: Iterator<Item = u8>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> SimpleTextReader<P>
where
    P: Iterator<Item = u8>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> ImageProvider for SimpleTextReader<P>
where
    P: Iterator<Item = u8>,
{
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        loop {
            match self.provider.next() {
                Some(b'B') => return Ok(Color::Black),
                Some(b'W') => return Ok(Color::White),
                Some(b'T') => return Ok(Color::Third),
                Some(b' ') => return Ok(Color::Transpalent),
                Some(_) => continue,
                None => return Err(Error::UnexpectedEOF),
            }
        }
    }
}

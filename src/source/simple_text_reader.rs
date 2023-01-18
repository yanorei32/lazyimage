use crate::interface::{ByteProvider, Color, Error, ImageProvider, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SimpleTextReader<P>
where
    P: ByteProvider,
{
    size: Size,
    provider: P,
}

impl<P> SimpleTextReader<P>
where
    P: ByteProvider,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> ImageProvider for SimpleTextReader<P>
where
    P: ByteProvider,
{
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        loop {
            match self.provider.next_byte()? {
                b'B' => return Ok(Color::Black),
                b'W' => return Ok(Color::White),
                b'T' => return Ok(Color::Third),
                b' ' => return Ok(Color::Transpalent),
                _ => continue,
            }
        }
    }
}

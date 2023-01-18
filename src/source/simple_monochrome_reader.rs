use crate::interface::{BitProvider, Color, Error, ImageProvider, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SimpleMonochromeReader<P>
where
    P: BitProvider,
{
    size: Size,
    provider: P,
}

impl<P> SimpleMonochromeReader<P>
where
    P: BitProvider,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> ImageProvider for SimpleMonochromeReader<P>
where
    P: BitProvider
{
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        Ok(match self.provider.next_bit()? {
            true => Color::Black,
            false => Color::White,
        })
    }
}

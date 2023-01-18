use crate::interface::{Color, Error, ImageProvider, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SimpleMonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> SimpleMonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> ImageProvider for SimpleMonochromeReader<P>
where
    P: Iterator<Item = bool>
{
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        match self.provider.next() {
            Some(true) => Ok(Color::Black),
            Some(false) => Ok(Color::White),
            None => Err(Error::UnexpectedEOF),
        }
    }
}

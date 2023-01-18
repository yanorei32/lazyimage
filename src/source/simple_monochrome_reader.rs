use crate::interface::{Color, ImageProvider, Size};
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

impl<P> Iterator for SimpleMonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Color;
    fn next(&mut self) -> Option<Color> {
        Some(match self.provider.next()? {
            true => Color::Black,
            false => Color::White,
        })
    }
}

impl<P> ImageProvider for SimpleMonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    fn get_size(&self) -> Size {
        self.size
    }
}

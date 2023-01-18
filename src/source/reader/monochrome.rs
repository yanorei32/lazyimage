use crate::interface::{Color, Image, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> MonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> Iterator for MonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Color;
    fn next(&mut self) -> Option<Color> {
        self.provider.next().map(|v| match v {
            true => Color::Black,
            false => Color::White,
        })
    }
}

impl<P> Image for MonochromeReader<P>
where
    P: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}


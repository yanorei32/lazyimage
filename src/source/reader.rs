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

#[derive(Derivative)]
#[derivative(Debug)]
pub struct TextReader<P>
where
    P: Iterator<Item = u8>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> TextReader<P>
where
    P: Iterator<Item = u8>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> Iterator for TextReader<P>
where
    P: Iterator<Item = u8>,
{
    type Item = Color;
    fn next(&mut self) -> Option<Color> {
        loop {
            match self.provider.next()? {
                b'B' => return Some(Color::Black),
                b'W' => return Some(Color::White),
                b'T' => return Some(Color::Third),
                b' ' => return Some(Color::Transpalent),
                _ => continue,
            }
        }
    }
}

impl<P> Image for TextReader<P>
where
    P: Iterator<Item = u8>,
{
    fn size(&self) -> Size {
        self.size
    }
}

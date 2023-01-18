use crate::interface::{Color, ImageProvider, Size};
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

impl<P> Iterator for SimpleTextReader<P>
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

impl<P> ImageProvider for SimpleTextReader<P>
where
    P: Iterator<Item = u8>,
{
    fn get_size(&self) -> Size {
        self.size
    }
}

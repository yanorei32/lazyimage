use crate::interface::{Color, Image, Size};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self { size, provider }
    }
}

impl<P> Iterator for FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Color;
    fn next(&mut self) -> Option<Color> {
        Some(match (self.provider.next()?, self.provider.next()?) {
            (false, false) => Color::White,
            (false, true) => Color::Black,
            (true, false) => Color::Third,
            (true, true) => Color::Transpalent,
        })
    }
}

impl<P> Image for FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}

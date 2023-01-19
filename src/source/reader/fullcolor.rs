use crate::interface::{Cutout, FullColor, Image, Size};
use crate::utility::CanvasIterator;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            provider,
        }
    }
}

impl<P> Iterator for FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Cutout<FullColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        Some(match (self.provider.next()?, self.provider.next()?) {
            (false, false) => Cutout::Opaque(FullColor::White),
            (false, true) => Cutout::Opaque(FullColor::Black),
            (true, false) => Cutout::Opaque(FullColor::Third),
            (true, true) => Cutout::Cutout,
        })
    }
}

impl<P> Image<Cutout<FullColor>> for FullcolorReader<P>
where
    P: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}

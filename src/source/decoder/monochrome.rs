use crate::canvas::CanvasIterator;
use crate::color::{Cutout, MonoColor};
use crate::image::Image;
use crate::math::*;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MonochromeDecoder<P>
where
    P: Iterator<Item = bool>,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> MonochromeDecoder<P>
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

impl<P> Iterator for MonochromeDecoder<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Cutout<MonoColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        self.provider.next().map(|v| {
            if v {
                Cutout::Opaque(MonoColor::Black)
            } else {
                Cutout::Cutout
            }
        })
    }
}

impl<Provider> Image<Cutout<MonoColor>> for MonochromeDecoder<Provider>
where
    Provider: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}

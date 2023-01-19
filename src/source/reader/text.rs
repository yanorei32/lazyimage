use crate::interface::{Cutout, FullColor, Image, Size};
use crate::utility::CanvasIterator;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct TextReader<P>
where
    P: Iterator<Item = u8>,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> TextReader<P>
where
    P: Iterator<Item = u8>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            provider,
        }
    }
}

impl<P> Iterator for TextReader<P>
where
    P: Iterator<Item = u8>,
{
    type Item = Cutout<FullColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        loop {
            match self.provider.next()? {
                b'B' => return Some(Cutout::Opaque(FullColor::Black)),
                b'W' => return Some(Cutout::Opaque(FullColor::White)),
                b'T' => return Some(Cutout::Opaque(FullColor::Third)),
                b' ' => return Some(Cutout::Cutout),
                _ => continue,
            }
        }
    }
}

impl<P> Image<Cutout<FullColor>> for TextReader<P>
where
    P: Iterator<Item = u8>,
{
    fn size(&self) -> Size {
        self.size
    }
}

use crate::interface::{Cutout, MonoColor, Image, Size};
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
    type Item = Cutout<MonoColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.provider.next().map(|v| match v {
            true => Cutout::Opaque(MonoColor::Black),
            false => Cutout::Cutout,
        })
    }
}

impl<Provider> Image<Cutout<MonoColor>> for MonochromeReader<Provider>
where
    Provider: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}

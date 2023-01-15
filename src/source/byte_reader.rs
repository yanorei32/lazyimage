use crate::interface::{RawImageByteProvider, Color, ImageProvider, Size, Error};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RawByteSource<T, F> {
    provider: T,
    #[derivative(Debug = "ignore")]
    processor: F,
}

impl<T: RawImageByteProvider, F: Fn(&mut T) -> Result<Color, Error>> RawByteSource<T, F> {
    pub fn new(provider: T, processor: F) -> Self {
        Self {
            provider,
            processor,
        }
    }
}

impl<T: RawImageByteProvider, F: Fn(&mut T) -> Result<Color, Error>> ImageProvider for RawByteSource<T, F> {
    fn next(&mut self) -> Result<Color, Error> {
        (self.processor)(&mut self.provider)
    }

    fn get_size(&self) -> Size {
        self.provider.get_size()
    }
}

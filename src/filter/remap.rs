use crate::interface::{Color, Image, Size};
use core::iter::Iterator;
use enum_map::{enum_map, EnumMap};

type Remap = EnumMap<Color, Color>;

#[derive(Debug)]
pub struct RemapBuilder(Remap);

impl Default for RemapBuilder {
    fn default() -> Self {
        Self(enum_map! {
            Color::Third => Color::Third,
            Color::White => Color::White,
            Color::Black => Color::Black,
            Color::Transpalent => Color::Transpalent,
        })
    }
}

impl RemapBuilder {
    #[must_use]
    pub fn set_flip(mut self, a: Color, b: Color) -> Self {
        self.0[a] = b;
        self.0[b] = a;
        self
    }

    #[must_use]
    pub fn set_map(mut self, from: Color, to: Color) -> Self {
        self.0[from] = to;
        self
    }

    #[must_use]
    pub fn build(self) -> Remap {
        self.0
    }
}

#[derive(Debug)]
pub struct RemappedImage<T> {
    image: T,
    remap: Remap,
}

impl<T> RemappedImage<T>
where
    T: Image,
{
    #[must_use]
    pub fn new(image: T, remap: Remap) -> Self {
        Self { image, remap }
    }
}

impl<T> Image for RemappedImage<T>
where
    T: Image,
{
    fn size(&self) -> Size {
        self.image.size()
    }
}

impl<T> Iterator for RemappedImage<T>
where
    T: Image,
{
    type Item = Color;

    fn next(&mut self) -> Option<Color> {
        Some(self.remap[self.image.next()?])
    }
}

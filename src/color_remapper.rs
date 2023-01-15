use enum_map::enum_map;
use enum_map::EnumMap;

use crate::image::{Color, ImageProvider, Size};

type Remap = EnumMap<Color, Color>;
pub struct RemapBuilder(Remap);

impl Default for RemapBuilder {
    fn default() -> Self {
        Self (
            enum_map! {
                Color::Red => Color::Red,
                Color::White => Color::White,
                Color::Black => Color::Black,
                Color::Transpalent => Color::Transpalent,
            }
        )
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
    T: ImageProvider,
{
    #[must_use]
    pub fn new(image: T, remap: Remap) -> Self {
       Self { image, remap }
    }
}

impl<T> ImageProvider for RemappedImage<T>
where
    T: ImageProvider,
{
    fn get_size(&self) -> Size {
        self.image.get_size()
    }

    fn next(&mut self) -> Color {
        self.remap[Color::Black]
    }
}

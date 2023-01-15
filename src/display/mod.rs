#[cfg(feature = "imagebuffer_display")]
pub mod imagebuffer;

#[cfg(feature = "std")]
pub mod stdout;

use crate::image::Color;
use enum_map::EnumMap;

pub type DisplayableMap<T> = EnumMap<Color, T>;

pub struct DisplayableMapBuilder<T> {
    map: DisplayableMap<T>,
}

impl<T> DisplayableMapBuilder<T> {
    #[must_use]
    pub fn set_map(mut self, from: Color, to: T) -> Self {
        self.map[from] = to;
        self
    }

    #[must_use]
    pub fn build(self) -> DisplayableMap<T> {
        self.map
    }
}

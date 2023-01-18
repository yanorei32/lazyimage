use crate::interface::{Color, Image, Size};
use core::iter::Iterator;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Rect {
    size: Size,
    color: Color,
}

impl Rect {
    #[must_use]
    pub fn new(size: Size, color: Color) -> Self {
        Self { size, color }
    }
}

impl Iterator for Rect {
    type Item = Color;
    fn next(&mut self) -> Option<Color> {
        Some(self.color)
    }
}

impl Image for Rect {
    fn size(&self) -> Size {
        self.size
    }
}

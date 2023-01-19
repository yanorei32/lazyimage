use crate::interface::{Image, Size};
use crate::utility::CanvasIterator;
use core::fmt::Debug;
use core::iter::Iterator;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Rect<Color>
where
    Color: Copy,
{
    ptr: CanvasIterator,
    size: Size,
    color: Color,
}

impl<Color> Rect<Color>
where
    Color: Copy,
{
    #[must_use]
    pub fn new(size: Size, color: Color) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            color,
        }
    }
}

impl<Color> Iterator for Rect<Color>
where
    Color: Copy,
{
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        Some(self.color)
    }
}

impl<Color> Image<Color> for Rect<Color>
where
    Color: Debug + Copy,
{
    fn size(&self) -> Size {
        self.size
    }
}

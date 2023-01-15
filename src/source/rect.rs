use crate::interface::{Color, ImageProvider, Size};

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

impl ImageProvider for Rect {
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Color {
        self.color
    }
}

use crate::image::{Color, ImageProvider, Size};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct FilledImage {
    size: Size,
    color: Color,
}

impl FilledImage {
    #[must_use]
    pub fn new(size: Size, color: Color) -> Self {
        Self { size, color }
    }
}

impl ImageProvider for FilledImage {
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Color {
        self.color
    }
}

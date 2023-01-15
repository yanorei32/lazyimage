use crate::image::{Color, ImageProvider, Size};
use alloc::{boxed::Box, vec::Vec};

struct Layer {
    image: Box<dyn ImageProvider>,
    pos: Size,
}

#[allow(clippy::module_name_repetitions)]
pub struct LayeredImage {
    current_pos: Size,
    background: Color,
    size: Size,
    layers: Vec<Layer>,
}

#[allow(clippy::module_name_repetitions)]
pub struct LayeredImageBuilder {
    background: Color,
    size: Size,
    layers: Vec<Layer>,
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    HorizontalOverflow,
}

impl LayeredImageBuilder {
    #[must_use]
    pub fn new(size: Size) -> Self {
        Self {
            size,
            background: Color::Black,
            layers: Vec::new(),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `image` overflowed horizontal.
    pub fn add_layer(mut self, image: Box<dyn ImageProvider>, pos: Size) -> Result<Self, Error> {
        if pos.w + image.get_size().w > self.size.w {
            return Err(Error::HorizontalOverflow);
        }

        self.layers.push(Layer { image, pos });

        Ok(self)
    }

    #[must_use]
    pub fn set_bg(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    #[must_use]
    pub fn build(self) -> LayeredImage {
        LayeredImage {
            current_pos: Size { w: 0, h: 0 },
            size: self.size,
            layers: self.layers,
            background: self.background,
        }
    }
}

impl LayeredImage {}

impl ImageProvider for LayeredImage {
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Color {
        let mut c = self.current_pos;

        let mut color = self.background;

        for layer in &mut self.layers {
            if layer.pos.h > c.h {
                continue;
            }

            if c.h >= layer.pos.h + layer.image.get_size().h {
                continue;
            }

            if layer.pos.w > c.w {
                continue;
            }

            if c.w >= layer.pos.w + layer.image.get_size().w {
                continue;
            }

            let c = layer.image.next();

            if c != Color::Transpalent {
                color = c;
            };
        }

        if c.w == self.size.w - 1 {
            c.w = 0;
            c.h += 1;
        } else {
            c.w += 1;
        }

        self.current_pos = c;

        color
    }
}

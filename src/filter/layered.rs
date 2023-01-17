use crate::interface::{Area, Color, Error, ImageProvider, Size};

#[cfg(feature = "alloc")]
pub(crate) extern crate alloc;
use alloc::{boxed::Box, vec::Vec};

use core::fmt::Debug;

#[derive(Debug)]
struct Layer {
    image: Box<dyn ImageProvider>,
    area: Area,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct LayeredImage {
    current_pos: Size,
    size: Size,
    layers: Vec<Layer>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct LayeredImageBuilder {
    size: Size,
    layers: Vec<Layer>,
}

impl LayeredImageBuilder {
    #[must_use]
    pub fn new(size: Size) -> Self {
        Self {
            size,
            layers: Vec::new(),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `image` overflowed horizontal.
    pub fn add_layer(mut self, image: Box<dyn ImageProvider>, pos: Size) -> Result<Self, Error> {
        let size = image.get_size();

        if pos.w + size.w > self.size.w {
            return Err(Error::HorizontalOverflowIsDetected);
        }

        self.layers.push(Layer {
            image,
            area: Area::from_pos_size(pos, size),
        });

        Ok(self)
    }

    #[must_use]
    pub fn build(self) -> LayeredImage {
        LayeredImage {
            current_pos: Size { w: 0, h: 0 },
            size: self.size,
            layers: self.layers,
        }
    }
}

impl LayeredImage {}

impl ImageProvider for LayeredImage {
    fn get_size(&self) -> Size {
        self.size
    }

    fn next(&mut self) -> Result<Color, Error> {
        let mut c = self.current_pos;

        let mut color = Color::Transpalent;

        for layer in &mut self.layers {
            if !layer.area.contains(c) {
                continue;
            }

            let c = layer.image.next()?;

            if c != Color::Transpalent {
                color = c;
            }
        }

        if c.w == self.size.w - 1 {
            c.w = 0;
            c.h += 1;
        } else {
            c.w += 1;
        }

        self.current_pos = c;

        Ok(color)
    }
}

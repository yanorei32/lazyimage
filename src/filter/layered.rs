use crate::interface::{Area, Color, Error, Image, Size};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;
use core::iter::Iterator;

#[derive(Debug)]
struct Layer {
    image: Box<dyn Image>,
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
    pub fn add_layer(mut self, image: Box<dyn Image>, pos: Size) -> Result<Self, Error> {
        let size = image.size();

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

impl Iterator for LayeredImage {
    type Item = Color;

    fn next(&mut self) -> Option<Color> {
        let mut color = Color::Transpalent;

        for layer in &mut self.layers {
            if !layer.area.contains(self.current_pos) {
                continue;
            }

            match layer.image.next()? {
                Color::Transpalent => {}
                opaque => color = opaque,
            }
        }

        self.current_pos = match self.current_pos {
            // if end of line
            p if p.w == self.size.w - 1 => Size { w: 0, h: p.h + 1 },

            // otherwise
            p => Size { w: p.w + 1, h: p.h },
        };

        Some(color)
    }
}

impl Image for LayeredImage {
    fn size(&self) -> Size {
        self.size
    }
}

use crate::interface::{Area, Cutout, Error, Image, Size};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;
use core::iter::Iterator;

#[derive(Debug)]
struct Layer<Color> {
    image: Box<dyn Image<Cutout<Color>>>,
    area: Area,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct LayeredImage<Color> {
    current_pos: Size,
    size: Size,
    layers: Vec<Layer<Color>>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct LayeredImageBuilder<Color> {
    size: Size,
    layers: Vec<Layer<Color>>,
}

impl<Color> LayeredImageBuilder<Color>
where
    Color: Debug + Copy,
{
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
    pub fn add_layer(
        mut self,
        image: Box<dyn Image<Cutout<Color>>>,
        pos: Size,
    ) -> Result<Self, Error> {
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
    pub fn build(self) -> LayeredImage<Color> {
        LayeredImage {
            current_pos: Size { w: 0, h: 0 },
            size: self.size,
            layers: self.layers,
        }
    }
}

impl<Color> Iterator for LayeredImage<Color> {
    type Item = Cutout<Color>;

    fn next(&mut self) -> Option<Self::Item> {
        let color = self
            .layers
            .iter_mut()
            .filter(|l| l.area.contains(self.current_pos))
            .try_fold(Cutout::Cutout, |base, l| match l.image.next()? {
                Cutout::Cutout => Some(base),
                opaque => Some(opaque),
            })?;

        self.current_pos = match self.current_pos {
            // if end of line
            p if p.w == self.size.w - 1 => Size { w: 0, h: p.h + 1 },

            // otherwise
            p => Size { w: p.w + 1, h: p.h },
        };

        Some(color)
    }
}

impl<Color> Image<Cutout<Color>> for LayeredImage<Color>
where
    Color: Debug,
{
    fn size(&self) -> Size {
        self.size
    }
}

use crate::canvas::CanvasIterator;
use crate::color::Cutout;
use crate::image::Image;
use crate::math::Size;
use crate::source::Font;
use core::fmt::Debug;
use core::iter::Iterator;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct ScaledTextArea<const LENGTH: usize> {
    ptr: CanvasIterator,
    chars: [Font; LENGTH],
    buffer: [[Cutout<()>; 8]; LENGTH],
    scale: usize,
}

impl<const LENGTH: usize> ScaledTextArea<LENGTH> {
    #[must_use]
    pub fn new(ascii_codes: &[u8; LENGTH], scale: usize) -> Self {
        Self {
            ptr: CanvasIterator::new(Size::new((8 * LENGTH * scale) as u16, 16 * scale as u16)),
            chars: array_init::array_init(|i| Font::new(ascii_codes[i])),
            buffer: array_init::array_init(|_| array_init::array_init(|_| Default::default())),
            scale,
        }
    }
}

impl<const LENGTH: usize> Iterator for ScaledTextArea<LENGTH> {
    type Item = Cutout<()>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.ptr.next()?;

        if pos.w == 0 && (pos.h % self.scale as u16) == 0 {
            for n in 0..LENGTH {
                for w in 0..8 {
                    self.buffer[n][w] = self.chars[n].next().unwrap();
                }
            }
        }

        let w_scaled = pos.w / self.scale as u16;
        Some(self.buffer[(w_scaled / 8) as usize][(w_scaled % 8) as usize])
    }
}

impl<const LENGTH: usize> Image<Cutout<()>> for ScaledTextArea<LENGTH> {
    fn size(&self) -> Size {
        Size::new(
            8 * LENGTH as u16 * self.scale as u16,
            16 * self.scale as u16,
        )
    }
}

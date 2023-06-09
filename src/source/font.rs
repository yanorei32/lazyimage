use crate::canvas::CanvasIterator;
use crate::color::Cutout;
use crate::image::Image;
use crate::math::Size;
use core::fmt::Debug;
use core::iter::Iterator;

const BYTES_PER_CHAR: usize = 16;
const FONT: &[u8; BYTES_PER_CHAR * 95] = include_bytes!("../../assets/font8x16_remapped_1bpp.raw");

// 8x16 font
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Font {
    ptr: CanvasIterator,
    bitmap: &'static [u8],
}

impl Font {
    #[must_use]
    pub fn new(ascii_code: u8) -> Self {
        let pos = (ascii_code as usize - 0x20) * BYTES_PER_CHAR;
        Self {
            ptr: CanvasIterator::new(Size::new(8, 16)),
            bitmap: &FONT[pos..pos + BYTES_PER_CHAR],
        }
    }
}

impl Iterator for Font {
    type Item = Cutout<()>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.ptr.next()?;

        match self.bitmap[pos.h as usize] & (1 << 7 - pos.w) {
            0 => Some(Cutout::Cutout),
            _ => Some(Cutout::Opaque(())),
        }
    }
}

impl Image<Cutout<()>> for Font {
    fn size(&self) -> Size {
        Size::new(8, 16)
    }
}

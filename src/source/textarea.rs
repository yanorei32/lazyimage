use crate::canvas::CanvasIterator;
use crate::color::Cutout;
use crate::image::Image;
use crate::math::Size;
use crate::source::Font;
use core::fmt::Debug;
use core::iter::Iterator;

// 8x16 font
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TextArea<const LENGTH: usize> {
    ptr: CanvasIterator,
    chars: [Font; LENGTH],
}

impl<const LENGTH: usize> TextArea<LENGTH> {
    #[must_use]
    pub fn new(ascii_codes: &[u8; LENGTH]) -> Self {
        Self {
            ptr: CanvasIterator::new(Size::new(8 * LENGTH as u16, 16)),
            chars: array_init::array_init(|i| Font::new(ascii_codes[i])),
        }
    }
}

impl<const LENGTH: usize> Iterator for TextArea<LENGTH> {
    type Item = Cutout<()>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.ptr.next()?;
        self.chars[(pos.w / 8) as usize].next()
    }
}

impl<const LENGTH: usize> Image<Cutout<()>> for TextArea<LENGTH> {
    fn size(&self) -> Size {
        Size::new(8 * LENGTH as u16, 16)
    }
}

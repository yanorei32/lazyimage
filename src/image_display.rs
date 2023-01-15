use crate::image::{Color, ImageProvider};
use enum_map::enum_map;
use enum_map::EnumMap;
use image::ImageBuffer;

type DisplayableMap<P> = EnumMap<Color, P>;
pub struct DisplayableMapBuilder<P: image::Pixel>(DisplayableMap<P>);

impl<P> DisplayableMapBuilder<P>
where
    P: image::Pixel,
{
    #[must_use]
    pub fn new(transpalent: P, white: P, black: P, third: P) -> Self {
        Self(enum_map! {
            Color::Transpalent => transpalent,
            Color::White => white,
            Color::Black => black,
            Color::Third => third,
        })
    }

    pub fn build(self) -> DisplayableMap<P> {
        self.0
    }
}

pub trait ImageBufferDisplay<T> {
    fn create_imagebuffer<P: image::Pixel>(
        &mut self,
        map: DisplayableMap<P>,
    ) -> ImageBuffer<P, std::vec::Vec<P::Subpixel>>;
}

impl<T> ImageBufferDisplay<T> for T
where
    T: ImageProvider,
{
    fn create_imagebuffer<P: image::Pixel>(
        &mut self,
        map: DisplayableMap<P>,
    ) -> ImageBuffer<P, std::vec::Vec<P::Subpixel>> {
        let s = self.get_size();

        let mut buf = ImageBuffer::new(s.w.into(), s.h.into());

        for y in 0..s.h {
            for x in 0..s.w {
                buf.put_pixel(x.into(), y.into(), map[self.next()]);
            }
        }

        buf
    }
}

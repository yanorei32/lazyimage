use crate::display::DisplayableMap;
use crate::display::DisplayableMapBuilder;
use crate::interface::{Color, ImageProvider};
use enum_map::enum_map;
use image::ImageBuffer;

impl Default for DisplayableMapBuilder<image::Rgba<u8>> {
    fn default() -> Self {
        Self {
            map: enum_map! {
                Color::Transpalent => image::Rgba([0, 0, 0, 0]),
                Color::White => image::Rgba([192, 192, 192, 255]),
                Color::Black => image::Rgba([32, 32, 32, 255]),
                Color::Third => image::Rgba([192, 32, 32, 255]),
            },
        }
    }
}

impl Default for DisplayableMapBuilder<image::Rgb<u8>> {
    fn default() -> Self {
        Self {
            map: enum_map! {
                Color::Transpalent => image::Rgb([255, 0, 255] as [u8; 3]),
                Color::White => image::Rgb([192, 192, 192]),
                Color::Black => image::Rgb([32, 32, 32]),
                Color::Third => image::Rgb([192, 32, 32]),
            },
        }
    }
}

pub trait CreateImageBuffer<T> {
    fn create_imagebuffer<P: image::Pixel>(
        &mut self,
        map: DisplayableMap<P>,
    ) -> ImageBuffer<P, std::vec::Vec<P::Subpixel>>;
}

impl<T> CreateImageBuffer<T> for T
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

use crate::canvas::CanvasIterator;
use crate::image::Image;
use crate::math::Size;
use core::fmt::Debug;
use core::iter::Iterator;
use image::GenericImageView;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct ImageCrate<T: GenericImageView + Debug>
where
    T::Pixel: Debug,
{
    ptr: CanvasIterator,
    image: T,
}

impl<T: GenericImageView + Debug> ImageCrate<T>
where
    T::Pixel: Debug,
{
    #[must_use]
    pub fn new(image: T) -> Self {
        Self {
            ptr: CanvasIterator::new(Size::new(image.width() as u16, image.height() as u16)),
            image,
        }
    }
}

impl<T: GenericImageView + Debug> Iterator for ImageCrate<T>
where
    T::Pixel: Debug,
{
    type Item = T::Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.ptr.next()?;
        Some(self.image.get_pixel(pos.w as u32, pos.h as u32))
    }
}

impl<T: GenericImageView + Debug> Image<T::Pixel> for ImageCrate<T>
where
    T::Pixel: Debug,
{
    fn size(&self) -> Size {
        Size::new(self.image.width() as u16, self.image.height() as u16)
    }
}

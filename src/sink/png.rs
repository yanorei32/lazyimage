use crate::image::Image;
use core::fmt::Debug;

extern crate std;
use std::{borrow::ToOwned, boxed::Box, error::Error, path::Path, vec::Vec};

extern crate image;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageFormat, Pixel, Rgb};

pub trait Png<I, P>
where
    Self: Sized,
    I: Image<P> + Iterator<Item = P>,
    P: Into<Rgb<u8>> + Debug,
{
    /// # Errors
    /// An error is returned in the following cases.
    /// - when the PNG could not be saved by the image library.
    /// - if the iterator returns None.
    fn png_sink<Q>(self, path: Q, scale: u8) -> Result<(), Box<dyn Error>>
    where
        Q: AsRef<Path>;
}

impl<I, P> Png<I, P> for I
where
    Self: Sized,
    I: Image<P> + Iterator<Item = P>,
    P: Into<Rgb<u8>> + Debug,
{
    fn png_sink<Q>(
        self,
        path: Q,
        scale: u8,
    ) -> Result<(), Box<dyn Error>>
    where
        Q: AsRef<Path>,
    {
        let size = self.size();

        let pixels: Vec<u8> = self
            .map(core::convert::Into::into)
            .flat_map(|v| v.channels().to_owned())
            .collect();

        let buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_vec(size.w.into(), size.h.into(), pixels)
                .ok_or("Invalid buffer size")?;

        if scale == 1 {
            buffer.save_with_format(path, ImageFormat::Png)?;
        } else {
            DynamicImage::from(buffer)
                .resize(
                    u32::from(size.w) * u32::from(scale),
                    u32::from(size.h) * u32::from(scale),
                    FilterType::Nearest,
                )
                .save_with_format(path, ImageFormat::Png)?;
        }

        Ok(())
    }
}

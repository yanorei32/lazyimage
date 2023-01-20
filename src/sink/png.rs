use crate::interface::Image;
use core::fmt::Debug;

pub trait Png<I, P>
where
    Self: Sized,
    I: Image<P> + Iterator<Item = P>,
    P: Into<crate::image::Rgb<u8>> + Debug,
{
    fn png_sink<Q>(
        self,
        path: Q,
        scale: u8,
    ) -> Result<(), crate::std::boxed::Box<dyn crate::std::error::Error>>
    where
        Q: AsRef<crate::std::path::Path>;
}

impl<I, P> Png<I, P> for I
where
    Self: Sized,
    I: Image<P> + Iterator<Item = P>,
    P: Into<crate::image::Rgb<u8>> + Debug,
{
    /// # Errors
    /// image can cause error in save png
    fn png_sink<Q>(
        self,
        path: Q,
        scale: u8,
    ) -> Result<(), crate::std::boxed::Box<dyn crate::std::error::Error>>
    where
        Q: AsRef<crate::std::path::Path>,
    {
        use crate::image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageFormat, Pixel};
        use crate::std::{borrow::ToOwned, vec::Vec};

        let size = self.size();

        let pixels: crate::std::vec::Vec<u8> = self
            .map(core::convert::Into::into)
            .flat_map(|v| v.channels().to_owned())
            .collect();

        let buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
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

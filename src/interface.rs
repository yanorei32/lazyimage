use crate::filter::{overlay::OverlayedImage, remap::RemappedImage};
use core::{fmt::Debug, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub w: u16,
    pub h: u16,
}

impl From<Point> for Size {
    fn from(s: Point) -> Size {
        Size { w: s.w, h: s.h }
    }
}

impl Add for Size {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            w: self.w + rhs.w,
            h: self.h + rhs.h,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cutout<T> {
    Cutout,
    Opaque(T),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MonoColor {
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FullColor {
    White,
    Black,
    Third,
}

impl<F> Cutout<F> {
    pub fn convert_inner<T>(self) -> Cutout<T>
    where
        T: From<F>,
    {
        match self {
            Cutout::Cutout => Cutout::Cutout,
            Cutout::Opaque(v) => Cutout::Opaque(T::from(v)),
        }
    }
}

impl<T> From<T> for Cutout<T> {
    fn from(c: T) -> Cutout<T> {
        Cutout::Opaque(c)
    }
}

#[cfg(feature = "image")]
impl From<FullColor> for crate::image::Rgb<u8> {
    fn from(value: FullColor) -> Self {
        match value {
            FullColor::White => image::Rgb([192, 192, 192]),
            FullColor::Black => image::Rgb([32, 32, 32]),
            FullColor::Third => image::Rgb([192, 32, 32]),
        }
    }
}

#[cfg(feature = "image")]
impl From<Cutout<FullColor>> for crate::image::Rgb<u8> {
    fn from(value: Cutout<FullColor>) -> Self {
        match value {
            Cutout::Cutout => image::Rgb([255, 0, 255]),
            Cutout::Opaque(v) => v.into(),
        }
    }
}

impl From<MonoColor> for FullColor {
    fn from(c: MonoColor) -> FullColor {
        match c {
            MonoColor::Black => FullColor::Black,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Error {
    HorizontalOverflowIsDetected,
}

pub trait Image<P>: Iterator<Item = P> + Debug
where
    P: Debug,
{
    fn size(&self) -> Size;

    fn remap<F, B>(self, f: F) -> RemappedImage<Self, F, Self::Item, B>
    where
        Self: Sized,
        F: Fn(Self::Item) -> B,
        B: Debug,
    {
        RemappedImage::new(self, f)
    }

    /// # Errors
    /// `image_provider::interface::Error::HorizontalOverflowIsDetected` If src.horz < overlay.horz
    fn overlay<Overlay, OverlayColor>(
        self,
        pos: Point,
        overlay: Overlay,
    ) -> Result<OverlayedImage<Self, Self::Item, Overlay, OverlayColor>, Error>
    where
        Self: Sized,
        Overlay: Image<Cutout<OverlayColor>>,
        OverlayColor: Into<Self::Item> + Debug,
    {
        OverlayedImage::new(self, pos, overlay)
    }

    /// # Errors
    /// image can cause error in save png
    #[cfg(feature = "image")]
    fn png_sink<Q>(
        self,
        path: Q,
        scale: u8,
    ) -> Result<(), crate::std::boxed::Box<dyn crate::std::error::Error>>
    where
        Q: AsRef<crate::std::path::Path>,
        Self: Sized,
        Self::Item: Into<crate::image::Rgb<u8>>,
    {
        use crate::image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageFormat, Pixel};
        use crate::std::{borrow::ToOwned, vec::Vec};

        let size = self.size();

        let pixels: crate::std::vec::Vec<u8> = self
            .map(core::convert::Into::into)
            .flat_map(|v| v.channels().to_owned())
            .collect();

        let buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_vec(size.w.into(), size.h.into(), pixels).ok_or("Invalid size")?;

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

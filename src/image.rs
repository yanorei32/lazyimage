use crate::math::*;
use crate::color::*;
use core::fmt::Debug;

pub trait Image<P>: Iterator<Item = P> + Debug
where
    P: Debug,
{
    fn size(&self) -> Size;
}

impl From<MonoColor> for FullColor {
    fn from(c: MonoColor) -> FullColor {
        match c {
            MonoColor::Black => FullColor::Black,
        }
    }
}

#[cfg(feature = "image")]
impl From<FullColor> for image::Rgb<u8> {
    fn from(value: FullColor) -> Self {
        match value {
            FullColor::White => image::Rgb([192, 192, 192]),
            FullColor::Black => image::Rgb([32, 32, 32]),
            FullColor::Third => image::Rgb([192, 32, 32]),
        }
    }
}

#[cfg(feature = "image")]
impl From<Cutout<FullColor>> for image::Rgb<u8> {
    fn from(value: Cutout<FullColor>) -> Self {
        match value {
            Cutout::Cutout => image::Rgb([255, 0, 255]),
            Cutout::Opaque(v) => v.into(),
        }
    }
}

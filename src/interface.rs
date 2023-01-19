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

#[derive(Debug, Copy, Clone)]
pub enum Cutout<T> {
    Cutout,
    Opaque(T),
}

#[derive(Debug, Copy, Clone)]
pub enum MonoColor {
    Black,
}

#[derive(Debug, Copy, Clone)]
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
}

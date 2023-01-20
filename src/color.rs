use core::fmt::Debug;

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

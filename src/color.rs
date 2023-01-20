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

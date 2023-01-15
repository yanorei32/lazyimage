use core::fmt::Debug;
use enum_map::Enum;

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
pub enum Color {
    Transpalent,
    White,
    Black,
    Third,
}

pub trait ImageProvider: Debug {
    fn get_size(&self) -> Size;
    fn next(&mut self) -> Color;
}

#[cfg(feature = "byte_reader")]
pub trait RawImageByteProvider: Debug {
    fn get_size(&self) -> Size;
    fn next(&mut self) -> u8;
}

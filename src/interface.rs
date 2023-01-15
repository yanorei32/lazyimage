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

#[allow(clippy::module_name_repetitions)]
pub trait ImageProvider: Debug {
    fn get_size(&self) -> Size;
    fn next(&mut self) -> Color;
}

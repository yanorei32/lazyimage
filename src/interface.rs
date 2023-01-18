use core::fmt::Debug;
use core::ops::{Add, Range};
use enum_map::Enum;

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub w: u16,
    pub h: u16,
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

#[derive(Debug, Clone)]
pub struct Area {
    w: Range<u16>,
    h: Range<u16>,
}

impl Area {
    #[must_use] pub fn from_pos_size(pos: Size, size: Size) -> Self {
        Self {
            w: pos.w..pos.w + size.w,
            h: pos.h..pos.h + size.h,
        }
    }

    #[must_use] pub fn contains(&self, pos: Size) -> bool {
        self.w.contains(&pos.w) && self.h.contains(&pos.h)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
pub enum Color {
    Transpalent,
    White,
    Black,
    Third,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    RequestedPixelIsNotFound,
    RequestedU8IsNotFound,
    HorizontalOverflowIsDetected,
    BufferProbingError,
    UnexpectedEOF,
}

pub trait Image: Iterator<Item = Color> + Debug {
    fn size(&self) -> Size;
}

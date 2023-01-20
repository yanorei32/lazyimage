use core::{fmt::Debug, ops::Range};

#[derive(Debug, Clone)]
pub struct Area {
    w: Range<u16>,
    h: Range<u16>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

impl Area {
    #[must_use]
    pub fn new(pos: Point, size: Size) -> Self {
        Self {
            w: pos.w..pos.w + size.w,
            h: pos.h..pos.h + size.h,
        }
    }

    #[must_use]
    pub fn contains(&self, pos: Point) -> bool {
        self.w.contains(&pos.w) && self.h.contains(&pos.h)
    }
}

impl Point {
    #[must_use]
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }
}

impl Size {
    #[must_use]
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }

    pub fn is_zero(self) -> bool {
        self.w == 0 || self.h == 0
    }
}

use crate::interface::{Point, Size};
use core::{fmt::Debug, ops::Range};

#[derive(Debug)]
pub struct CanvasIterator {
    size: Size,
    pointer: Point,
}

impl CanvasIterator {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            pointer: Point { w: 0, h: 0 },
        }
    }
}

impl Iterator for CanvasIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.pointer;

        self.pointer = match self.pointer {
            // if end of file
            p if p.w == self.size.w - 1 && p.h == self.size.h  => return None,

            // if end of line
            p if p.w == self.size.w - 1 => Point { w: 0, h: p.h + 1 },

            // otherwise
            p => Point { w: p.w + 1, h: p.h },
        };

        Some(current)
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    w: Range<u16>,
    h: Range<u16>,
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

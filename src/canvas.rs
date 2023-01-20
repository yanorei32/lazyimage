use crate::math::{Size, Point};
use core::fmt::Debug;

#[derive(Debug)]
pub struct CanvasIterator {
    size: Size,
    pointer: Point,
}

impl CanvasIterator {
    #[must_use]
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

        if self.size.is_zero() {
            return None;
        }

        self.pointer = match self.pointer {
            // if end of file
            p if p.w == 0 && p.h == self.size.h => return None,

            // if end of line
            p if p.w == self.size.w - 1 => Point { w: 0, h: p.h + 1 },

            // otherwise
            p => Point { w: p.w + 1, h: p.h },
        };

        Some(current)
    }
}

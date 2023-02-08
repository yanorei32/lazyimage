use crate::math::{Point, Size};
use core::fmt::Debug;

/// A helper for implements Image.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct CanvasIterator {
    size: Size,
    pointer: Point,
}

impl CanvasIterator {
    #[must_use]
    pub fn new(size: Size) -> Self {
        Self {
            size,
            pointer: Point::new(0, 0),
        }
    }

    #[must_use]
    pub fn size(&self) -> Size {
        self.size
    }
}

impl Iterator for CanvasIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.pointer;

        if self.size.is_zero() {
            return None;
        }

        // NOTE:
        // If this feature is implemented in stable,
        //   I'll use this in the future.
        // https://github.com/rust-lang/rust/issues/43122
        self.pointer = match self.pointer {
            // if end of file
            p if p.w == 0 && p.h == self.size.h => return None,

            // if end of line
            p if p.w == self.size.w - 1 => Point::new(0, p.h + 1),

            // otherwise
            p => Point::new(p.w + 1, p.h),
        };

        Some(current)
    }
}

#[test]
fn canvas_iterator_test() {
    use pretty_assertions::assert_eq;

    let run = |s| -> Vec<Point> { CanvasIterator::new(s).collect() };

    let expected = |s: Size| -> Vec<Point> {
        (0..s.h)
            .flat_map(|h| (0..s.w).map(move |w| Point::new(w, h)))
            .collect()
    };

    // size 0
    assert_eq!(run(Size::new(0, 0)), expected(Size::new(0, 0)));
    assert_eq!(run(Size::new(1, 0)), expected(Size::new(1, 0)));
    assert_eq!(run(Size::new(0, 1)), expected(Size::new(0, 1)));

    // size 1
    assert_eq!(run(Size::new(1, 1)), expected(Size::new(1, 1)));

    // size N
    assert_eq!(run(Size::new(10, 5)), expected(Size::new(10, 5)));
}

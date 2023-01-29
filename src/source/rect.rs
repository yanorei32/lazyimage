use crate::canvas::CanvasIterator;
use crate::math::Size;
use crate::image::Image;
use core::fmt::Debug;
use core::iter::Iterator;

/// Primitive Rect
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Rect<Color>
where
    Color: Debug + Copy,
{
    ptr: CanvasIterator,
    size: Size,
    color: Color,
}

impl<Color> Rect<Color>
where
    Color: Debug + Copy,
{
    #[must_use]
    pub fn new(size: Size, color: Color) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            color,
        }
    }
}

impl<Color> Iterator for Rect<Color>
where
    Color: Debug + Copy,
{
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        Some(self.color)
    }
}

impl<Color> Image<Color> for Rect<Color>
where
    Color: Debug + Copy,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[test]
fn rect_test() {
    use pretty_assertions::assert_eq;

    let expected = [(), (), (), ()];
    let run = |s| -> Vec<()> { Rect::new(s, ()).collect() };

    // empty
    assert_eq!(run(Size { h: 0, w: 0 }), &[]);
    assert_eq!(run(Size { h: 0, w: 1 }), &[]);
    assert_eq!(run(Size { h: 1, w: 0 }), &[]);

    // 0
    assert_eq!(run(Size { h: 1, w: 1 }), &expected[..1]);

    // N
    assert_eq!(run(Size { h: 2, w: 2 }), &expected);
}

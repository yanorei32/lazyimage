use crate::canvas::CanvasIterator;
use crate::image::Image;
use crate::math::Size;
use core::fmt::Debug;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Passthrough<P, Color>
where
    P: IntoIterator<Item = Color>,
    Color: Debug,
{
    ptr: CanvasIterator,
    #[derivative(Debug = "ignore")]
    provider: P::IntoIter,
}

impl<P, Color> Passthrough<P, Color>
where
    P: IntoIterator<Item = Color>,
    Color: Debug,
{
    pub fn new(provider: P, size: Size) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            provider: provider.into_iter(),
        }
    }
}

impl<P, Color> Iterator for Passthrough<P, Color>
where
    P: IntoIterator<Item = Color>,
    Color: Debug,
{
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        self.provider.next()
    }
}

impl<P, Color> Image<Color> for Passthrough<P, Color>
where
    P: IntoIterator<Item = Color>,
    Color: Debug,
{
    fn size(&self) -> Size {
        self.ptr.size()
    }
}


#[test]
fn passthrough_test() {
    use pretty_assertions::assert_eq;

    let data = b"RB RB ";

    let run = |size, src: &[u8]| -> Vec<u8> {
        Passthrough::new(src.iter().copied(), size).collect()
    };

    // don't read
    assert_eq!(run(Size::new(0, 0), data), []);
    assert_eq!(run(Size::new(1, 0), data), []);
    assert_eq!(run(Size::new(0, 1), data), []);

    // empty input
    assert_eq!(run(Size::new(1, 1), &[]), []);

    // justfit
    assert_eq!(run(Size::new(3, 2), data), data);

    // (3 x 2) + remaining: 2
    assert_eq!(run(Size::new(2, 2), data), &data[..4]);

    // need more inputs.
    assert_eq!(run(Size::new(100, 100), data), data);
}

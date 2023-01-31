use crate::canvas::CanvasIterator;
use crate::image::Image;
use crate::math::Size;
use core::fmt::Debug;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TextDecoder<P, F, Color>
where
    P: IntoIterator<Item = u8>,
    F: Fn(u8) -> Option<Color>,
    Color: Debug,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    mapper: F,
    #[derivative(Debug = "ignore")]
    provider: P::IntoIter,
}

impl<P, F, Color> TextDecoder<P, F, Color>
where
    P: IntoIterator<Item = u8>,
    F: Fn(u8) -> Option<Color>,
    Color: Debug,
{
    pub fn new(provider: P, size: Size, mapper: F) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            provider: provider.into_iter(),
            mapper,
        }
    }
}

impl<P, F, Color> Iterator for TextDecoder<P, F, Color>
where
    P: IntoIterator<Item = u8>,
    F: Fn(u8) -> Option<Color>,
    Color: Debug,
{
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;

        loop {
            match (self.mapper)(self.provider.next()?) {
                Some(v) => return Some(v),
                None => continue,
            }
        }
    }
}

impl<P, F, Color> Image<Color> for TextDecoder<P, F, Color>
where
    P: IntoIterator<Item = u8>,
    F: Fn(u8) -> Option<Color>,
    Color: Debug,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[test]
fn textdecoder_test() {
    use pretty_assertions::assert_eq;

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum C {
        Blank,
        Black,
        Red,
    }

    let valid = b"RB RB ";
    let valid_with_trash = b"RB \r\nRB \r\n";
    let expected = [C::Red, C::Black, C::Blank, C::Red, C::Black, C::Blank];

    let run = |size, src: &[u8]| -> Vec<C> {
        TextDecoder::new(src.iter().copied(), size, |v| match v {
            b'R' => Some(C::Red),
            b'B' => Some(C::Black),
            b' ' => Some(C::Blank),
            _ => None,
        })
        .collect()
    };

    // don't read
    assert_eq!(run(Size::new(0, 0), valid), []);
    assert_eq!(run(Size::new(1, 0), valid), []);
    assert_eq!(run(Size::new(0, 1), valid), []);

    // empty input
    assert_eq!(run(Size::new(1, 1), &[]), []);

    // justfit
    assert_eq!(run(Size::new(3, 2), valid), expected);

    // with trash
    assert_eq!(run(Size::new(3, 2), valid_with_trash), expected);

    // (3 x 2) + remaining: 2
    assert_eq!(run(Size::new(2, 2), valid), &expected[..4]);

    // need more inputs.
    assert_eq!(run(Size::new(100, 100), valid), expected);
}

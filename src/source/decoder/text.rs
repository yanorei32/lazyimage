use crate::canvas::CanvasIterator;
use crate::color::{Cutout, FullColor};
use crate::image::Image;
use crate::math::Size;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TextDecoder<P>
where
    P: Iterator<Item = u8>,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> TextDecoder<P>
where
    P: Iterator<Item = u8>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            provider,
        }
    }
}

impl<P> Iterator for TextDecoder<P>
where
    P: Iterator<Item = u8>,
{
    type Item = Cutout<FullColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        loop {
            match self.provider.next()? {
                b'B' => return Some(Cutout::Opaque(FullColor::Black)),
                b'W' => return Some(Cutout::Opaque(FullColor::White)),
                b'T' => return Some(Cutout::Opaque(FullColor::Third)),
                b' ' => return Some(Cutout::Cutout),
                _ => continue,
            }
        }
    }
}

impl<P> Image<Cutout<FullColor>> for TextDecoder<P>
where
    P: Iterator<Item = u8>,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[test]
fn text_decoder_test() {
    use pretty_assertions::assert_eq;

    let invalid = "ぼくはまちちゃん！こんにちはこんにちは!!".as_bytes();
    let valid = "BWT  TWB".as_bytes();
    let mixed_valid = "!BWT hello TWB!".as_bytes();
    let expected = [
        Cutout::Opaque(FullColor::Black),
        Cutout::Opaque(FullColor::White),
        Cutout::Opaque(FullColor::Third),
        Cutout::Cutout,
        Cutout::Cutout,
        Cutout::Opaque(FullColor::Third),
        Cutout::Opaque(FullColor::White),
        Cutout::Opaque(FullColor::Black),
    ];

    let run = |size, src: &[u8]| -> Vec<Cutout<FullColor>> {
        TextDecoder::new(size, src.iter().copied()).collect()
    };

    // don't read
    assert_eq!(run(Size { h: 0, w: 0 }, &valid), []);
    assert_eq!(run(Size { h: 1, w: 0 }, &valid), []);
    assert_eq!(run(Size { h: 0, w: 1 }, &valid), []);

    // invalid input
    assert_eq!(run(Size { h: 1, w: 1 }, &invalid), []);

    // empty input
    assert_eq!(run(Size { h: 1, w: 1 }, &[]), []);

    // justfit
    assert_eq!(run(Size { h: 4, w: 2 }, &valid), &expected);

    // (3 x 2) + remaining: 2
    assert_eq!(run(Size { h: 3, w: 2 }, &valid), &expected[..6]);

    // need more inputs.
    assert_eq!(run(Size { h: 100, w: 100 }, &valid), &expected);

    // justfit w/ mixed_input
    assert_eq!(run(Size { h: 4, w: 2}, &mixed_valid), &expected);
}

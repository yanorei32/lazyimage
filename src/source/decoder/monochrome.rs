use crate::canvas::CanvasIterator;
use crate::color::{Cutout, MonoColor};
use crate::image::Image;
use crate::math::Size;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct MonochromeDecoder<P>
where
    P: Iterator<Item = bool>,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> MonochromeDecoder<P>
where
    P: Iterator<Item = bool>,
{
    pub fn new(size: Size, provider: P) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            provider,
        }
    }
}

impl<P> Iterator for MonochromeDecoder<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Cutout<MonoColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        self.provider.next().map(|v| {
            if v {
                Cutout::Opaque(MonoColor::Black)
            } else {
                Cutout::Cutout
            }
        })
    }
}

impl<Provider> Image<Cutout<MonoColor>> for MonochromeDecoder<Provider>
where
    Provider: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[test]
fn monochrome_decoder_test() {
    use pretty_assertions::assert_eq;

    let valid = [true, false, false, true];

    let expected = [
        Cutout::Opaque(MonoColor::Black),
        Cutout::Cutout,
        Cutout::Cutout,
        Cutout::Opaque(MonoColor::Black),
    ];

    let run = |size, src: &[bool]| -> Vec<Cutout<MonoColor>> {
        MonochromeDecoder::new(size, src.iter().copied()).collect()
    };

    // don't read
    assert_eq!(run(Size { h: 0, w: 0 }, &valid), []);
    assert_eq!(run(Size { h: 1, w: 0 }, &valid), []);
    assert_eq!(run(Size { h: 0, w: 1 }, &valid), []);

    // empty input
    assert_eq!(run(Size { h: 1, w: 1 }, &[]), []);

    // justfit
    assert_eq!(run(Size { h: 2, w: 2 }, &valid), &expected);

    // (3 x 1) + remaining: 1
    assert_eq!(run(Size { h: 3, w: 1 }, &valid), &expected[..3]);

    // need more inputs.
    assert_eq!(run(Size { h: 100, w: 100 }, &valid), &expected);
}

use crate::canvas::CanvasIterator;
use crate::color::{Cutout, FullColor};
use crate::image::Image;
use crate::math::Size;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct FullcolorDecoder<P>
where
    P: Iterator<Item = bool>,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P> FullcolorDecoder<P>
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

impl<P> Iterator for FullcolorDecoder<P>
where
    P: Iterator<Item = bool>,
{
    type Item = Cutout<FullColor>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;
        Some(match (self.provider.next()?, self.provider.next()?) {
            (false, false) => Cutout::Opaque(FullColor::White),
            (false, true) => Cutout::Opaque(FullColor::Black),
            (true, false) => Cutout::Opaque(FullColor::Third),
            (true, true) => Cutout::Cutout,
        })
    }
}

impl<P> Image<Cutout<FullColor>> for FullcolorDecoder<P>
where
    P: Iterator<Item = bool>,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[test]
fn fullcolor_decoder_test() {
    use pretty_assertions::assert_eq;

    let valid: Vec<bool> = [
        [false, true],
        [false, false],
        [true, false],
        [true, true],
        [true, true],
        [true, false],
        [false, false],
        [false, true],
    ]
    .map(|v| v.to_vec())
    .iter()
    .flatten()
    .copied()
    .collect();

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

    let run = |size, src: &[bool]| -> Vec<Cutout<FullColor>> {
        FullcolorDecoder::new(size, src.iter().copied()).collect()
    };

    // don't read
    assert_eq!(run(Size { h: 0, w: 0 }, &valid), []);
    assert_eq!(run(Size { h: 1, w: 0 }, &valid), []);
    assert_eq!(run(Size { h: 0, w: 1 }, &valid), []);

    // empty input
    assert_eq!(run(Size { h: 1, w: 1 }, &[]), []);

    // justfit
    assert_eq!(run(Size { h: 4, w: 2 }, &valid), &expected);

    // (3 x 2) + remaining: 2
    assert_eq!(run(Size { h: 3, w: 2 }, &valid), &expected[..6]);

    // need more inputs.
    assert_eq!(run(Size { h: 100, w: 100 }, &valid), &expected);
}

use crate::canvas::CanvasIterator;
use crate::image::Image;
use crate::math::Size;
use core::fmt::Debug;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct NbitDecoder<P, F, Color, const BIT_WIDTH: usize = 1>
where
    P: Iterator<Item = bool>,
    F: Fn(u8) -> Color,
    Color: Debug,
{
    ptr: CanvasIterator,
    size: Size,
    #[derivative(Debug = "ignore")]
    mapper: F,
    #[derivative(Debug = "ignore")]
    provider: P,
}

impl<P, F, Color, const BIT_WIDTH: usize> NbitDecoder<P, F, Color, BIT_WIDTH>
where
    P: Iterator<Item = bool>,
    F: Fn(u8) -> Color,
    Color: Debug,
{
    pub fn new(provider: P, size: Size, mapper: F) -> Self {
        Self {
            ptr: CanvasIterator::new(size),
            size,
            provider,
            mapper,
        }
    }
}

impl<P, F, Color, const BIT_WIDTH: usize> Iterator for NbitDecoder<P, F, Color, BIT_WIDTH>
where
    P: Iterator<Item = bool>,
    F: Fn(u8) -> Color,
    Color: Debug,
{
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.ptr.next()?;

        let mut v: u8 = 0;

        for n in 0..BIT_WIDTH {
            v |= if self.provider.next()? { 1 << n } else { 0 };
        }

        Some((self.mapper)(v))
    }
}

impl<P, F, Color, const BIT_WIDTH: usize> Image<Color> for NbitDecoder<P, F, Color, BIT_WIDTH>
where
    P: Iterator<Item = bool>,
    F: Fn(u8) -> Color,
    Color: Debug,
{
    fn size(&self) -> Size {
        self.size
    }
}

/// A bit iterator cap for the [`Iterator`] of [`u8`].
pub trait NbitDecode<T>
where
    T: Iterator<Item = bool>,
{
    fn nbit_decode<F, Color, const BIT_WIDTH: usize>(
        self,
        size: Size,
        mapper: F,
    ) -> NbitDecoder<T, F, Color, BIT_WIDTH>
    where
        F: Fn(u8) -> Color,
        Color: Debug;
}

impl<T> NbitDecode<T> for T
where
    T: Iterator<Item = bool>,
{
    fn nbit_decode<F, Color, const BIT_WIDTH: usize>(
        self,
        size: Size,
        mapper: F,
    ) -> NbitDecoder<T, F, Color, BIT_WIDTH>
    where
        F: Fn(u8) -> Color,
        Color: Debug,
    {
        NbitDecoder::new(self, size, mapper)
    }
}

#[test]
fn nbitdecoder_1bit_test() {
    use crate::iohelper::BitIterCap;
    use pretty_assertions::assert_eq;

    let input = [0b0101 as u8];

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum C {
        Blank,
        Black,
    }

    let expected = [C::Black, C::Blank, C::Black, C::Blank];

    let run = |size, src: &[u8]| -> Vec<C> {
        let d: NbitDecoder<_, _, _, 1> =
            NbitDecoder::new(src.to_vec().into_bit_iter(), size, |v| match v {
                0 => C::Blank,
                1 => C::Black,
                _ => unreachable!(),
            });

        d.collect()
    };

    // don't read
    assert_eq!(run(Size { h: 0, w: 0 }, &input), []);
    assert_eq!(run(Size { h: 1, w: 0 }, &input), []);
    assert_eq!(run(Size { h: 0, w: 1 }, &input), []);

    // empty input
    assert_eq!(run(Size { h: 1, w: 1 }, &[]), []);

    // justfit
    assert_eq!(run(Size { h: 2, w: 2 }, &input), expected);

    // non 1:1 aspect ratio
    assert_eq!(run(Size { h: 1, w: 4 }, &input), expected);

    // (1 x 3) + remaining: 1
    assert_eq!(run(Size { h: 1, w: 3 }, &input), &expected[..3]);
}

#[test]
fn nbitdecoder_2bit_test() {
    use crate::color::Cutout;
    use crate::iohelper::BitIterCap;
    use pretty_assertions::assert_eq;

    let input = [0b11_10_01_00 as u8];

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum C {
        Blank,
        Black,
        Red,
    }

    let expected = [
        Cutout::Cutout,
        Cutout::Opaque(C::Blank),
        Cutout::Opaque(C::Black),
        Cutout::Opaque(C::Red),
    ];

    let run = |size, src: &[u8]| -> Vec<Cutout<C>> {
        let d: NbitDecoder<_, _, _, 2> =
            NbitDecoder::new(src.to_vec().into_bit_iter(), size, |v| match v {
                0 => Cutout::Cutout,
                1 => Cutout::Opaque(C::Blank),
                2 => Cutout::Opaque(C::Black),
                3 => Cutout::Opaque(C::Red),
                _ => unreachable!(),
            });

        d.collect()
    };

    // don't read
    assert_eq!(run(Size { h: 0, w: 0 }, &input), []);
    assert_eq!(run(Size { h: 1, w: 0 }, &input), []);
    assert_eq!(run(Size { h: 0, w: 1 }, &input), []);

    // empty input
    assert_eq!(run(Size { h: 1, w: 1 }, &[]), []);

    // justfit
    assert_eq!(run(Size { h: 2, w: 2 }, &input), expected);

    // non 1:1 aspect ratio
    assert_eq!(run(Size { h: 1, w: 4 }, &input), expected);

    // (1 x 3) + remaining: 1
    assert_eq!(run(Size { h: 1, w: 3 }, &input), &expected[..3]);
}

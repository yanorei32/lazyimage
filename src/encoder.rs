use core::fmt::Debug;
use core::marker::PhantomData;
use derivative::Derivative;

#[allow(clippy::module_name_repetitions)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct NbitEncoder<I, F, Color, const BIT_WIDTH: usize = 1>
where
    I: Iterator<Item = Color>,
    F: Fn(Color) -> u8,
    Color: Debug,
{
    remaining: u8,
    buffer: u8,
    image: I,
    mapper: F,
    color_t: PhantomData<Color>,
}

impl<I, F, Color, const BIT_WIDTH: usize> NbitEncoder<I, F, Color, BIT_WIDTH>
where
    I: Iterator<Item = Color>,
    F: Fn(Color) -> u8,
    Color: Debug,
{
    pub fn new(image: I, mapper: F) -> Self {
        Self {
            remaining: 0,
            buffer: 0,
            image,
            mapper,
            color_t: PhantomData,
        }
    }
}

impl<I, F, Color, const BIT_WIDTH: usize> Iterator for NbitEncoder<I, F, Color, BIT_WIDTH>
where
    I: Iterator<Item = Color>,
    F: Fn(Color) -> u8,
    Color: Debug,
{
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            self.buffer = (self.mapper)(self.image.next()?);
            self.remaining = BIT_WIDTH as u8;
        }

        self.remaining -= 1;
        Some(self.buffer & (1 << (BIT_WIDTH as u8 - 1 - self.remaining)) != 0)
    }
}

#[test]
fn nbitencoder_1bit_test() {
    use pretty_assertions::assert_eq;
    use crate::iohelper::ByteIterCap;

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum C {
        Blank,
        Black,
    }

    let input = [C::Black, C::Blank, C::Black, C::Blank];
    let expected = [0b0101 as u8];

    let run = |src: &[C]| -> Vec<u8> {
        let e: NbitEncoder<_, _, _, 1> = NbitEncoder::new(src.into_iter(), |v| match v {
            C::Blank => 0,
            C::Black => 1,
        });

        e.into_byte_iter().collect()
    };

    // empty input
    assert_eq!(run(&[]), []);

    // justfit
    assert_eq!(run(&input), expected);
}

#[test]
fn nbitencoder_2bit_test() {
    use pretty_assertions::assert_eq;
    use crate::iohelper::ByteIterCap;

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum C {
        C0,
        C1,
        C2,
        C3,
    }

    let input = [C::C0, C::C1, C::C2, C::C3];
    let expected = [0b11_10_01_00 as u8];

    let run = |src: &[C]| -> Vec<u8> {
        let e: NbitEncoder<_, _, _, 2> = NbitEncoder::new(src.into_iter(), |v| match v {
            C::C0 => 0,
            C::C1 => 1,
            C::C2 => 2,
            C::C3 => 3,
        });

        e.into_byte_iter().collect()
    };

    // empty input
    assert_eq!(run(&[]), []);

    // justfit
    assert_eq!(run(&input), expected);
}

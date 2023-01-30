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

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum C {
        Blank,
        Black,
    }

    let input = [C::Black, C::Blank, C::Black, C::Blank];
    let expected = [0b0101 as u8];

    let run = |src: &[C]| -> Vec<u8> {
        let d: NbitEncoder<_, _, _, 1> = NbitEncoder::new(src.into_iter(), |v| match v {
            C::Blank => 0,
            C::Black => 1,
        });

        let d = ByteCap::new(d);

        d.collect()
    };

    // empty input
    assert_eq!(run(&[]), []);

    // justfit
    assert_eq!(run(&input), expected);
}

#[test]
fn nbitencoder_2bit_test() {
    use pretty_assertions::assert_eq;

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
        let d: NbitEncoder<_, _, _, 2> = NbitEncoder::new(src.into_iter(), |v| match v {
            C::C0 => 0,
            C::C1 => 1,
            C::C2 => 2,
            C::C3 => 3,
        });

        let d = ByteCap::new(d);

        d.collect()
    };

    // empty input
    assert_eq!(run(&[]), []);

    // justfit
    assert_eq!(run(&input), expected);
}

pub struct ByteCap<T>
where
    T: IntoIterator<Item = bool>,
{
    pub provider: T::IntoIter,
}

impl<T> ByteCap<T>
where
    T: IntoIterator<Item = bool>,
{
    pub fn new(provider: T) -> Self {
        Self {
            provider: provider.into_iter(),
        }
    }
}

impl<T> Iterator for ByteCap<T>
where
    T: IntoIterator<Item = bool>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let mut v: u8 = 0;

        for n in 0..8 {
            match self.provider.next() {
                Some(false) => continue,
                Some(true) => v |= 1 << n,
                None if n == 0 => return None,
                None => break,
            }
        }

        Some(v)
    }
}

#[test]
fn bytecap_test() {
    use pretty_assertions::assert_eq;

    let input = [
        true, false, true, false, true, false, true, false,
        false, true, false, true, false, true, false, true,
    ];

    let run = |src: &[bool]| -> Vec<u8> {
        ByteCap::new(src.iter().copied()).collect()
    };

    assert_eq!(run(&[]), &[]);
    assert_eq!(run(&input[..1]), &[0x01]);
    assert_eq!(run(&input[..4]), &[0x05]);
    assert_eq!(run(&input[..8]), &[0x55]);
    assert_eq!(run(&input[..12]), &[0x55, 0x0A]);
    assert_eq!(run(&input[..16]), &[0x55, 0xAA]);
}

extern crate image_provider;

use image_provider::{
    interface::*,
    math::*,
    source::{primitive::Rect, FullcolorDecoder, MonochromeDecoder, TextDecoder},
};
use pretty_assertions::assert_eq;

#[test]
fn primitive_rect_test() {
    let x = Rect::new(Size { h: 0, w: 0 }, ()).count();
    assert_eq!(0, x);

    let mut x = Rect::new(Size { h: 10, w: 5 }, ());

    for _ in 0..10 {
        for _ in 0..5 {
            assert_eq!(x.next(), Some(()));
        }
    }

    assert_eq!(x.next(), None);
}

#[test]
fn text_decoder_test() {
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

    let check_len = |size, len, src: &[u8]| {
        assert_eq!(
            TextDecoder::new(size, src.iter().map(|v| v.to_owned())).count(),
            len,
        );
    };

    let check_result = |size, slice: &[Cutout<FullColor>], src: &[u8]| {
        let result: Vec<Cutout<FullColor>> =
            TextDecoder::new(size, src.iter().map(|v| v.to_owned())).collect();

        assert_eq!(result.len(), slice.len());

        slice
            .iter()
            .zip(result.iter())
            .for_each(|(s, r)| assert!(s.to_owned() == r.to_owned()));
    };

    // don't read
    check_len(Size { h: 0, w: 0 }, 0, &valid);
    check_len(Size { h: 1, w: 0 }, 0, &valid);
    check_len(Size { h: 0, w: 1 }, 0, &valid);

    // invalid input
    check_len(Size { h: 1, w: 1 }, 0, &invalid);

    // empty input
    check_len(Size { h: 1, w: 1 }, 0, &valid[0..0]);

    // justfit
    check_result(Size { h: 4, w: 2 }, &expected, &valid);

    // (3 x 2) + remaining: 2
    check_result(Size { h: 3, w: 2 }, &expected[..6], &valid);

    // need more inputs.
    check_result(Size { h: 100, w: 100 }, &expected, &valid);

    // justfit!
    check_result(Size { h: 4, w: 2 }, &expected, &mixed_valid);
}

#[test]
fn fullcolor_decoder_test() {
    let base = [
        [false, true],
        [false, false],
        [true, false],
        [true, true],
        [true, true],
        [true, false],
        [false, false],
        [false, true],
    ];

    let valid: Vec<bool> = base
        .map(|v| v.to_vec())
        .iter()
        .flatten()
        .map(|v| v.to_owned())
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

    let check_len = |size, len, src: &[bool]| {
        assert_eq!(
            FullcolorDecoder::new(size, src.iter().map(|v| v.to_owned())).count(),
            len
        );
    };

    let check_result = |size, slice: &[Cutout<FullColor>], src: &[bool]| {
        let result: Vec<Cutout<FullColor>> =
            FullcolorDecoder::new(size, src.iter().map(|v| v.to_owned())).collect();

        assert_eq!(result.len(), slice.len());

        slice
            .iter()
            .zip(result.iter())
            .for_each(|(s, r)| assert!(s.to_owned() == r.to_owned()));
    };

    // don't read
    check_len(Size { h: 0, w: 0 }, 0, &valid);
    check_len(Size { h: 1, w: 0 }, 0, &valid);
    check_len(Size { h: 0, w: 1 }, 0, &valid);

    // empty input
    check_len(Size { h: 1, w: 1 }, 0, &valid[0..0]);

    // justfit
    check_result(Size { h: 4, w: 2 }, &expected, &valid);

    // (3 x 2) + remaining: 2
    check_result(Size { h: 3, w: 2 }, &expected[..6], &valid);

    // need more inputs.
    check_result(Size { h: 100, w: 100 }, &expected, &valid);
}

#[test]
fn monochrome_decoder_test() {
    let base = [true, false, true, false, false, true, false, true];

    let valid: Vec<bool> = base.iter().map(|v| v.to_owned()).collect();

    let expected = [
        Cutout::Opaque(MonoColor::Black),
        Cutout::Cutout,
        Cutout::Opaque(MonoColor::Black),
        Cutout::Cutout,
        Cutout::Cutout,
        Cutout::Opaque(MonoColor::Black),
        Cutout::Cutout,
        Cutout::Opaque(MonoColor::Black),
    ];

    let check_len = |size, len, src: &[bool]| {
        assert_eq!(
            MonochromeDecoder::new(size, src.iter().map(|v| v.to_owned())).count(),
            len
        );
    };

    let check_result = |size, slice: &[Cutout<MonoColor>], src: &[bool]| {
        let result: Vec<Cutout<MonoColor>> =
            MonochromeDecoder::new(size, src.iter().map(|v| v.to_owned())).collect();

        assert_eq!(result.len(), slice.len());

        slice
            .iter()
            .zip(result.iter())
            .for_each(|(s, r)| assert!(s.to_owned() == r.to_owned()));
    };

    // don't read
    check_len(Size { h: 0, w: 0 }, 0, &valid);
    check_len(Size { h: 1, w: 0 }, 0, &valid);
    check_len(Size { h: 0, w: 1 }, 0, &valid);

    // empty input
    check_len(Size { h: 1, w: 1 }, 0, &valid[0..0]);

    // justfit
    check_result(Size { h: 4, w: 2 }, &expected, &valid);

    // (3 x 2) + remaining: 2
    check_result(Size { h: 3, w: 2 }, &expected[..6], &valid);

    // need more inputs.
    check_result(Size { h: 100, w: 100 }, &expected, &valid);
}

extern crate image_provider;

use pretty_assertions::assert_eq;
use image_provider::{utility::*, math::*};

#[test]
fn area_test() {
    // w/o offset
    for scale in 0..2 {
        let area = Area::new(
            Point { h: 0, w: 0 },
            Size {
                h: scale,
                w: scale * 2,
            },
        );

        for h in 0..(scale * 3) {
            for w in 0..(scale * 3) {
                assert_eq!(area.contains(Point { h, w }), h < scale && w < (scale * 2),);
            }
        }
    }

    // w/ offset
    let pos = Point { h: 3, w: 5 };
    let size = Size { h: 5, w: 3 };
    let area = Area::new(pos, size);

    for h in 0..20 {
        for w in 0..20 {
            assert_eq!(
                area.contains(Point { h, w }),
                3 <= h && 5 <= w && h < 8 && w < 8,
            );
        }
    }
}

#[test]
fn canvas_iterator_test() {
    assert_eq!(0, CanvasIterator::new(Size { h: 0, w: 0 }).count());
    assert_eq!(0, CanvasIterator::new(Size { h: 1, w: 0 }).count());
    assert_eq!(0, CanvasIterator::new(Size { h: 0, w: 1 }).count());

    let mut x = CanvasIterator::new(Size { h: 10, w: 5 });

    for h in 0..10 {
        for w in 0..5 {
            assert_eq!(x.next(), Some(Point { h, w }));
        }
    }

    assert_eq!(x.next(), None);
}

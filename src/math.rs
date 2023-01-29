use core::{fmt::Debug, ops::Range};
use core::ops::{Add, Sub};

/// It provides [`Area::contains`] from [`Point`] and [`Size`].
#[derive(Debug, Clone)]
pub struct Area {
    w: Range<u16>,
    h: Range<u16>,
}

/// It contians `w` and `h` in [`u16`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub w: u16,
    pub h: u16,
}

/// It contians `w` and `h` in [`u16`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    /// The widhth of [`Size`]
    pub w: u16,
    /// The height of [`Size`]
    pub h: u16,
}

impl Area {
    #[must_use]
    pub fn new(pos: Point, size: Size) -> Self {
        Self {
            w: pos.w..pos.w + size.w,
            h: pos.h..pos.h + size.h,
        }
    }

    #[must_use]
    pub fn contains(&self, pos: Point) -> bool {
        self.w.contains(&pos.w) && self.h.contains(&pos.h)
    }
}

impl Point {
    #[must_use]
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w + rhs.w,
            h: self.h + rhs.h,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            w: self.w - rhs.w,
            h: self.h - rhs.h,
        }
    }
}

impl Size {
    #[must_use]
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }

    /// ```rust
    /// use lazyimage::math::Size;
    /// assert_eq!(Size::new(16, 9).is_zero(), false);
    /// assert_eq!(Size::new(16, 0).is_zero(), true);
    /// ```
    pub fn is_zero(self) -> bool {
        self.w == 0 || self.h == 0
    }
}

#[test]
fn size_test() {
    assert_eq!(Size::new(0, 0).is_zero(), true);
    assert_eq!(Size::new(1, 0).is_zero(), true);
    assert_eq!(Size::new(0, 1).is_zero(), true);
    assert_eq!(Size::new(1, 1).is_zero(), false);
}

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
fn point_test() {
    let zero = Point::new(0, 0);
    let one_zero = Point::new(1, 0);
    let zero_one = Point::new(0, 1);
    let one_one = Point::new(1, 1);
    assert_eq!((one_zero + zero_one), one_one);
    assert_eq!((one_zero + zero_one) - one_one, zero);
}

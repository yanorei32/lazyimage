use core::ops::{Add, Sub};
use core::{fmt::Debug, ops::Range};

/// It provides [`Area::contains`] from [`Point`] and [`Size`].
#[derive(Debug, Clone)]
pub struct Area {
    w: Range<u16>,
    h: Range<u16>,
}

/// It contians `w` and `h` in [`u16`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    /// offset from left [`Point`]
    pub w: u16,
    /// offset from top [`Point`]
    pub h: u16,
}

/// It contians `w` and `h` in [`u16`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    /// width of [`Size`]
    pub w: u16,
    /// height of [`Size`]
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
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.w == 0 || self.h == 0
    }
}

#[test]
fn size_test() {
    assert!(Size::new(0, 0).is_zero());
    assert!(Size::new(1, 0).is_zero());
    assert!(Size::new(0, 1).is_zero());
    assert!(!Size::new(1, 1).is_zero());
}

#[test]
fn area_test() {
    // w/o offset
    for scale in 0..2 {
        let area = Area::new(Point::new(0, 0), Size::new(scale * 2, scale));

        for h in 0..(scale * 3) {
            for w in 0..(scale * 3) {
                assert_eq!(
                    area.contains(Point::new(w, h)),
                    h < scale && w < (scale * 2)
                );
            }
        }
    }

    // w/ offset
    let pos = Point::new(3, 5);
    let size = Size::new(5, 3);
    let area = Area::new(pos, size);

    for h in 0..20 {
        for w in 0..20 {
            assert_eq!(
                area.contains(Point::new(w, h)),
                5 <= h && 3 <= w && h < 8 && w < 8,
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

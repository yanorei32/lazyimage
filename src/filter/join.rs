use crate::canvas::CanvasIterator;
use crate::image::Image;
use crate::math::Size;
use core::cmp::min;
use core::fmt::Debug;
use core::iter::Iterator;
use core::marker::PhantomData;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct JoinedImage<Base, BaseColor, Append, AppendColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Append: Image<AppendColor>,
    AppendColor: Into<BaseColor> + Debug,
{
    ptr: CanvasIterator,
    base: Base,
    base_size: Size,
    base_color: PhantomData<BaseColor>,
    append: Append,
    append_color: PhantomData<AppendColor>,
}

impl<Base, BaseColor, Append, AppendColor> Iterator
    for JoinedImage<Base, BaseColor, Append, AppendColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Append: Image<AppendColor>,
    AppendColor: Into<BaseColor> + Debug,
{
    type Item = BaseColor;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.ptr.next()?;

        if pos.w < self.base_size.w {
            self.base.next()
        } else {
            self.append.next().map(|v| v.into())
        }
    }
}

impl<Base, BaseColor, Append, AppendColor> Image<BaseColor>
    for JoinedImage<Base, BaseColor, Append, AppendColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Append: Image<AppendColor>,
    AppendColor: Into<BaseColor> + Debug,
{
    fn size(&self) -> Size {
        self.ptr.size()
    }
}

impl<Base, BaseColor, Append, AppendColor> JoinedImage<Base, BaseColor, Append, AppendColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Append: Image<AppendColor>,
    AppendColor: Into<BaseColor> + Debug,
{
    pub fn new(base: Base, append: Append) -> Self {
        let bs_s = base.size();
        let ap_s = append.size();
        let size = Size::new(bs_s.w + ap_s.w, min(bs_s.h, ap_s.h));

        Self {
            base,
            append,
            ptr: CanvasIterator::new(size),
            base_size: bs_s,
            append_color: PhantomData,
            base_color: PhantomData,
        }
    }
}

pub trait Join<I, P>
where
    Self: Sized,
    I: Image<P>,
    P: Debug,
{
    /// Provide joined image
    fn join<Append, AppendColor>(self, append: Append) -> JoinedImage<I, P, Append, AppendColor>
    where
        Append: Image<AppendColor>,
        AppendColor: Into<P> + Debug;
}

impl<I, P> Join<I, P> for I
where
    Self: Sized,
    I: Image<P>,
    P: Debug,
{
    fn join<Append, AppendColor>(self, append: Append) -> JoinedImage<I, P, Append, AppendColor>
    where
        Append: Image<AppendColor>,
        AppendColor: Into<P> + Debug,
    {
        JoinedImage::new(self, append)
    }
}

#[test]
fn joined_image_test() {
    use crate::source::Rect;
    use itertools::interleave;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    let run = |left, add| -> Vec<i64> { JoinedImage::new(left, add).collect() };

    let expected = |left: Rect<i64>, add: Rect<i64>| -> Vec<i64> {
        let lsz = left.size();
        let asz = add.size();
        let length = (lsz.w + asz.w) * min(lsz.h, asz.h);
        interleave(&left.chunks(lsz.w.into()), &add.chunks(asz.w.into()))
            .flatten()
            .take(length.into())
            .collect()
    };

    // zero size test
    assert_eq!(
        run(Rect::new(Size::new(0, 0), 1), Rect::new(Size::new(0, 0), 0)),
        [],
    );

    // zero size test
    assert_eq!(
        run(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(0, 0), 0)),
        [],
    );

    // zero size test
    assert_eq!(
        run(Rect::new(Size::new(0, 0), 1), Rect::new(Size::new(4, 3), 0)),
        [],
    );

    // zero size test
    assert_eq!(
        run(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 0), 0)),
        [],
    );

    // zero size test
    assert_eq!(
        run(Rect::new(Size::new(0, 3), 1), Rect::new(Size::new(0, 3), 0)),
        [],
    );

    // Basic test
    assert_eq!(
        run(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 3), 0)),
        expected(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 3), 0)),
    );

    // Basic test
    assert_eq!(
        run(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 3), 0)).len(),
        (4 + 4) * 3,
    );

    // Cut by left test
    assert_eq!(
        run(Rect::new(Size::new(4, 1), 1), Rect::new(Size::new(4, 3), 0)),
        expected(Rect::new(Size::new(4, 1), 1), Rect::new(Size::new(4, 3), 0)),
    );

    // Cut by left test
    assert_eq!(
        run(Rect::new(Size::new(4, 1), 1), Rect::new(Size::new(4, 3), 0)).len(),
        (4 + 4) * 1,
    );

    // Cut by right test
    assert_eq!(
        run(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 1), 0)),
        expected(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 1), 0)),
    );

    // Cut by right test
    assert_eq!(
        run(Rect::new(Size::new(4, 3), 1), Rect::new(Size::new(4, 1), 0)).len(),
        (4 + 4) * 1,
    );
}

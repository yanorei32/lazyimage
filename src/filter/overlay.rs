use crate::canvas::CanvasIterator;
use crate::color::Cutout;
use crate::error::Error;
use crate::image::Image;
use crate::math::{Area, Point, Size};
use core::fmt::Debug;
use core::iter::Iterator;
use core::marker::PhantomData;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct OverlayedImage<Base, BaseColor, Overlay, OverlayColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Overlay: Image<Cutout<OverlayColor>>,
    OverlayColor: Into<BaseColor> + Debug,
{
    area: Area,
    ptr: CanvasIterator,
    base: Base,
    base_color: PhantomData<BaseColor>,
    overlay: Overlay,
    overlay_color: PhantomData<OverlayColor>,
}

impl<Base, BaseColor, Overlay, OverlayColor> Iterator
    for OverlayedImage<Base, BaseColor, Overlay, OverlayColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Overlay: Image<Cutout<OverlayColor>>,
    OverlayColor: Into<BaseColor> + Debug,
{
    type Item = BaseColor;
    fn next(&mut self) -> Option<Self::Item> {
        let base = self.base.next()?;

        Some(if self.area.contains(self.ptr.next()?) {
            match self.overlay.next()? {
                Cutout::Cutout => base,
                Cutout::Opaque(v) => v.into(),
            }
        } else {
            base
        })
    }
}

impl<Base, BaseColor, Overlay, OverlayColor> Image<BaseColor>
    for OverlayedImage<Base, BaseColor, Overlay, OverlayColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Overlay: Image<Cutout<OverlayColor>>,
    OverlayColor: Into<BaseColor> + Debug,
{
    fn size(&self) -> Size {
        self.base.size()
    }
}

impl<Base, BaseColor, Overlay, OverlayColor> OverlayedImage<Base, BaseColor, Overlay, OverlayColor>
where
    Base: Image<BaseColor>,
    BaseColor: Debug,
    Overlay: Image<Cutout<OverlayColor>>,
    OverlayColor: Into<BaseColor> + Debug,
{
    pub fn new(base: Base, pos: Point, overlay: Overlay) -> Result<Self, Error> {
        if pos.w + overlay.size().w > base.size().w {
            return Err(Error::HorizontalOverflowIsDetected);
        }

        Ok(Self {
            area: Area::new(pos, overlay.size()),
            ptr: CanvasIterator::new(base.size()),
            base,
            overlay,
            base_color: PhantomData,
            overlay_color: PhantomData,
        })
    }
}

pub trait Overlay<I, P>
where
    Self: Sized,
    I: Image<P> + Iterator<Item = P>,
    P: Debug,
{
    fn overlay<Overlay, OverlayColor>(
        self,
        pos: Point,
        overlay: Overlay,
    ) -> Result<OverlayedImage<I, P, Overlay, OverlayColor>, Error>
    where
        Overlay: Image<Cutout<OverlayColor>>,
        OverlayColor: Into<P> + Debug;
}

impl<I, P> Overlay<I, P> for I
where
    Self: Sized,
    I: Image<P> + Iterator<Item = P>,
    P: Debug,
{
    fn overlay<Overlay, OverlayColor>(
        self,
        pos: Point,
        overlay: Overlay,
    ) -> Result<OverlayedImage<Self, P, Overlay, OverlayColor>, Error>
    where
        Overlay: Image<Cutout<OverlayColor>>,
        OverlayColor: Into<P> + Debug,
    {
        OverlayedImage::new(self, pos, overlay)
    }
}

#[test]
fn overlayed_image_test() {
    use crate::source::Rect;
    use crate::CanvasIterator;
    use core::iter::zip;
    use pretty_assertions::assert_eq;

    let run = |pos, add| -> Vec<i64> {
        OverlayedImage::new(Rect::new(Size::new(4, 3), 0), pos, add)
            .unwrap()
            .collect()
    };

    let expected = |pos, add: Rect<Cutout<i64>>| -> Vec<i64> {
        let mut pixels: Vec<i64> = Rect::new(Size::new(4, 3), 0).collect();

        let canvas = CanvasIterator::new(add.size()).map(|v| v + pos);

        for (pos, pixel) in zip(canvas, add) {
            match pixel {
                Cutout::Cutout => {}
                Cutout::Opaque(v) => pixels[usize::from(pos.h * 4 + pos.w)] = v,
            }
        }

        pixels
    };

    // Basic cutout test
    assert_eq!(
        run(Point::new(0, 0), Rect::new(Size::new(4, 3), Cutout::Cutout)),
        expected(Point::new(0, 0), Rect::new(Size::new(4, 3), Cutout::Cutout)),
    );

    // Basic opaque test
    assert_eq!(
        run(
            Point::new(0, 0),
            Rect::new(Size::new(4, 3), Cutout::Opaque(1))
        ),
        expected(
            Point::new(0, 0),
            Rect::new(Size::new(4, 3), Cutout::Opaque(1))
        ),
    );

    // opaque test
    assert_eq!(
        run(
            Point::new(1, 1),
            Rect::new(Size::new(1, 1), Cutout::Opaque(1))
        ),
        expected(
            Point::new(1, 1),
            Rect::new(Size::new(1, 1), Cutout::Opaque(1))
        ),
    );

    // zerosize test
    assert_eq!(
        run(
            Point::new(1, 1),
            Rect::new(Size::new(0, 0), Cutout::Opaque(1))
        ),
        expected(
            Point::new(1, 1),
            Rect::new(Size::new(0, 0), Cutout::Opaque(1))
        ),
    );
}

use crate::interface::{Cutout, Error, Image, Point, Size};
use crate::utility::{Area, CanvasIterator};
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
    pub(crate) fn new(base: Base, pos: Point, overlay: Overlay) -> Result<Self, Error> {
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

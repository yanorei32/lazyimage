use crate::interface::{Area, Cutout, Error, Image, Size};
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
    current_pos: Size,
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

        if !self.area.contains(self.current_pos) {
            return Some(base);
        }

        self.current_pos = match self.current_pos {
            // if end of line
            p if p.w == self.base.size().w - 1 => Size { w: 0, h: p.h + 1 },

            // otherwise
            p => Size { w: p.w + 1, h: p.h },
        };

        Some(
            match self.overlay.next()? {
                Cutout::Cutout => base,
                Cutout::Opaque(v) => v.into(),
            }
        )
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
    pub(crate) fn new(base: Base, pos: Size, overlay: Overlay) -> Result<Self, Error> {
        if pos.w + overlay.size().w > base.size().w {
            return Err(Error::HorizontalOverflowIsDetected);
        }

        Ok(Self {
            area: Area::from_pos_size(pos, overlay.size()),
            current_pos: Size { w: 0, h: 0 },
            base,
            overlay,
            base_color: PhantomData,
            overlay_color: PhantomData,
        })
    }
}

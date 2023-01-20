use crate::image::Image;
use crate::math::Size;
use core::fmt::Debug;
use core::iter::Iterator;
use core::marker::PhantomData;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RemappedImage<I, F, FromColor, ToColor>
where
    I: Image<FromColor>,
    F: Fn(FromColor) -> ToColor,
    FromColor: Debug,
    ToColor: Debug,
{
    image: I,
    #[derivative(Debug = "ignore")]
    f: F,
    from_color: PhantomData<FromColor>,
    to_color: PhantomData<ToColor>,
}

impl<I, F, FromColor, ToColor> Iterator for RemappedImage<I, F, FromColor, ToColor>
where
    I: Image<FromColor>,
    F: Fn(FromColor) -> ToColor,
    FromColor: Debug,
    ToColor: Debug,
{
    type Item = ToColor;
    fn next(&mut self) -> Option<ToColor> {
        self.image.next().map(&self.f)
    }
}

impl<I, F, FromColor, ToColor> Image<ToColor> for RemappedImage<I, F, FromColor, ToColor>
where
    I: Image<FromColor>,
    F: Fn(FromColor) -> ToColor,
    FromColor: Debug,
    ToColor: Debug,
{
    fn size(&self) -> Size {
        self.image.size()
    }
}

impl<I, F, FromColor, ToColor> RemappedImage<I, F, FromColor, ToColor>
where
    I: Image<FromColor>,
    F: Fn(FromColor) -> ToColor,
    FromColor: Debug,
    ToColor: Debug,
{
    pub fn new(image: I, f: F) -> Self {
        Self {
            image,
            f,
            from_color: PhantomData,
            to_color: PhantomData,
        }
    }
}

pub trait Remap<I, FromColor>
where
    Self: Sized,
    I: Image<FromColor>,
    FromColor: Debug,
{
    fn remap<F, ToColor>(self, f: F) -> RemappedImage<I, F, FromColor, ToColor>
    where
        ToColor: Debug,
        F: Fn(FromColor) -> ToColor;
}

impl<I, FromColor> Remap<I, FromColor> for I
where
    Self: Sized,
    I: Image<FromColor>,
    FromColor: Debug,
{
    fn remap<F, ToColor>(self, f: F) -> RemappedImage<I, F, FromColor, ToColor>
    where
        ToColor: Debug,
        F: Fn(FromColor) -> ToColor,
    {
        RemappedImage::new(self, f)
    }
}

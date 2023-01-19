use crate::interface::{Image, Size};
use core::fmt::Debug;
use core::iter::Iterator;
use core::marker::PhantomData;
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct RemappedImage<Im, Func, F, T>
where
    Im: Image<F>,
    Func: Fn(F) -> T,
    F: Debug,
    T: Debug,
{
    image: Im,
    #[derivative(Debug = "ignore")]
    f: Func,
    from_type: PhantomData<F>,
    to_type: PhantomData<T>,
}

impl<Im, Func, F, T> Iterator for RemappedImage<Im, Func, F, T>
where
    Im: Image<F>,
    Func: Fn(F) -> T,
    F: Debug,
    T: Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.image.next().map(&self.f)
    }
}

impl<Im, Func, F, T> Image<T> for RemappedImage<Im, Func, F, T>
where
    Im: Image<F>,
    Func: Fn(F) -> T,
    F: Debug,
    T: Debug,
{
    fn size(&self) -> Size {
        self.image.size()
    }
}

impl<Im, Func, F, T> RemappedImage<Im, Func, F, T>
where
    Im: Image<F>,
    Func: Fn(F) -> T,
    F: Debug,
    T: Debug,
{
    pub(crate) fn new(image: Im, f: Func) -> Self {
        Self { image, f, from_type: PhantomData, to_type: PhantomData }
    }
}

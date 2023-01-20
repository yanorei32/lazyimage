use crate::math::Size;
use core::fmt::Debug;

pub trait Image<P>: Iterator<Item = P> + Debug
where
    P: Debug,
{
    fn size(&self) -> Size;
}

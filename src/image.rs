use crate::math::Size;
use core::fmt::Debug;

/// A trait of generic image iterator
pub trait Image<P>: Iterator<Item = P> + Debug
where
    P: Debug,
{
    fn size(&self) -> Size;
}

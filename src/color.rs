use core::fmt::Debug;

/// The generic cutout behaviour
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cutout<T> {
    Cutout,
    Opaque(T),
}

impl<T> From<T> for Cutout<T> {
    fn from(c: T) -> Cutout<T> {
        Cutout::Opaque(c)
    }
}

impl<T> From<Cutout<Cutout<T>>> for Cutout<T> {
    fn from(value: Cutout<Cutout<T>>) -> Self {
        match value {
            Cutout::Cutout => Cutout::Cutout,
            Cutout::Opaque(Cutout::Cutout) => Cutout::Cutout,
            Cutout::Opaque(v) => v,
        }
    }
}

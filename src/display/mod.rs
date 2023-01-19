// use enum_map::EnumMap;
//
// #[cfg(feature = "imagebuffer_display")]
// pub mod imagebuffer;
//
// #[cfg(feature = "stdout_display")]
// pub mod stdout;
//
// pub type DisplayableMap<P, T> = EnumMap<P, T>;
//
// pub struct DisplayableMapBuilder<T> {
//     map: DisplayableMap<T>,
// }
//
// impl<T> DisplayableMapBuilder<T> {
//     #[must_use]
//     pub fn set_map(mut self, from: Color, to: T) -> Self {
//         self.map[from] = to;
//         self
//     }
//
//     #[must_use]
//     pub fn build(self) -> DisplayableMap<T> {
//         self.map
//     }
// }

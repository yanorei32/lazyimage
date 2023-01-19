// use crate::display::DisplayableMap;
// use crate::display::DisplayableMapBuilder;
// use crate::interface::Error;
// use crate::interface::{Color, Image};
// use crate::std::{
//     print, println,
//     string::{String, ToString},
// };
// use enum_map::enum_map;
//
// impl Default for DisplayableMapBuilder<String> {
//     fn default() -> Self {
//         Self {
//             map: enum_map! {
//                 Color::White => "##".to_string(),
//                 Color::Third => "**".to_string(),
//                 Color::Black => "..".to_string(),
//                 Color::Transpalent => "TT".to_string(),
//             },
//         }
//     }
// }
//
// pub trait Stdout<T> {
//     fn display_to_stdout<P: core::fmt::Display>(
//         &mut self,
//         map: DisplayableMap<P>,
//     ) -> Result<(), Error>;
// }
//
// impl<T> Stdout<T> for T
// where
//     T: Image,
// {
//     fn display_to_stdout<P: core::fmt::Display>(
//         &mut self,
//         map: DisplayableMap<P>,
//     ) -> Result<(), Error> {
//         let s = self.size();
//         println!("{}x{}", s.w, s.h);
//
//         for _ in 0..s.h {
//             self.take(s.w.into())
//                 .into_iter()
//                 .map(|v| &map[v])
//                 .for_each(|v| print!("{v}"));
//
//             println!();
//         }
//
//         Ok(())
//     }
// }

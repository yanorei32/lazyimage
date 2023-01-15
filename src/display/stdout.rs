use crate::display::DisplayableMap;
use crate::display::DisplayableMapBuilder;
use crate::interface::Error;
use crate::interface::{Color, ImageProvider};
use enum_map::enum_map;
use std::string::{String, ToString};

impl Default for DisplayableMapBuilder<String> {
    fn default() -> Self {
        Self {
            map: enum_map! {
                Color::White => "##".to_string(),
                Color::Third => "**".to_string(),
                Color::Black => "..".to_string(),
                Color::Transpalent => "TT".to_string(),
            },
        }
    }
}

pub trait Stdout<T> {
    fn display_to_stdout<P: core::fmt::Display>(&mut self, map: DisplayableMap<P>) -> Result<(), Error>;
}

impl<T> Stdout<T> for T
where
    T: ImageProvider,
{
    fn display_to_stdout<P: core::fmt::Display>(&mut self, map: DisplayableMap<P>) -> Result<(), Error> {
        let s = self.get_size();

        std::println!("{}x{}", s.w, s.h);

        for _ in 0..s.h {
            for _ in 0..s.w {
                let stdout = &map[self.next()?];
                std::print!("{stdout}");
            }

            std::println!();
        }

        Ok(())
    }
}

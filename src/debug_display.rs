use enum_map::enum_map;
use enum_map::EnumMap;

use enum_map::Enum;
use crate::image::{Color, ImageProvider};

type DisplayableMap = EnumMap<Color, DisplayableColor>;
pub struct DisplayableMapBuilder(DisplayableMap);

#[derive(Copy, Clone, PartialEq, Enum)]
pub enum DisplayableColor {
    Strong,
    Medium,
    Weak,
}

impl Default for DisplayableMapBuilder {
    fn default() -> Self {
        Self(enum_map! {
            Color::White => DisplayableColor::Strong,
            Color::Third => DisplayableColor::Medium,
            Color::Black => DisplayableColor::Weak,
            Color::Transpalent => DisplayableColor::Weak,
        })
    }
}

impl DisplayableMapBuilder {
    #[must_use]
    pub fn set_bg(mut self, bg: DisplayableColor) -> Self {
        self.0[Color::Transpalent] = bg;
        self
    }

    #[must_use]
    pub fn set_map(mut self, from: Color, to: DisplayableColor) -> Self {
        self.0[from] = to;
        self
    }

    #[must_use]
    pub fn build(self) -> DisplayableMap {
        self.0
    }
}

pub trait StdoutDisplay<T> {
    fn display_to_stdout(&mut self, map: DisplayableMap);
}

impl<T> StdoutDisplay<T> for T
where
    T: ImageProvider,
{
    fn display_to_stdout(&mut self, map: DisplayableMap) {
        let s = self.get_size();

        std::println!("{}x{}", s.w, s.h);

        for _ in 0..s.h {
            for _ in 0..s.w {
                let stdout = match map[self.next()] {
                    DisplayableColor::Strong => "##",
                    DisplayableColor::Medium => "==",
                    DisplayableColor::Weak => "..",
                };

                std::print!("{stdout}");
            }

            std::println!();
        }
    }
}

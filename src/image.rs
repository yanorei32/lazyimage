use enum_map::Enum;

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Enum)]
pub enum Color {
    Red,
    Black,
    White,
    Transpalent,
}

#[allow(clippy::module_name_repetitions)]
pub trait ImageProvider {
    fn get_size(&self) -> Size;
    fn next(&mut self) -> Color;
}

pub trait SimpleDisplay<T> {
    fn display(&mut self);
}

#[cfg(feature = "std")]
impl<T> SimpleDisplay<T> for T where T: ImageProvider {
    fn display(&mut self) {
        let s = self.get_size();

        std::println!("{}x{}", s.w, s.h);

        for _ in 0..s.h {
            for _ in 0..s.w {
                let stdout = match self.next() {
                    Color::White => "##",
                    Color::Red => "==",
                    Color::Black => "..",
                    Color::Transpalent => unreachable!(),
                };

                std::print!("{stdout}");
            }

            std::println!();
        }
    }
}

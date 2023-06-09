use lazyimage::{
    color::Cutout,
    filter::{Overlay, Remap},
    math::{Point, Size},
    sink::Png,
    source::{Rect, ScaledTextArea},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Rect::new(Size::new(640, 384), image::Rgb([192, 192, 192]))
        .overlay(
            Point::new(20, 20),
            ScaledTextArea::new(b"2023/06/09 (WED) BIN", 2).remap(|v| match v {
                Cutout::Opaque(_) => Cutout::Opaque(image::Rgb([0, 0, 0])),
                Cutout::Cutout => Cutout::Cutout,
            }),
        )
        .unwrap()
        .overlay(
            Point::new(20, 80),
            ScaledTextArea::new(b"Error: Failed to read SD Card.", 2).remap(|v| match v {
                Cutout::Opaque(_) => Cutout::Opaque(image::Rgb([0, 0, 0])),
                Cutout::Cutout => Cutout::Cutout,
            }),
        )
        .unwrap()
        .png_sink("File", 1)
        .unwrap();

    Ok(())
}

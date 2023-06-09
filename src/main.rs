use lazyimage::{
    color::Cutout,
    source::TextArea,
    filter::Remap,
    sink::Png,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    TextArea::new(b"C:\\>echo Hello World! 01234_")
        .remap(|v| match v {
            Cutout::Opaque(_) => image::Rgb([0, 0, 0]),
            Cutout::Cutout => image::Rgb([255, 255, 255]),
        })
        .png_sink("File", 8)
        .unwrap();

    Ok(())
}

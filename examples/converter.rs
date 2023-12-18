use lazyimage::{filter::Remap, iohelper::ByteIterCap, source::ImageCrate};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use image::{io::Reader as ImageReader, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let paths: Vec<PathBuf> = std::env::args()
        .skip(1)
        .map(|v| PathBuf::from_str(&v).unwrap())
        .collect();

    for path in paths {
        let image = ImageReader::open(&path)?.decode()?;

        let binary: Vec<u8> = ImageCrate::new(image)
            .remap(|v| match v {
                Rgba([255, 255, 255, 255]) => [false, false],
                Rgba([0, 0, 0, 255]) => [false, true],
                Rgba([255, 0, 0, 255]) => [true, false],
                Rgba([_, _, _, 0]) => [true, true],
                // _ => unimplemented!("{:?}", v),
                _ => [false, false],
            })
            .flatten()
            .into_byte_iter()
            .collect();

        let mut f = std::fs::File::create(path.with_extension("bin"))?;
        f.write_all(&binary)?;
    }

    Ok(())
}

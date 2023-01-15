use image::DynamicImage;
use std::{fs::File, io::Read};
use image_provider::{
    display::{imagebuffer::CreateImageBuffer, DisplayableMap, DisplayableMapBuilder},
    filter::{
        layered::LayeredImageBuilder,
        remap::{RemapBuilder, RemappedImage},
    },
    interface::{Color, Size, RawImageByteProvider},
    source::{
        rect::Rect,
        byte_reader::RawByteSource,
    },
};

#[derive(Debug)]
struct FileReader {
    file: File,
    size: Size,
    buffer: [u8; 32],
    ptr: usize,
}

impl RawImageByteProvider for FileReader {
    fn get_size(&self) -> Size {
        self.size
    }
    fn next(&mut self) -> u8 {
        self.ptr += 1;
        if self.ptr == 32 {
            println!("HERE");
            self.file.read(&mut self.buffer).unwrap();
            self.ptr = 0;
        }
        self.buffer[self.ptr]
    }
}

fn main() {
    let bg = Rect::new(Size { w: 16, h: 9 }, Color::Third);
    let bg2 = Rect::new(Size { w: 12, h: 9 }, Color::White);
    let bg3 = Rect::new(Size { w: 2, h: 2 }, Color::Black);

    let remap = RemapBuilder::default()
        .set_map(Color::Black, Color::Third)
        .build();

    let bg3 = RemappedImage::new(bg3, remap);

    let provider = FileReader {
        file: File::open("bin").unwrap(),
        size: Size { w: 5, h: 5 },
        ptr: 31,
        buffer: [0; 32],
    };

    let file = RawByteSource::new(provider, |p| {
        loop {
            match p.next() {
                b'B' => return Color::Black,
                b'W' => return Color::White,
                b'T' => return Color::Third,
                b' ' => return Color::Transpalent,
                _ => continue,
            }
        }
    });

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(bg2), Size { w: 2, h: 2 })
        .unwrap()
        .add_layer(Box::new(bg3), Size { w: 3, h: 3 })
        .unwrap()
        .add_layer(Box::new(file), Size { w: 0, h: 4 })
        .unwrap()
        .build();

    // let display_map: DisplayableMap<String> = DisplayableMapBuilder::default().build();
    // layered.display_to_stdout(display_map);

    let display_map: DisplayableMap<image::Rgba<u8>> = DisplayableMapBuilder::default().build();
    let buf = layered.create_imagebuffer(display_map);

    let dynamic = DynamicImage::from(buf);
    dynamic
        .resize(160, 90, image::imageops::FilterType::Nearest)
        .save("example.png")
        .unwrap();
}

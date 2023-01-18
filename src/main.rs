extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;
use image::DynamicImage;
use image_provider::{
    display::{imagebuffer::CreateImageBuffer, DisplayableMap, DisplayableMapBuilder},
    filter::{
        layered::LayeredImageBuilder,
        remap::{RemapBuilder, RemappedImage},
    },
    interface::{Color, Size},
    reader::{ByteIter, BitIter},
    source::{
        primitive::rect::Rect, reader::{monochrome::MonochromeReader, text::TextReader},
    },
};
use std::{fs::File, io::Read};

fn main() {
    let bg = Rect::new(Size { w: 16, h: 9 }, Color::Third);
    let bg2 = Rect::new(Size { w: 12, h: 9 }, Color::White);
    let bg3 = Rect::new(Size { w: 2, h: 2 }, Color::Black);

    let remap = RemapBuilder::default()
        .set_map(Color::Black, Color::Third)
        .build();

    let bg3 = RemappedImage::new(bg3, remap);

    let txt = Rc::new(RefCell::new(File::open("example.txt").unwrap()));
    let txt_clousure = Rc::clone(&txt);
    let txt_iter: ByteIter<_, 16> = ByteIter::new(move |buf| {
        let mut f = txt_clousure.try_borrow_mut().unwrap();
        Some(f.read(buf).ok()?)
    });
    let txt_src = TextReader::new(Size { w: 5, h: 5 }, txt_iter);

    let mono = Rc::new(RefCell::new(File::open("example.monochrome").unwrap()));
    let mono_clousure = Rc::clone(&mono);
    let mono_iter: BitIter<_, 16> = BitIter::new(move |buf| {
        let mut f = mono_clousure.try_borrow_mut().unwrap();
        Some(f.read(buf).ok()?)
    });
    let mono_src = MonochromeReader::new(Size { w: 2, h: 2 }, mono_iter);

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(bg2), Size { w: 2, h: 2 })
        .unwrap()
        .add_layer(Box::new(bg3), Size { w: 3, h: 3 })
        .unwrap()
        .add_layer(Box::new(txt_src), Size { w: 11, h: 4 })
        .unwrap()
        .add_layer(Box::new(mono_src), Size { w: 0, h: 0 })
        .unwrap()
        .build();

    // let display_map: DisplayableMap<String> = DisplayableMapBuilder::default().build();
    // layered.display_to_stdout(display_map).unwrap();

    let display_map: DisplayableMap<image::Rgba<u8>> = DisplayableMapBuilder::default().build();
    let buf = layered.create_imagebuffer(display_map).unwrap();
    //
    let dynamic = DynamicImage::from(buf);
    dynamic
        .resize(160, 90, image::imageops::FilterType::Nearest)
        .save("example.png")
        .unwrap();

    // let mut file = file.try_borrow_mut().unwrap();
    // let mut x: Vec<u8> = Vec::new();
    // file.read_to_end(&mut x).unwrap();
    // println!("{}", unsafe { std::str::from_utf8_unchecked(&x) } );
}

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
    reader::{BufToByte, BufToBit},
    source::{
        rect::Rect, simple_monochrome_reader::SimpleMonochromeReader,
        simple_text_reader::SimpleTextReader,
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

    let txtfile = File::open("example.txt").unwrap();
    let txtfile = Rc::new(RefCell::new(txtfile));
    let txtfile_clousure = Rc::clone(&txtfile);
    let txtfile_byteprov: BufToByte<_, 16> = BufToByte::new(move |buf| {
        let mut f = txtfile_clousure.try_borrow_mut().unwrap();
        Some(f.read(buf).ok()?)
    });
    let txtfile_src = SimpleTextReader::new(Size { w: 5, h: 5 }, txtfile_byteprov);

    let monofile = File::open("example.monochrome").unwrap();
    let monofile = Rc::new(RefCell::new(monofile));
    let monofile_clousure = Rc::clone(&monofile);
    let monofile_bitprov: BufToBit<_, 16> = BufToBit::new(move |buf| {
        let mut f = monofile_clousure.try_borrow_mut().unwrap();
        Some(f.read(buf).ok()?)
    });
    let monofile_src = SimpleMonochromeReader::new(Size { w: 2, h: 2 }, monofile_bitprov);

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(bg2), Size { w: 2, h: 2 })
        .unwrap()
        .add_layer(Box::new(bg3), Size { w: 3, h: 3 })
        .unwrap()
        .add_layer(Box::new(txtfile_src), Size { w: 11, h: 4 })
        .unwrap()
        .add_layer(Box::new(monofile_src), Size { w: 0, h: 0 })
        .unwrap()
        .build();

    // let display_map: DisplayableMap<String> = DisplayableMapBuilder::default().build();
    // layered.display_to_stdout(display_map).unwrap();

    let display_map: DisplayableMap<image::Rgba<u8>> = DisplayableMapBuilder::default().build();
    let buf = layered.create_imagebuffer(display_map).unwrap();

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

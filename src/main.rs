extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;
use image::{imageops::ColorMap, DynamicImage};
use image_provider::{
    filter::{layered::LayeredImageBuilder},
    interface::{FullColor, Image, Size},
    reader::{BitIter, ByteIter},
    source::{
        primitive::rect::Rect,
        reader::{fullcolor::FullcolorReader, monochrome::MonochromeReader, text::TextReader},
    },
};
use std::{fs::File, io::Read};

fn main() {
    let bg = Rect::new(Size { w: 16, h: 9 }, FullColor::Third);
    let bg2 = Rect::new(Size { w: 12, h: 9 }, FullColor::White);
    let bg3 = Rect::new(Size { w: 2, h: 2 }, FullColor::Black);

    let txt = Rc::new(RefCell::new(File::open("example.txt").unwrap()));
    let txt_clousure = Rc::clone(&txt);
    let txt_iter: ByteIter<_, 16> =
        ByteIter::new(move |buf| txt_clousure.try_borrow_mut().unwrap().read(buf).ok());
    let txt_src = TextReader::new(Size { w: 5, h: 5 }, txt_iter);

    let mono = Rc::new(RefCell::new(File::open("example.monochrome").unwrap()));
    let mono_clousure = Rc::clone(&mono);
    let mono_iter: BitIter<_, 16> =
        BitIter::new(move |buf| mono_clousure.try_borrow_mut().unwrap().read(buf).ok());
    let mono_src = MonochromeReader::new(Size { w: 2, h: 2 }, mono_iter);

    let color = Rc::new(RefCell::new(File::open("example.fullcolor").unwrap()));
    let color_clousure = Rc::clone(&color);
    let color_iter: BitIter<_, 16> =
        BitIter::new(move |buf| color_clousure.try_borrow_mut().unwrap().read(buf).ok());
    let color_src = FullcolorReader::new(Size { w: 2, h: 2 }, color_iter);

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg.remap(|v| v.into())), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(bg2.remap(|v| v.into())), Size { w: 2, h: 2 })
        .unwrap()
        .add_layer(Box::new(bg3.remap(|v| v.into())), Size { w: 3, h: 3 })
        .unwrap()
        .add_layer(Box::new(txt_src), Size { w: 11, h: 4 })
        .unwrap()
        .add_layer(Box::new(mono_src.remap(|v| v.convert_inner())), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(color_src), Size { w: 3, h: 7 })
        .unwrap()
        .build();

    // let color: Cutout<FullColor> = layered.next().unwrap();

    // let display_map: DisplayableMap<String> = DisplayableMapBuilder::default().build();
    // layered.display_to_stdout(display_map).unwrap();

    // let display_map: DisplayableMap<image::Rgba<u8>> = DisplayableMapBuilder::default().build();
    // let buf = layered.create_imagebuffer(display_map).unwrap();
    // //
    // let dynamic = DynamicImage::from(buf);
    // dynamic
    //     .resize(160, 90, image::imageops::FilterType::Nearest)
    //     .save("example.png")
    //     .unwrap();

    // let mut file = file.try_borrow_mut().unwrap();
    // let mut x: Vec<u8> = Vec::new();
    // file.read_to_end(&mut x).unwrap();
    // println!("{}", unsafe { std::str::from_utf8_unchecked(&x) } );
}

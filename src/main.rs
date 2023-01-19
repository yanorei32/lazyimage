extern crate alloc;
use alloc::rc::Rc;
use core::cell::RefCell;
use image::{DynamicImage, ImageBuffer, Pixel};
use image_provider::{
    interface::{FullColor, Image, Point, Size},
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

    let image = bg
        .overlay(Point { w: 2, h: 2 }, bg2.remap(|v| v.into()))
        .unwrap()
        .overlay(Point { w: 3, h: 3 }, bg3.remap(|v| v.into()))
        .unwrap()
        .overlay(Point { w: 11, h: 4 }, txt_src)
        .unwrap()
        .overlay(
            Point { w: 0, h: 0 },
            mono_src.remap(|v| v.convert_inner::<FullColor>()),
        )
        .unwrap()
        .overlay(Point { w: 3, h: 7 }, color_src)
        .unwrap();

    let size = image.size();

    let pixels: Vec<u8> = image
        .remap(|v| match v {
            FullColor::White => image::Rgb([192, 192, 192]),
            FullColor::Black => image::Rgb([32, 32, 32]),
            FullColor::Third => image::Rgb([192, 32, 32]),
        })
        .map(|v| v.channels().to_owned())
        .flatten()
        .collect();

    let buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        ImageBuffer::from_vec(size.w.into(), size.h.into(), pixels).unwrap();

    let dynamic = DynamicImage::from(buffer);
    dynamic
        .resize(160, 90, image::imageops::FilterType::Nearest)
        .save("example.png")
        .unwrap();
}

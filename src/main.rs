use core::cell::RefCell;
use image_provider::{
    color::FullColor,
    filter::{Overlay, Remap},
    math::*,
    reader::{BitCap, ByteIter},
    sink::Png,
    source::{primitive::Rect, FullcolorDecoder, MonochromeDecoder, TextDecoder},
};
use std::{fs::File, io::Read, rc::Rc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bg = Rect::new(Size::new(16, 9), FullColor::Third);
    let bg2 = Rect::new(Size::new(12, 9), FullColor::White);
    let bg3 = Rect::new(Size { w: 2, h: 2 }, FullColor::Black);

    let txt = Rc::new(RefCell::new(File::open("example.txt").unwrap()));
    let txt_clousure = Rc::clone(&txt);
    let txt_iter: ByteIter<_, 16> =
        ByteIter::new(move |buf| txt_clousure.borrow_mut().read(buf).ok());
    let txt_src = TextDecoder::new(Size { w: 5, h: 5 }, txt_iter);

    let mono = Rc::new(RefCell::new(File::open("example.monochrome").unwrap()));
    let mono_clousure = Rc::clone(&mono);
    let mono_iter: ByteIter<_, 16> =
        ByteIter::new(move |buf| mono_clousure.borrow_mut().read(buf).ok());

    let mono_src = MonochromeDecoder::new(Size { w: 2, h: 2 }, mono_iter.bits());

    let color = Rc::new(RefCell::new(File::open("example.fullcolor").unwrap()));
    let color_clousure = Rc::clone(&color);
    let color_iter: ByteIter<_, 16> =
        ByteIter::new(move |buf| color_clousure.borrow_mut().read(buf).ok());
    let color_src = FullcolorDecoder::new(Size { w: 2, h: 2 }, color_iter.bits());

    let image = bg
        .overlay(Point { w: 2, h: 2 }, bg2.remap(|v| v.into()))
        .unwrap()
        .overlay(Point { w: 3, h: 3 }, bg3.remap(|v| v.into()))
        .unwrap()
        .overlay(Point { w: 11, h: 4 }, txt_src)
        .unwrap()
        .overlay(Point { w: 0, h: 0 }, mono_src)
        .unwrap()
        .overlay(Point { w: 3, h: 7 }, color_src)
        .unwrap();

    image.png_sink("example.png", 10)?;

    Ok(())
}

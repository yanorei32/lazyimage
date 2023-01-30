use lazyimage::{
    decoder::{NbitDecoder, TextDecoder},
    encoder::{ByteCap, NbitEncoder},
    math::Size,
    reader::{BitCap, ByteIter},
    source::Rect,
};
use std::io::prelude::*;
use std::{cell::RefCell, error::Error, fs::File, io::Read, rc::Rc};

fn main() -> Result<(), Box<dyn Error>> {
    #[derive(Debug, Copy, Clone)]
    #[repr(u8)]
    enum Color {
        Black = 0,
        Gray1 = 1,
        Gray2 = 2,
        White = 3,
        Red0 = 4,
        Red1 = 5,
        Red2 = 6,
        Red3 = 7,
    }

    let bg = Rect::new(Size::new(640, 384), Color::Gray1);
    let encoder: NbitEncoder<_, _, _, 3> = NbitEncoder::new(bg, |c| c as u8);
    let bytecap: Vec<u8> = ByteCap::new(encoder).collect();
    let mut file = File::create("test.bin")?;
    file.write_all(&bytecap)?;

    let bin = Rc::new(RefCell::new(File::open("test.bin")?));
    let bin_ = Rc::clone(&bin);
    let iter: ByteIter<_, 16> = ByteIter::new(move |buf| bin_.borrow_mut().read(buf).ok());

    let decoder: NbitDecoder<_, _, _, 3> =
        NbitDecoder::new(iter.bits(), Size::new(640, 384), |v| unsafe {
            let c: Color = core::mem::transmute(v);
            c
        });

    let x: Vec<Color> = decoder.collect();

    println!("{:?}", x);

    Ok(())
}

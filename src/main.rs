use image::DynamicImage;
use image_provider::{
    display::{imagebuffer::CreateImageBuffer, DisplayableMap, DisplayableMapBuilder},
    filter::{
        layered::LayeredImageBuilder,
        remap::{RemapBuilder, RemappedImage},
    },
    interface::{Color, Size},
    source::rect::Rect,
};

fn main() {
    let bg = Rect::new(Size { w: 16, h: 9 }, Color::Third);
    let bg2 = Rect::new(Size { w: 12, h: 9 }, Color::White);
    let bg3 = Rect::new(Size { w: 2, h: 2 }, Color::Black);

    let remap = RemapBuilder::default()
        .set_map(Color::Black, Color::Third)
        .build();

    let bg3 = RemappedImage::new(bg3, remap);

    // println!("{:?}", bg3);

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(bg2), Size { w: 2, h: 2 })
        .unwrap()
        .add_layer(Box::new(bg3), Size { w: 3, h: 3 })
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

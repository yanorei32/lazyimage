use image::{DynamicImage, Rgb};
use image_provider::color_remapper::{RemapBuilder, RemappedImage};
// use image_provider::debug_display::{DisplayableMapBuilder, StdoutDisplay};
use image_provider::filled::FilledImage;
use image_provider::image::{Color, Size};
use image_provider::image_display::{DisplayableMapBuilder, ImageBufferDisplay};
use image_provider::layered::LayeredImageBuilder;

// use image::{ImageBuffer};

fn main() {
    //     let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
    //     if (x + y) % 2 == 0 {
    //         image::Rgb([0, 0, 0])
    //     } else {
    //         image::Rgb([255, 255, 255])
    //     }
    // }).xxxsa;

    let bg = FilledImage::new(Size { w: 16, h: 9 }, Color::Third);
    let bg2 = FilledImage::new(Size { w: 12, h: 9 }, Color::White);
    let bg3 = FilledImage::new(Size { w: 2, h: 2 }, Color::Black);

    // let remap = RemapBuilder::default()
    //     .set_map(Color::Black, Color::Third)
    //     .build();

    // let bg3 = RemappedImage::new(bg3, remap);

    // println!("{:?}", bg3);

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg), Size { w: 0, h: 0 })
        .unwrap()
        .add_layer(Box::new(bg2), Size { w: 2, h: 2 })
        .unwrap()
        .add_layer(Box::new(bg3), Size { w: 3, h: 3 })
        .unwrap()
        .build();

    let display_map = DisplayableMapBuilder::new(
        Rgb([255, 0, 255] as [u8; 3]),
        Rgb([192, 192, 192]),
        Rgb([32, 32, 32]),
        Rgb([192, 32, 32]),
    )
    .build();

    let buf = layered.create_imagebuffer(display_map);
    let dynamic = DynamicImage::from(buf);
    dynamic
        .resize(160, 90, image::imageops::FilterType::Nearest)
        .save("example.png")
        .unwrap();

    // println!("{:?}", layered);
    // layered.display_to_stdout(display_map);
}

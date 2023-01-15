use image_provider::filled::FilledImage;
use image_provider::color_remapper::{RemapBuilder, RemappedImage};
use image_provider::image::{Color, SimpleDisplay, Size};
use image_provider::layered::LayeredImageBuilder;

fn main() {
    let bg = FilledImage::new(Size { w: 16-1, h: 9 }, Color::Red);
    let bg2 = FilledImage::new(Size { w: 16-3, h: 9 }, Color::White);
    let bg3 = FilledImage::new(Size { w: 2, h: 2 }, Color::Black);

    let remap = RemapBuilder::default().set_map(Color::Black, Color::Red).build();
    let bg3 = RemappedImage::new(bg3, remap);

    println!("{:?}", bg3);

    let mut layered = LayeredImageBuilder::new(Size { w: 16, h: 9 })
        .add_layer(Box::new(bg), Size { w: 1, h: 1 }).unwrap()
        .add_layer(Box::new(bg2), Size { w: 3, h: 3 }).unwrap()
        .add_layer(Box::new(bg3), Size { w: 5, h: 5 }).unwrap()
        .set_bg(Color::Black)
        .build();


    layered.display();

}

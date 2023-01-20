pub mod primitive;
mod decoder;

pub use decoder::fullcolor::FullcolorDecoder;
pub use decoder::monochrome::MonochromeDecoder;
pub use decoder::text::TextDecoder;

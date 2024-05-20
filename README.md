# Lazy Image
If you are using a platform with low memory, you can think of an image as an iterator of pixels.
This is because the image buffer can be larger than system memory.

Core idea is here:
```rust
pub trait Image<P>: Iterator<Item = P> + Debug
where
    P: Debug,
{
    fn size(&self) -> Size;
}
```

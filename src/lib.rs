#![no_std]
#![warn(clippy::pedantic)]
#![cfg_attr(not(feature = "alloc"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
pub mod debug_display;

#[cfg(feature = "image_display")]
pub mod image_display;

pub mod filled;
pub mod image;
pub mod layered;
pub mod color_remapper;

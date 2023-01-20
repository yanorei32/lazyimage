#![cfg_attr(not(test), no_std)]
#![warn(clippy::pedantic)]

pub mod image;
pub mod color;
pub mod source;
pub mod reader;
pub mod canvas;
pub mod filter;
pub mod math;
pub mod error;

#[cfg(feature = "sink")]
pub mod sink;

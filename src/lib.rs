#![no_std]
#![warn(clippy::pedantic)]

pub mod interface;
pub mod source;
pub mod reader;
pub mod utility;
pub mod filter;

#[cfg(feature = "sink")]
pub mod sink;

#[cfg(feature = "std")]
pub extern crate std;

#[cfg(feature = "image")]
pub extern crate image;

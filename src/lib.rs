#![no_std]
#![warn(clippy::pedantic)]

pub mod interface;
pub mod source;
pub mod reader;
pub(crate) mod filter;
pub(crate) mod utility;

#[cfg(feature = "std")]
pub extern crate std;

#[cfg(feature = "image")]
pub extern crate image;

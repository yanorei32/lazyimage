#![no_std]
#![warn(clippy::pedantic)]

pub mod display;
pub mod filter;
pub mod interface;
pub mod source;
pub mod buffer_prober;

pub extern crate alloc;

#[cfg(feature = "std")]
pub extern crate std;

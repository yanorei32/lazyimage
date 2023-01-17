#![no_std]
#![warn(clippy::pedantic)]

//#![cfg_attr(not(feature = "alloc"), no_std)]

#[cfg(feature = "std")]
pub(crate) extern crate std;

#[cfg(feature = "alloc")]
pub(crate) extern crate alloc;

pub mod display;
pub mod filter;
pub mod interface;
pub mod source;

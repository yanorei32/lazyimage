#![no_std]
#![warn(clippy::pedantic)]
#![cfg_attr(not(feature = "alloc"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod display;
pub mod filter;
pub mod interface;
pub mod source;

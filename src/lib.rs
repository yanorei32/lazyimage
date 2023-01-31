#![cfg_attr(all(not(test), not(doctest)), no_std)]
#![warn(clippy::pedantic)]

//! # Overview
//! 
//! This crate is a iterator-based lazy evaluation image library
//! for low-memory embedded targets (likes STM32).
//!
//! # Todo
//! - Implement `filter::JoinRight`.

mod image;
pub use crate::image::Image;

mod canvas;
pub use canvas::CanvasIterator;

mod error;
pub use error::Error;

/// The minimum I/O helper for this crate.
pub mod iohelper;

/// The filter is built from [`Image`] (s) and provides [`Image`].
pub mod filter;

/// The minimum color impementation for this crate.
pub mod color;

/// The source is primitives and provides [`Image`]
pub mod source;

/// The minimum math implementation for this crate.
pub mod math;

/// The deocder is built from [`Iterator`] and provides [`Image`].
pub mod decoder;

/// The encoder is built from [`Iterator<Color>`] and provides [`Iterator<bool>`].
///
/// If you need [`Iterator<u8>`] or more bytes, you can use the [`filter::Remap::remap`] or
/// [`Iterator::map`].
pub mod encoder;

/// The sink for debugging.
#[cfg(feature = "sink")]
pub mod sink;

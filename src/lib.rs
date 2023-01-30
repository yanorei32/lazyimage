#![cfg_attr(all(not(test), not(doctest)), no_std)]
#![warn(clippy::pedantic)]

//! # Overview
//! 
//! This crate is a iterator-based lazy evaluation image library
//! for low-memory embedded targets (likes STM32).
//!
//! # Todo
//! - Implement `filter::JoinRight`.
//! - move BitCap / ByteCap to itertool

mod image;
pub use crate::image::Image;

mod canvas;
pub use canvas::CanvasIterator;

mod error;
pub use error::Error;

/// The reader is built from callback function and provides [`Iterator`]`<Item = `[`bool`] or [`u8`]`>`.
pub mod reader;

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

/// The encoder is built from [`Image`] and provides [`Iterator<u8>`].
pub mod encoder;

/// The sink for debugging.
#[cfg(feature = "sink")]
pub mod sink;

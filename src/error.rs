use core::fmt::Debug;

/// Errors
#[derive(Debug, Copy, Clone)]
pub enum Error {
    HorizontalOverflowIsDetected,
}

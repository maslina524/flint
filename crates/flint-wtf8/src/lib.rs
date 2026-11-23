#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod slice;
pub use slice::Wtf8;

mod codepoint;
pub use codepoint::*;

mod utf16;
pub use utf16::*;

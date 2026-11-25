#![cfg_attr(not(test), no_std)]

mod api;
pub use api::*;

mod init_array;
mod raw;

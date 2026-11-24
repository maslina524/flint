mod syscalls;
pub use syscalls::*;

use core::ffi::c_int;

pub type RawFd = c_int;

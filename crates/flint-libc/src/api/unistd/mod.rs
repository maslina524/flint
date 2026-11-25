mod syscalls;
pub use syscalls::*;

use core::{
    ffi::{c_char, c_int},
    ptr,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering::Relaxed},
};

pub type RawFd = c_int;

pub(crate) static ARGC: AtomicUsize = AtomicUsize::new(0);
pub(crate) static ARGV: AtomicPtr<*const c_char> = AtomicPtr::new(ptr::null_mut());
pub(crate) static ENVP: AtomicPtr<*const c_char> = AtomicPtr::new(ptr::null_mut());

pub fn argc() -> usize {
    ARGC.load(Relaxed)
}

pub fn argv() -> *const *const c_char {
    ARGV.load(Relaxed).cast_const()
}

pub fn envp() -> *const *const c_char {
    ENVP.load(Relaxed).cast_const()
}

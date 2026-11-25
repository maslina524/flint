mod file;
use core::mem::ManuallyDrop;

pub use file::*;

mod perm;
pub use perm::*;

// SAFETY: Reserved for Stdin.
pub static STDIN: ManuallyDrop<File> = unsafe { File::from_fd_no_close_unchecked(0) };
// SAFETY: Reserved for Stdout.
pub static STDOUT: ManuallyDrop<File> = unsafe { File::from_fd_no_close_unchecked(1) };
// SAFETY: Reserved for Stderr.
pub static STDERR: ManuallyDrop<File> = unsafe { File::from_fd_no_close_unchecked(2) };

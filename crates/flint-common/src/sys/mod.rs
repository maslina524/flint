#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_family = "unix")]
pub mod unix;
#[cfg(target_family = "unix")]
pub use unix::*;

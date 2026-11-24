use core::ffi::{CStr, c_int, c_uint};

use crate::{errno::Errno, raw::Table, syscall, unistd::RawFd};

/// # Errors
/// On success: raw file fd;
/// on failure: [`Errno`] returned by the kernel.
///
/// # References
///  - [Linux]
///
/// [Linux]: <https://elixir.bootlin.com/linux/v6.17/source/fs/open.c#L1461>
pub fn openat(dirfd: RawFd, path: &CStr, flags: c_int, mode: c_uint) -> Result<RawFd, Errno> {
    // SAFETY: The number is valid, passing a valid null-terminated `CStr`;
    // if `difd`, `flags`, or `mode` are invalid, the kernel will return `Errno`.
    let ret = unsafe { syscall!(Table::OpenAt, dirfd, path.as_ptr(), flags, mode) };
    ret.map(|v| v as RawFd)
}

/// # Errors
/// On success: raw file fd;
/// on failure: [`Errno`] returned by the kernel.
///
/// # References
///  - [Linux]
///
/// [Linux]: <https://elixir.bootlin.com/linux/v6.17/source/fs/open.c#L1454>
pub fn open(path: &CStr, flags: c_int, mode: c_uint) -> Result<RawFd, Errno> {
    const AT_FDCWD: RawFd = -100;
    openat(AT_FDCWD, path, flags, mode)
}

/// # Errors
/// On success: bitmask;
/// on failure: [`Errno`] returned by the kernel.
///
/// # References
///  - [Linux]
///
/// [Linux]: <https://elixir.bootlin.com/linux/v6.17/source/fs/fcntl.c#L576>
pub fn fcntl(fd: RawFd, cmd: c_uint) -> Result<i32, Errno> {
    // SAFETY: The number is valid;
    // if `cmd` are invalid, the kernel will return `Errno`.
    let ret = unsafe { syscall!(Table::Fcntl, fd, cmd, 0) };
    ret.map(|v| v as i32)
}

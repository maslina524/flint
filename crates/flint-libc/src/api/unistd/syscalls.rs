use core::ffi::c_int;

use crate::{errno::Errno, raw::Table, syscall};

/// # Errors
/// On success, it returns the number of bytes written;
/// on failure, it records the Errno returned by the kernel.
///
/// # References
///  - [Linux]
///
/// [Linux]: <https://elixir.bootlin.com/linux/v6.17/source/fs/read_write.c#L746>
pub fn write(fd: c_int, buf: &[u8]) -> Result<usize, Errno> {
    if buf.is_empty() {
        return Ok(0);
    }

    // SAFETY: The number is valid, and the pointer is valid and resides entirely within userspace;
    // the kernel may truncate the slice if it exceeds 2 GB.
    let ret = unsafe { syscall!(Table::Write, fd, buf.as_ptr(), buf.len()) };
    // In Linux, if the returned value is >= 0, the syscall has completed successfully.
    // Ref: <https://elixir.bootlin.com/linux/v6.17/source/fs/read_write.c#L739>
    ret.map(isize::cast_unsigned)
}

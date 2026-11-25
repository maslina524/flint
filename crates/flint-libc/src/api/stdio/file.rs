use core::{
    error::Error,
    fmt,
    mem::{self, ManuallyDrop},
};

use crate::{
    errno::Errno,
    fcntl,
    unistd::{self, RawFd},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidFd(Errno);

impl fmt::Display for InvalidFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid fd: {}", self.0)
    }
}

impl Error for InvalidFd {}

#[derive(PartialEq, Eq)]
pub struct File(RawFd);

impl File {
    /// # Errors
    /// If fd is invalid.
    pub fn from_fd(raw: RawFd) -> Result<Self, InvalidFd> {
        fcntl::fcntl(raw, 1).map(Self).map_err(InvalidFd)
    }

    /// # Safety
    /// The fd must be valid for the kernel.
    #[must_use]
    pub const unsafe fn from_fd_unchecked(raw: RawFd) -> Self {
        Self(raw)
    }

    /// # Errors
    /// If fd is invalid.
    pub fn from_fd_no_close(raw: RawFd) -> Result<ManuallyDrop<Self>, InvalidFd> {
        let ret = Self::from_fd(raw)?;
        Ok(ManuallyDrop::new(ret))
    }

    /// # Safety
    /// The fd must be valid for the kernel.
    #[must_use]
    pub const unsafe fn from_fd_no_close_unchecked(raw: RawFd) -> ManuallyDrop<Self> {
        let ret = Self(raw);
        ManuallyDrop::new(ret)
    }

    #[must_use]
    pub const fn as_raw(&self) -> RawFd {
        self.0
    }

    pub fn close(self) {
        let _ = unistd::close(self.as_raw());
        mem::forget(self);
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = unistd::close(self.as_raw());
    }
}

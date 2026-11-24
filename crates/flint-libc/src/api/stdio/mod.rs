use core::mem;

use crate::unistd::{self, RawFd};

#[derive(PartialEq, Eq)]
pub struct File(RawFd);

impl File {
    #[must_use]
    pub(crate) const fn from_fd(raw: RawFd) -> Self {
        Self(raw)
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

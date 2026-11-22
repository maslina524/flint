use std::borrow::Cow;

use flint_wtf8::Wtf8;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Slice {
    bytes: Wtf8,
}

impl Slice {
    #[must_use]
    pub const fn from_str(s: &str) -> &Self {
        // SAFETY: Since WTF-8 is a superset of UTF-8, this always is valid.
        unsafe { Self::from_bytes_unchecked(s.as_bytes()) }
    }

    /// # Safety
    /// `value` must contain well-formed WTF-8.
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(value: &[u8]) -> &Self {
        // SAFETY: Wtf8 == Slice (repr transparent)
        unsafe { &*(&raw const *value as *const Self) }
    }

    /// # Safety
    /// `value` must contain well-formed WTF-8.
    pub unsafe fn from_mut_bytes_unchecked(value: &mut [u8]) -> &mut Self {
        // SAFETY: Wtf8 == Slice (repr transparent)
        unsafe { &mut *(&raw mut *value as *mut Self) }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.bytes.as_bytes()
    }

    /// # Errors
    /// If [`Wtf8`] is not a UTF-8 compatible string.
    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        match self.bytes.as_str() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.bytes.to_string_lossy()
    }
}

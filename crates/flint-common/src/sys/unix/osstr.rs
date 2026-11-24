use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    fmt, slice,
};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Slice {
    bytes: [u8],
}

impl Slice {
    #[must_use]
    pub const fn from_str(s: &str) -> &Self {
        // SAFETY: If a string contains \0, it is truncated at that point.
        unsafe { Self::from_bytes_unchecked(s.as_bytes()) }
    }

    #[must_use]
    pub const fn from_str_mut(s: &mut str) -> &mut Self {
        // SAFETY: If a string contains \0, it is truncated at that point.
        unsafe { Self::from_mut_bytes_unchecked(s.as_bytes_mut()) }
    }

    #[must_use]
    pub const fn from_cstr(s: &CStr) -> &Self {
        // SAFETY: No NUL in string.
        unsafe { Self::from_bytes_unchecked(s.to_bytes()) }
    }

    #[must_use]
    pub const fn from_cstr_mut(s: &mut CStr) -> &mut Self {
        #[allow(clippy::needless_pass_by_ref_mut, reason = "Mut CStr.")]
        pub const fn cstr_as_mut_bytes(cstr: &mut CStr) -> &mut [u8] {
            let len = cstr.count_bytes();
            let ptr = cstr.as_ptr().cast_mut().cast::<u8>();

            // SAFETY: The ptr is created from a CStr, and the length is determined by counting the bytes.
            unsafe { slice::from_raw_parts_mut(ptr, len) }
        }

        let mut_buf = cstr_as_mut_bytes(s);
        // SAFETY: No NUL in string
        unsafe { Self::from_mut_bytes_unchecked(mut_buf) }
    }

    pub const fn from_bytes(bytes: &[u8]) -> Option<&Self> {
        const fn contains_nul(bytes: &[u8]) -> bool {
            let mut idx = 0;
            while idx < bytes.len() {
                if bytes[idx] == 0 {
                    return true;
                }
                idx += 1;
            }
            false
        }

        if contains_nul(bytes) {
            None
        } else {
            // SAFETY: Not contains NUL.
            let slice = unsafe { Self::from_bytes_unchecked(bytes) };
            Some(slice)
        }
    }

    /// # Safety
    /// `value` must not contain null bytes, but the last byte may be a null byte.
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(value: &[u8]) -> &Self {
        // SAFETY: [u8] == Slice (repr transparent)
        unsafe { &*(&raw const *value as *const Self) }
    }

    /// # Safety
    /// `value` must not contain null bytes, but the last byte may be a null byte.
    pub const unsafe fn from_mut_bytes_unchecked(value: &mut [u8]) -> &mut Self {
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
        &self.bytes
    }

    /// # Errors
    /// If [`Wtf8`] is not a UTF-8 compatible string.
    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        let bytes = self.as_bytes();
        if bytes.is_empty() {
            return Some("");
        }

        match str::from_utf8(bytes) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    #[must_use]
    pub fn as_c_str(&self) -> CString {
        let bytes = &self.bytes;
        let mut buf = Vec::with_capacity(bytes.len() + 1);
        buf.extend_from_slice(bytes);

        // SAFETY: Create a `CString` from bytes that initially lack a NUL,
        // and then append a single NUL at the end of the string.
        unsafe { CString::from_vec_unchecked(buf) }
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        let mut iter = bytes.utf8_chunks();

        let Some(chunk) = iter.next() else {
            return Ok(());
        };
        let first_valid = chunk.valid();
        if chunk.invalid().is_empty() {
            debug_assert_eq!(first_valid.len(), bytes.len());
            return write!(f, "{first_valid}");
        }

        write!(f, "{first_valid}{}", char::REPLACEMENT_CHARACTER)?;

        for chunk in iter {
            write!(f, "{}", chunk.valid())?;
            if !chunk.invalid().is_empty() {
                write!(f, "{}", char::REPLACEMENT_CHARACTER)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Slice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"")?;

        for chunk in self.as_bytes().utf8_chunks() {
            for ch in chunk.valid().escape_debug() {
                write!(f, "{ch}")?;
            }

            for &byte in chunk.invalid() {
                write!(f, "\\x{byte:02X}")?;
            }
        }

        write!(f, "\"")
    }
}

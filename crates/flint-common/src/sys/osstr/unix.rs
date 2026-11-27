use std::{
    borrow::Cow,
    ffi::{CStr, CString, FromVecWithNulError},
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
        Self::from_bytes(s.as_bytes())
    }

    #[must_use]
    pub const fn from_cstr(s: &CStr) -> &Self {
        Self::from_bytes(s.to_bytes())
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
        Self::from_bytes_mut(mut_buf)
    }

    pub const fn from_bytes(bytes: &[u8]) -> &Self {
        // SAFETY: [u8] == Slice (repr transparent)
        unsafe { &*(&raw const *bytes as *const Self) }
    }

    pub const fn from_bytes_mut(bytes: &mut [u8]) -> &mut Self {
        // SAFETY: [u8] == Slice (repr transparent)
        unsafe { &mut *(&raw mut *bytes as *mut Self) }
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
    /// If bytes is not a UTF-8 compatible string.
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

    pub fn to_c_str(&self) -> Result<CString, FromVecWithNulError> {
        let bytes = &self.bytes;
        let mut buf = Vec::with_capacity(bytes.len() + 1);
        buf.extend_from_slice(bytes);
        buf.push(0);

        CString::from_vec_with_nul(buf)
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

#[repr(transparent)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct Buf {
    bytes: Vec<u8>,
}

impl Buf {
    #[must_use]
    pub const fn new() -> Self {
        Self { bytes: Vec::new() }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    #[must_use]
    pub fn from_c_str(s: &CStr) -> Self {
        Self::from_bytes(s.to_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: Vec::from(bytes),
        }
    }

    pub const fn from_vec(vec: Vec<u8>) -> Self {
        Self { bytes: vec }
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
        self.bytes.as_slice()
    }

    #[must_use]
    pub const fn as_inner_slice(&self) -> &Slice {
        Slice::from_bytes(self.as_bytes())
    }

    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// # Errors
    /// If bytes is not a UTF-8 compatible string.
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

    pub fn to_c_str(&self) -> Result<CString, FromVecWithNulError> {
        let bytes = &self.bytes;
        let mut buf = Vec::with_capacity(bytes.len() + 1);
        buf.extend_from_slice(bytes);
        buf.push(0);

        CString::from_vec_with_nul(buf)
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.as_bytes())
    }

    pub fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.bytes.extend(iter);
    }

    pub fn push_str(&mut self, s: &str) {
        self.extend(s.bytes());
    }

    pub fn push_char(&mut self, ch: char) {
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        self.push_str(s);
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
}

impl fmt::Display for Buf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_inner_slice(), f)
    }
}

impl fmt::Debug for Buf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_inner_slice(), f)
    }
}

use std::{
    borrow::Cow,
    ffi::{CStr, CString, FromVecWithNulError},
    fmt,
};

use crate::sys::osstr::{Buf, Slice};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct OsStr {
    slice: Slice,
}

impl OsStr {
    pub fn new<S: AsRef<Self> + ?Sized>(s: &S) -> &Self {
        s.as_ref()
    }

    pub fn new_mut<S: AsMut<Self> + ?Sized>(s: &mut S) -> &mut Self {
        s.as_mut()
    }

    #[must_use]
    pub const fn from_bytes(bytes: &[u8]) -> &Self {
        let slice = Slice::from_bytes(bytes);
        Self::from_inner(slice)
    }

    const fn from_inner(slice: &Slice) -> &Self {
        // SAFETY: Slice == OsStr (repr transparent)
        unsafe { &*(&raw const *slice as *const Self) }
    }

    const fn from_inner_mut(slice: &mut Slice) -> &mut Self {
        // SAFETY: Slice == OsStr (repr transparent)
        unsafe { &mut *(&raw mut *slice as *mut Self) }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.slice.as_bytes()
    }

    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        self.slice.to_str()
    }

    /// # Errors
    /// If Path does not comply with the rules:
    /// - Unix: Must not contain zero.
    /// - Windows: Must be valid for UTF-16.
    pub fn to_c_str(&self) -> Result<CString, FromVecWithNulError> {
        self.slice.to_c_str()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.slice.to_string_lossy()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.slice.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }
}

impl fmt::Display for OsStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.slice, f)
    }
}

impl fmt::Debug for OsStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.slice, f)
    }
}

impl AsRef<Self> for OsStr {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Self> for OsStr {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsRef<OsStr> for str {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_inner(Slice::from_str(self))
    }
}

impl AsRef<OsStr> for String {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_inner(Slice::from_str(self))
    }
}

impl AsRef<OsStr> for CStr {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_inner(Slice::from_cstr(self))
    }
}

impl AsMut<OsStr> for CStr {
    fn as_mut(&mut self) -> &mut OsStr {
        OsStr::from_inner_mut(Slice::from_cstr_mut(self))
    }
}

impl AsRef<OsStr> for [u8] {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_inner(Slice::from_bytes(self))
    }
}

impl AsMut<OsStr> for [u8] {
    fn as_mut(&mut self) -> &mut OsStr {
        OsStr::from_inner_mut(Slice::from_bytes_mut(self))
    }
}

#[repr(transparent)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct OsString {
    buf: Buf,
}

impl OsString {
    #[must_use]
    pub const fn new() -> Self {
        Self { buf: Buf::new() }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Buf::with_capacity(capacity),
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.buf.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.buf.as_bytes()
    }

    #[must_use]
    pub const fn as_os_str(&self) -> &OsStr {
        let slice = Slice::from_bytes(self.as_bytes());
        OsStr::from_inner(slice)
    }

    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf.into_bytes()
    }

    /// # Errors
    /// If bytes is not a UTF-8 compatible string.
    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        self.buf.to_str()
    }

    /// # Errors
    /// If the string contains a NUL.
    pub fn to_c_str(&self) -> Result<CString, FromVecWithNulError> {
        self.buf.to_c_str()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.buf.to_string_lossy()
    }

    pub fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.buf.extend(iter);
    }

    pub fn push_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn push_char(&mut self, ch: char) {
        self.buf.push_char(ch);
    }

    pub fn push_byte(&mut self, byte: u8) {
        self.buf.push_byte(byte);
    }
}

impl From<&str> for OsString {
    fn from(value: &str) -> Self {
        Self {
            buf: Buf::from_str(value),
        }
    }
}

impl From<String> for OsString {
    fn from(value: String) -> Self {
        Self {
            buf: Buf::from_vec(value.into_bytes()),
        }
    }
}

impl From<&CStr> for OsString {
    fn from(value: &CStr) -> Self {
        Self {
            buf: Buf::from_c_str(value),
        }
    }
}

impl From<CString> for OsString {
    fn from(value: CString) -> Self {
        Self {
            buf: Buf::from_vec(value.into_bytes()),
        }
    }
}

impl From<&[u8]> for OsString {
    fn from(value: &[u8]) -> Self {
        Self {
            buf: Buf::from_bytes(value),
        }
    }
}

impl From<Vec<u8>> for OsString {
    fn from(value: Vec<u8>) -> Self {
        Self {
            buf: Buf::from_vec(value),
        }
    }
}

impl fmt::Display for OsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_os_str(), f)
    }
}

impl fmt::Debug for OsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_os_str(), f)
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, ffi::CStr};

    use crate::ffi::{OsStr, OsString};

    #[test]
    fn create_new() {
        let osstr = OsStr::new("Привет Мир!");
        assert_eq!(osstr.to_str(), Some("Привет Мир!"));

        let osstr = OsStr::new(c"OsStr from CStr");
        assert_eq!(osstr.to_str(), Some("OsStr from CStr"));
    }

    #[test]
    fn as_c_str_new() {
        let c_string = cr"From &str wo \0".to_owned();
        let os_str = OsStr::new(r"From &str wo \0");
        let c_str = os_str.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, Cow::<'_, CStr>::Owned(c_string));

        let os_str = OsStr::new(c"OsStr from CStr");
        let c_str = os_str.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, Cow::<'_, CStr>::Borrowed(c"OsStr from CStr"));
    }

    #[test]
    fn display_and_debug() {
        let osstr = OsStr::from_bytes(b"foo\xED\xA0\x80bar");
        assert_eq!(format!("{osstr}"), "foo���bar");
        assert_eq!(format!("{osstr:?}"), r#""foo\xED\xA0\x80bar""#);
    }

    #[test]
    fn str_with_bul() {
        let text = "Hello\0World";
        let path = OsStr::new(text);
        assert!(path.to_c_str().is_err());
    }

    #[test]
    fn push_and_push_str() {
        let mut osstring = OsString::with_capacity(32);
        osstring.push_str("Hello");
        osstring.push_char(' ');
        osstring.push_str("World");
        osstring.push_byte(b'!');

        assert_eq!(osstring.to_str(), Some("Hello World!"));
    }
}

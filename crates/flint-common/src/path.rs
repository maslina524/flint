use std::{
    borrow::{Borrow, Cow},
    ffi::{CStr, CString, FromVecWithNulError},
    fmt,
};

use crate::ffi::{OsStr, OsString};

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Path {
    inner: OsStr,
}

impl Path {
    pub fn new<T: AsRef<Self> + ?Sized>(v: &T) -> &Self {
        v.as_ref()
    }

    pub fn from_os_str<T: AsRef<OsStr> + ?Sized>(v: &T) -> &Self {
        // SAFETY: OsStr == Path (repr transparent)
        unsafe { &*(&raw const *v.as_ref() as *const Self) }
    }

    pub fn from_mut_os_str<T: AsMut<OsStr> + ?Sized>(v: &mut T) -> &mut Self {
        // SAFETY: OsStr == Path (repr transparent)
        unsafe { &mut *(&raw mut *v.as_mut() as *mut Self) }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        self.inner.to_str()
    }

    /// # Errors
    /// If Path does not comply with the rules:
    /// - Unix: Must not contain zero.
    /// - Windows: Must be valid for UTF-16.
    pub fn to_c_str(&self) -> Result<CString, FromVecWithNulError> {
        self.inner.to_c_str()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.inner.to_string_lossy()
    }

    #[must_use]
    pub fn to_pathbuf(&self) -> PathBuf {
        let bytes = self.inner.as_bytes();
        PathBuf::from(bytes)
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

macro_rules! impl_asref_for_path {
    ($($T:ty),* $(,)?) => {
        $(
            impl AsRef<Path> for $T {
                fn as_ref(&self) -> &Path {
                    Path::from_os_str(self)
                }
            }
        )*
    };
}

impl AsRef<Self> for Path {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Path> for [u8] {
    fn as_ref(&self) -> &Path {
        Path::from_os_str(self)
    }
}

impl<const N: usize> AsRef<Path> for [u8; N] {
    fn as_ref(&self) -> &Path {
        Path::from_os_str(&self[..])
    }
}

impl_asref_for_path!(str, String, CStr);

impl ToOwned for Path {
    type Owned = PathBuf;
    fn to_owned(&self) -> Self::Owned {
        self.to_pathbuf()
    }
}

impl PartialEq<PathBuf> for &Path {
    fn eq(&self, other: &PathBuf) -> bool {
        *self == other.as_path()
    }
}

#[repr(transparent)]
#[derive(Default, Clone, PartialEq, Eq)]
pub struct PathBuf {
    inner: OsString,
}

impl PathBuf {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: OsString::new(),
        }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: OsString::with_capacity(capacity),
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    #[must_use]
    pub const fn as_path(&self) -> &Path {
        let bytes = self.as_bytes();
        // SAFETY: [u8] == OsStr == Path (repr transparent)
        unsafe { &*(&raw const *bytes as *const Path) }
    }

    /// # Errors
    /// If bytes is not a UTF-8 compatible string.
    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        self.inner.to_str()
    }

    /// # Errors
    /// If the string contains a NUL.
    pub fn to_c_str(&self) -> Result<CString, FromVecWithNulError> {
        self.inner.to_c_str()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.inner.to_string_lossy()
    }
}

impl fmt::Display for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_path(), f)
    }
}

impl fmt::Debug for PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_path(), f)
    }
}

impl From<&Path> for PathBuf {
    fn from(value: &Path) -> Self {
        Self {
            inner: OsString::from(value.as_bytes()),
        }
    }
}

impl From<&str> for PathBuf {
    fn from(value: &str) -> Self {
        Self {
            inner: OsString::from(value),
        }
    }
}

impl From<String> for PathBuf {
    fn from(value: String) -> Self {
        Self {
            inner: OsString::from(value),
        }
    }
}

impl From<&CStr> for PathBuf {
    fn from(value: &CStr) -> Self {
        Self {
            inner: OsString::from(value),
        }
    }
}

impl From<CString> for PathBuf {
    fn from(value: CString) -> Self {
        Self {
            inner: OsString::from(value),
        }
    }
}

impl From<&[u8]> for PathBuf {
    fn from(value: &[u8]) -> Self {
        Self {
            inner: OsString::from(value),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for PathBuf {
    fn from(value: &[u8; N]) -> Self {
        Self {
            inner: OsString::from(&value[..]),
        }
    }
}

impl From<Vec<u8>> for PathBuf {
    fn from(value: Vec<u8>) -> Self {
        Self {
            inner: OsString::from(value),
        }
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

impl PartialEq<Path> for PathBuf {
    fn eq(&self, other: &Path) -> bool {
        self.as_path() == other
    }
}

#[cfg(test)]
mod tests {
    use crate::path::{Path, PathBuf};

    #[test]
    fn path_display_and_debug() {
        let path = Path::new("/home/rustling/Projects/flint");
        assert_eq!(format!("{path}"), "/home/rustling/Projects/flint");
        assert_eq!(format!("{path:?}"), r#""/home/rustling/Projects/flint""#);
    }

    #[test]
    fn path_as_c_str() {
        let path = Path::new("/home/rustling/path/from/str");
        let c_str = path.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, c"/home/rustling/path/from/str");

        let path = Path::new(c"/home/rustling/path/from/c_str");
        let c_str = path.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, c"/home/rustling/path/from/c_str");
    }

    #[test]
    fn path_len_and_is_empty() {
        let path = Path::new("/home/rustling/");
        assert_eq!(path.len(), 15);
        assert!(!path.is_empty());

        let path = Path::new(c"/home/rustling/");
        assert_eq!(path.len(), 15);
        assert!(!path.is_empty());

        assert!(Path::new("").is_empty());
        assert!(Path::new(c"").is_empty());
    }

    #[test]
    fn path_to_string_lossy() {
        let bytes = b"/home/some/path/\xED\xA0\x80/";
        let path = Path::new(bytes);
        let lossy = path.to_string_lossy();
        assert_eq!(lossy, "/home/some/path/���/");
    }

    #[test]
    fn pathbuf_display_and_debug() {
        let pathbuf = PathBuf::from("/home/pathbuf/");
        assert_eq!(format!("{pathbuf}"), "/home/pathbuf/");
        assert_eq!(format!("{pathbuf:?}"), r#""/home/pathbuf/""#);
    }

    #[test]
    fn pathbuf_as_c_str() {
        let pathbuf = PathBuf::from("/home/rustling/path/from/str");
        let c_str = pathbuf.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, c"/home/rustling/path/from/str");

        let pathbuf = PathBuf::from(c"/home/rustling/path/from/c_str");
        let c_str = pathbuf.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, c"/home/rustling/path/from/c_str");
    }

    #[test]
    fn pathbuf_len_and_is_empty() {
        let pathbuf = PathBuf::from("/home/rustling/");
        assert_eq!(pathbuf.len(), 15);
        assert!(!pathbuf.is_empty());

        let pathbuf = PathBuf::from(c"/home/rustling/");
        assert_eq!(pathbuf.len(), 15);
        assert!(!pathbuf.is_empty());

        assert!(Path::new("").is_empty());
        assert!(Path::new(c"").is_empty());
    }

    #[test]
    fn path_eq_pathbuf() {
        let path = Path::new("/home/some/path");
        let pathbuf = PathBuf::from("/home/some/path");
        assert_eq!(path, pathbuf);

        let path = Path::new(c"/home/some/path");
        let pathbuf = PathBuf::from(c"/home/some/path");
        assert_eq!(path, pathbuf);
    }

    #[test]
    fn pathbuf_to_string_lossy() {
        let bytes = b"/home/some/pathbuf/\xED\xA0\x80/";
        let pathbuf = PathBuf::from(bytes);
        let lossy = pathbuf.to_string_lossy();
        assert_eq!(lossy, "/home/some/pathbuf/���/");
    }
}

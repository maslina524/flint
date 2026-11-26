use std::{
    borrow::Cow,
    ffi::{CStr, CString, FromVecWithNulError},
    fmt,
};

use crate::ffi::OsStr;

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

macro_rules! impl_asrefmut_for_path {
    ($($T:ty),* $(,)?) => {
        $(
            impl AsRef<Path> for $T {
                fn as_ref(&self) -> &Path {
                    Path::from_os_str(self)
                }
            }

            impl AsMut<Path> for $T {
                fn as_mut(&mut self) -> &mut Path {
                    Path::from_mut_os_str(self)
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

impl AsMut<Self> for Path {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl_asrefmut_for_path!(str, String, CStr);

#[cfg(test)]
mod tests {
    use crate::path::Path;

    #[test]
    fn display_and_debug() {
        let path = Path::new("/home/rustling/Projects/flint");
        assert_eq!(format!("{path}"), "/home/rustling/Projects/flint");
        assert_eq!(format!("{path:?}"), r#""/home/rustling/Projects/flint""#);
    }

    #[test]
    fn as_c_str() {
        let path = Path::new("/home/rustling/path/from/str");
        let c_str = path.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, c"/home/rustling/path/from/str");

        let path = Path::new(c"/home/rustling/path/from/c_str");
        let c_str = path.to_c_str().unwrap_or_else(|_| unreachable!());
        assert_eq!(c_str, c"/home/rustling/path/from/c_str");
    }

    #[test]
    fn len_and_is_empty() {
        let path = Path::new("/home/rustling/");
        assert_eq!(path.len(), 15);
        assert!(!path.is_empty());

        let path = Path::new(c"/home/rustling/");
        assert_eq!(path.len(), 15);
        assert!(!path.is_empty());

        assert!(Path::new("").is_empty());
        assert!(Path::new(c"").is_empty());
    }
}

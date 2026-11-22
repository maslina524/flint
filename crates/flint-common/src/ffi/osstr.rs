use std::borrow::Cow;

use crate::imp::osstr::Slice;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct OsStr {
    slice: Slice,
}

impl OsStr {
    pub fn new<S: AsRef<Self> + ?Sized>(s: &S) -> &Self {
        s.as_ref()
    }

    const fn from_inner(slice: &Slice) -> &Self {
        // SAFETY: Slice == OsStr (repr transparent)
        unsafe { &*(&raw const *slice as *const Self) }
    }

    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        self.slice.as_str()
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        self.slice.to_string_lossy()
    }
}

impl AsRef<Self> for OsStr {
    fn as_ref(&self) -> &Self {
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

#[cfg(test)]
mod tests {
    use crate::ffi::OsStr;

    #[test]
    fn create_new() {
        let osstr = OsStr::new("Привет Мир!");
        assert_eq!(osstr.to_str(), Some("Привет Мир!"));
    }
}

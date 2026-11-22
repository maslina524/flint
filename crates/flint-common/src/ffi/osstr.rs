use std::{borrow::Cow, ffi::CStr};

use crate::sys::osstr::Slice;

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

    const fn from_inner(slice: &Slice) -> &Self {
        // SAFETY: Slice == OsStr (repr transparent)
        unsafe { &*(&raw const *slice as *const Self) }
    }

    const fn from_inner_mut(slice: &mut Slice) -> &mut Self {
        // SAFETY: Slice == OsStr (repr transparent)
        unsafe { &mut *(&raw mut *slice as *mut Self) }
    }

    #[must_use]
    pub const fn to_str(&self) -> Option<&str> {
        self.slice.to_str()
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

impl AsMut<OsStr> for str {
    fn as_mut(&mut self) -> &mut OsStr {
        OsStr::from_inner_mut(Slice::from_str_mut(self))
    }
}

impl AsRef<OsStr> for String {
    fn as_ref(&self) -> &OsStr {
        OsStr::from_inner(Slice::from_str(self))
    }
}

impl AsMut<OsStr> for String {
    fn as_mut(&mut self) -> &mut OsStr {
        OsStr::from_inner_mut(Slice::from_str_mut(self))
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

#[cfg(test)]
mod tests {
    use crate::ffi::OsStr;

    #[test]
    fn create_new() {
        let osstr = OsStr::new("Привет Мир!");
        assert_eq!(osstr.to_str(), Some("Привет Мир!"));

        let osstr = OsStr::new(c"OsStr from CStr");
        assert_eq!(osstr.to_str(), Some("OsStr from CStr"));
    }
}

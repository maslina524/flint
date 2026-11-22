use alloc::{borrow::Cow, string::String};
use core::str::Utf8Error;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
pub struct Wtf8 {
    bytes: [u8],
}

impl Wtf8 {
    #[must_use]
    pub const fn from_str(s: &str) -> &Self {
        // SAFETY: Since WTF-8 is a superset of UTF-8, this always is valid.
        unsafe { Self::from_bytes_unchecked(s.as_bytes()) }
    }

    /// # Safety
    /// `value` must contain well-formed WTF-8.
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(value: &[u8]) -> &Self {
        // SAFETY: [u8] == Wtf8 (repr transparent)
        unsafe { &*(&raw const *value as *const Self) }
    }

    /// # Safety
    /// `value` must contain well-formed WTF-8.
    pub unsafe fn from_mut_bytes_unchecked(value: &mut [u8]) -> &mut Self {
        // SAFETY: [u8] == Wtf8 (repr transparent)
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
    pub const fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(&self.bytes)
    }

    #[must_use]
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        let mut bytes = self.as_bytes();
        let mut result = match str::from_utf8(bytes) {
            Ok(valid) => return Cow::Borrowed(valid),
            Err(error) => {
                let mut s = String::with_capacity(bytes.len());
                let (valid, rest) = bytes.split_at(error.valid_up_to());

                // SAFETY: `valid` is valid UTF-8.
                let utf8 = unsafe { str::from_utf8_unchecked(valid) };
                s.push_str(utf8);
                s.push(char::REPLACEMENT_CHARACTER);

                bytes = &rest[3..];
                s
            }
        };

        loop {
            match str::from_utf8(bytes) {
                Ok(valid) => {
                    result.push_str(valid);
                    break;
                }
                Err(error) => {
                    let (valid, rest) = bytes.split_at(error.valid_up_to());
                    // SAFETY: `valid` is valid UTF-8.
                    result.push_str(unsafe { str::from_utf8_unchecked(valid) });
                    result.push(char::REPLACEMENT_CHARACTER);
                    bytes = &rest[3..];
                }
            }
        }

        Cow::Owned(result)
    }
}

impl AsRef<[u8]> for Wtf8 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<Self> for Wtf8 {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Wtf8> for str {
    fn as_ref(&self) -> &Wtf8 {
        Wtf8::from_str(self)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::Wtf8;

    #[test]
    fn from_str() {
        fn assert_wtf8_and_str(s: &str) {
            let wtf8 = Wtf8::from_str(s);
            assert_eq!(wtf8.as_str(), Ok(s));
        }

        assert_wtf8_and_str("Hello World!");
        assert_wtf8_and_str("Привет Мир!");
        assert_wtf8_and_str("Γεια σου κόσμε");
        assert_wtf8_and_str("Bonjour le monde");
        assert_wtf8_and_str("你好世界");
    }

    #[test]
    fn as_ref_wtf8() {
        fn as_ref<T: AsRef<Wtf8>>(v: T) {
            let _ = v.as_ref();
        }

        as_ref("Hello World");
        as_ref(Wtf8::from_str("Hello World"));
    }

    #[test]
    fn to_string_lossy() {
        let wtf8_bytes = [0xED, 0xA0, 0x80];
        // SAFETY: [0xED, 0xA0, 0x80] — Valid WTF-8
        let wtf8 = unsafe { Wtf8::from_bytes_unchecked(&wtf8_bytes) };

        let replace = char::REPLACEMENT_CHARACTER.to_string();
        assert_eq!(wtf8.to_string_lossy(), Cow::<'_, str>::Owned(replace));

        let valid = Wtf8::from_str("Hello World");
        assert_eq!(valid.to_string_lossy(), Cow::Borrowed("Hello World"));
    }
}

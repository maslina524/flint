mod repr_packed;
use repr_packed::Repr;

mod kind;
pub use kind::ErrorKind;

pub type RawOsError = i32;

const _: () = assert!(
    size_of::<usize>() == 8,
    "io::Error is implemented only for 64-bit systems; additional impl is required for builds targeting 32-bit and 16-bit systems"
);

pub enum ErrorData {
    Os(RawOsError),
    Simple(ErrorKind),
}

pub struct Error {
    repr: Repr,
}

impl Error {
    #[must_use]
    pub const fn from_raw_os_error(err: RawOsError) -> Self {
        Self {
            repr: Repr::new_os(err),
        }
    }

    #[must_use]
    pub const fn simple(kind: ErrorKind) -> Self {
        Self {
            repr: Repr::new_kind(kind),
        }
    }

    #[must_use]
    pub const fn raw_os_error(&self) -> Option<RawOsError> {
        let ErrorData::Os(err) = self.repr.decode_repr() else {
            return None;
        };
        Some(err)
    }

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.repr.kind()
    }
}

// ASSERT: size_of_bits::<Error>() == 63
const _: () = {
    assert!(size_of::<Option<Error>>() == 8);
};

#[cfg(test)]
mod tests {
    use crate::io::{
        Error,
        error::{ErrorKind, RawOsError},
    };

    #[test]
    fn from_os_error() {
        const OS: RawOsError = RawOsError::MAX;

        let err = Error::from_raw_os_error(OS);
        let raw = err
            .raw_os_error()
            .expect("Failed to parse tag in io::Error");
        assert_eq!(raw, OS);
    }

    #[test]
    fn simple_error() {
        let err = Error::simple(ErrorKind::AccessDenied);
        let raw = err.kind();
        assert_eq!(raw, ErrorKind::AccessDenied);
    }
}

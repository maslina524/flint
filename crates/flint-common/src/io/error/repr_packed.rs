use core::num::NonZeroUsize;
use std::mem;

use crate::io::error::{ErrorData, ErrorKind, RawOsError};

const TAG_MASK: usize = 0b11;
const TAG_OS: usize = 0b01;
const TAG_SIMPLE: usize = 0b10;

/// # Data in Memory
/// ## OS(code)
/// ```no_run
/// (code << 32) | OS_TAG
/// ```
///
/// ## Simple(kind)
/// ```no_run
/// ((kind as u64) << 32) | SIMPLE_TAG
/// ```
///
/// ## Custom(kind, dyn Error)
/// ```no_run
/// let custom = Custom::new(kind, dyn Error)
/// let boxed = Box::new(custom)
/// custom.as_ptr().addr() | CUSTOM_TAG
///
/// // If the Custom alignment is 8, then the 3 least significant bits are always zeros
/// ```
pub struct Repr(NonZeroUsize);

impl Repr {
    pub const fn new_os(err: RawOsError) -> Self {
        const _: () = assert!(TAG_OS != 0);

        let tagged = (err as usize) << 32 | TAG_OS;
        // SAFETY: TAG_OS is guaranteed not zero, safe.
        let nonzero = unsafe { NonZeroUsize::new_unchecked(tagged) };
        Self(nonzero)
    }

    pub const fn new_kind(kind: ErrorKind) -> Self {
        const _: () = assert!(TAG_SIMPLE != 0);

        let tagged = (kind as usize) << 32 | TAG_SIMPLE;
        // SAFETY: TAG_OS is guaranteed not zero, safe.
        let nonzero = unsafe { NonZeroUsize::new_unchecked(tagged) };
        Self(nonzero)
    }

    pub const fn decode_repr(&self) -> ErrorData {
        let inner = self.0.get();
        let tag = inner & TAG_MASK;
        match tag {
            TAG_OS => {
                let os = (inner >> 32) as RawOsError;
                ErrorData::Os(os)
            }
            TAG_SIMPLE => {
                let raw = (inner >> 32) as u32;
                // SAFETY: ErrorKind is repr u32, ErrorKind -> u32 -> ErrorKind, safe.
                let kind = unsafe { mem::transmute::<u32, ErrorKind>(raw) };
                ErrorData::Simple(kind)
            }
            _ => unreachable!(),
        }
    }

    pub const fn kind(&self) -> ErrorKind {
        match self.decode_repr() {
            ErrorData::Os(_) => todo!(),
            ErrorData::Simple(k) => k,
        }
    }
}

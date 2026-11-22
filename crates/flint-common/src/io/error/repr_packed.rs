use std::{marker::PhantomData, mem, ptr::NonNull};

use crate::io::error::{ErrorData, ErrorKind, RawOsError, SimpleMessage};

const TAG_MASK: usize = 0b11;
const TAG_OS: usize = 0b01;
const TAG_SIMPLE: usize = 0b10;
const TAG_MESSAGE: usize = 0b11;

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
pub struct Repr(NonNull<()>, PhantomData<SimpleMessage>);

impl Repr {
    pub const fn new_os(err: RawOsError) -> Self {
        const _: () = assert!(TAG_OS != 0);

        let tagged = (err as usize) << 32 | TAG_OS;
        // SAFETY: TAG_OS is guaranteed not zero, safe.
        let nonnull = unsafe { NonNull::new_unchecked(tagged as *mut ()) };
        Self(nonnull, PhantomData)
    }

    pub const fn new_kind(kind: ErrorKind) -> Self {
        const _: () = assert!(TAG_SIMPLE != 0);

        let tagged = (kind as usize) << 2 | TAG_SIMPLE;
        // SAFETY: TAG_SIMPLE is guaranteed not zero, safe.
        let nonnull = unsafe { NonNull::new_unchecked(tagged as *mut ()) };
        Self(nonnull, PhantomData)
    }

    pub const fn new_message(msg: &'static SimpleMessage) -> Self {
        const _: () = {
            assert!(TAG_MESSAGE != 0);
            assert!(TAG_MESSAGE <= 0b11);
        };

        let ptr = (&raw const *msg).cast::<()>();
        let tagged = ptr.wrapping_byte_add(TAG_MESSAGE);
        // SAFETY: TAG_MESSAGE is guaranteed not zero, safe.
        let nonnull = unsafe { NonNull::new_unchecked(tagged.cast_mut()) };
        Self(nonnull, PhantomData)
    }

    pub fn decode_repr(&self) -> ErrorData {
        let addr = self.0.addr().get();
        let tag = addr & TAG_MASK;
        match tag {
            TAG_OS => {
                let os = (addr >> 32) as RawOsError;
                ErrorData::Os(os)
            }
            TAG_SIMPLE => {
                let raw = addr >> 2;
                // SAFETY: ErrorKind is repr u64, ErrorKind -> u64 -> ErrorKind, safe.
                let kind = unsafe { mem::transmute::<usize, ErrorKind>(raw) };
                ErrorData::Simple(kind)
            }
            TAG_MESSAGE => {
                let clean = addr & !TAG_MASK;
                let ptr = clean as *mut SimpleMessage;
                // SAFETY: Raw is created from `&SimpleMessage`; the ref is guaranteed nonnull and can be safely read.
                let static_ref = unsafe { NonNull::new_unchecked(ptr).as_ref() };
                ErrorData::Message(static_ref)
            }
            _ => unreachable!(),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        match self.decode_repr() {
            ErrorData::Os(_) => todo!(),
            ErrorData::Simple(k) => k,
            ErrorData::Message(s) => s.kind,
        }
    }
}

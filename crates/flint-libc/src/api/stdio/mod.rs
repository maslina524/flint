mod file;
use core::fmt::{self, Write as _};

pub use file::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Perm {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Perm {
    pub const NONE: Self = Self {
        read: false,
        write: false,
        execute: false,
    };
    pub const READ: Self = Self {
        read: true,
        write: false,
        execute: false,
    };
    pub const WRITE: Self = Self {
        read: false,
        write: true,
        execute: false,
    };
    pub const EXEC: Self = Self {
        read: false,
        write: false,
        execute: true,
    };
    pub const ALL: Self = Self {
        read: true,
        write: true,
        execute: true,
    };

    #[must_use]
    pub const fn from_octal(val: u32) -> Option<Self> {
        if val > 0o7 {
            return None;
        }
        Some(Self {
            read: (val & 0o4) != 0,
            write: (val & 0o2) != 0,
            execute: (val & 0o1) != 0,
        })
    }

    #[must_use]
    pub const fn to_octal(self) -> u32 {
        let mut val = 0;
        if self.read {
            val |= 0o4;
        }
        if self.write {
            val |= 0o2;
        }
        if self.execute {
            val |= 0o1;
        }
        val
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Mode {
    pub owner: Perm,
    pub group: Perm,
    pub others: Perm,
}

impl Mode {
    #[must_use]
    pub const fn from_octal(val: u32) -> Option<Self> {
        if val > 0o777 {
            return None;
        }

        let Some(owner) = Perm::from_octal((val >> 6) & 0o7) else {
            return None;
        };
        let Some(group) = Perm::from_octal((val >> 3) & 0o7) else {
            return None;
        };
        let Some(others) = Perm::from_octal(val & 0o7) else {
            return None;
        };

        Some(Self {
            owner,
            group,
            others,
        })
    }

    #[must_use]
    pub const fn to_octal(self) -> u32 {
        (self.owner.to_octal() << 6) | (self.group.to_octal() << 3) | self.others.to_octal()
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn triplet(f: &mut fmt::Formatter<'_>, perm: Perm) -> fmt::Result {
            let read = perm.read;
            let write = perm.write;
            let execute = perm.execute;

            let vals = [read, write, execute].into_iter();
            for (val, ch) in vals.zip(['r', 'w', 'x']) {
                let print = if val { ch } else { '-' };
                f.write_char(print)?;
            }

            Ok(())
        }

        triplet(f, self.owner)?;
        triplet(f, self.group)?;
        triplet(f, self.others)
    }
}

#[cfg(test)]
mod tests {
    use crate::stdio::Mode;

    #[test]
    fn display() {
        let mode = Mode::from_octal(0o777).unwrap();
        assert_eq!(format!("{mode}"), "rwxrwxrwx");

        let mode = Mode::from_octal(0o755).unwrap();
        assert_eq!(format!("{mode}"), "rwxr-xr-x");

        let mode = Mode::from_octal(0o644).unwrap();
        assert_eq!(format!("{mode}"), "rw-r--r--");
    }
}

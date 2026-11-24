use std::{fs::File, io::Read as _, os::fd::FromRawFd as _};

use flint_libc::fcntl;

#[test]
fn open_file() {
    let fd = fcntl::open(c"Cargo.toml", 0, 0o744).expect("Faield to open fd");
    // SAFETY: Create a file from the fd returned by the kernel.
    let mut file = unsafe { File::from_raw_fd(fd) };

    // Perform any operation on a file
    let mut buf = [0u8; 16];
    let readed = file.read(&mut buf).expect("Failed to read file");

    assert_ne!(readed, 0);
    assert_ne!(&buf, &[0u8; 16]);
}

use std::{
    fs::File,
    io::{Read as _, Seek as _, SeekFrom},
    os::fd::AsRawFd as _,
};

use rustix::fs;

#[test]
fn write_to_fd() {
    const BYTES: &[u8] = b"Hello World!";

    // Create anon file
    let fd = fs::memfd_create("AnonFd", fs::MemfdFlags::empty())
        .expect("Failed to create memfd via rustix");
    let mut file = File::from(fd);

    // Write
    let written = flint_libc::unistd::write(file.as_raw_fd(), BYTES).expect("Failed to write");
    assert_eq!(written, BYTES.len());

    // Read
    file.seek(SeekFrom::Start(0))
        .expect("Failed to seek via std");
    let mut buf = [0u8; 4096];
    let readed = file.read(&mut buf).expect("Failed to read file via std");
    assert_eq!(readed, BYTES.len());
    assert_eq!(&buf[..readed], BYTES);
}

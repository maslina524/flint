mod mman {
    use std::ptr;

    use flint_libc::sys::mman::{self, MmapFlags, MmapProt};

    #[test]
    fn alloc_via_mmap() {
        let ptr = mman::mmap(
            None,
            128,
            MmapProt::new().read().write(),
            MmapFlags::new().private().anonymous(),
            -1,
            0,
        )
        .expect("Failed mmap");

        // SAFETY: We write bytes only to the allocated memory
        unsafe { ptr::write_bytes(ptr.cast::<u8>().as_ptr(), 42u8, 128) };
        for i in 0..128 {
            // SAFETY: All 128 bytes must be equal to 42
            let val = unsafe { ptr::read(ptr.cast::<u8>().add(i).as_ptr()) };
            assert_eq!(val, 42u8);
        }

        mman::munmap(ptr, 128).expect("Failed munmap");
    }
}

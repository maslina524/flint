#![allow(clippy::similar_names, reason = "C style")]

use core::{
    ffi::{c_char, c_int},
    sync::atomic::Ordering::Relaxed,
};

use crate::unistd;

/// glibc passes argc, argv, and envp to functions in `.init_array`, as a
/// non-standard extension
#[used]
#[unsafe(link_section = ".init_array.00099")]
static INIT_ARRAY_00099: InitArrayFn = init_array;

core::cfg_select! {
    any(
        feature = "stack_parse",
        not(any(target_libc = "glibc", target_libc = "bionic"))
    ) => {
        type InitArrayFn = extern "C" fn();

        extern "C" fn init_array() {
            unsafe extern "C" {
                safe static __libc_stack_end: *const core::ffi::c_void;
            }

            let sp = __libc_stack_end;
            debug_assert!(!sp.is_null());

            // SAFETY: In `.init_array` at the top of the stack lies argc of type `c_int`.
            let argc = unsafe { sp.cast::<c_int>().read() } as usize;
            debug_assert!(argc != 0);

            // SAFETY: After argc comes a pointer to `argv[0]`.
            let argv = unsafe { sp.byte_add(size_of::<usize>()).cast::<*const c_char>() };

            // SAFETY: `argv[argc..argc-1]`.
            let argv_end = unsafe { argv.add(argc) };
            // SAFETY: After argv comes NULL.
            debug_assert!(unsafe { argv_end.read() }.is_null());

            // SAFETY: After NULL comes envp[0].
            let envp = unsafe { argv_end.add(1) };

            unistd::ARGC.store(argc, Relaxed);
            unistd::ARGV.store(argv.cast_mut(), Relaxed);
            unistd::ENVP.store(envp.cast_mut(), Relaxed);
        }
    }
    _ => {
        type InitArrayFn = extern "C" fn(c_int, *const *const c_char, *const *const c_char);

        extern "C" fn init_array(argc: c_int, argv: *const *const c_char, envp: *const *const c_char) {
            unistd::ARGC.store(argc as usize, Relaxed);
            unistd::ARGV.store(argv.cast_mut(), Relaxed);
            unistd::ENVP.store(envp.cast_mut(), Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        env,
        ffi::{CStr, OsStr, OsString},
        os::unix::ffi::OsStrExt,
        sync::atomic::Ordering::Relaxed,
    };

    use crate::unistd;

    #[test]
    fn argc() {
        let my_argc = unistd::ARGC.load(Relaxed);
        assert_eq!(my_argc, env::args_os().len());
    }

    #[test]
    fn argv() {
        fn c_str_to_os_string(c_str: &CStr) -> OsString {
            OsStr::from_bytes(c_str.to_bytes()).to_os_string()
        }

        let argc = unistd::ARGC.load(Relaxed);
        let argv = unistd::ARGV.load(Relaxed);
        let mut my_args = Vec::with_capacity(argc);
        for i in 0..argc {
            // SAFETY: `argv[0..argc - 1]` is valid.
            let ptr = unsafe { argv.add(i).read() };
            // SAFETY: Ptr is valid `CStr`.
            let c_str = unsafe { CStr::from_ptr(ptr) };
            let os_string = c_str_to_os_string(c_str);
            my_args.push(os_string);
        }

        let args: Vec<OsString> = env::args_os().collect();

        assert_eq!(my_args, args);
    }

    #[test]
    fn envp() {
        let envp = unistd::ENVP.load(Relaxed);
        let mut version = None;

        let mut idx = 0;
        loop {
            // SAFETY: Until NULL ptr is valid.
            let ptr = unsafe { envp.add(idx).read() };
            if ptr.is_null() {
                break;
            }

            // SAFETY: Ptr is valid `CStr`.
            let c_str = unsafe { CStr::from_ptr(ptr) };
            let string = c_str.to_string_lossy();
            let Some(eq) = string.find('=') else {
                panic!("The string does not contain `=`, non valid");
            };

            let key = &string[..eq];
            if key == "CARGO_PKG_VERSION" {
                let ver = string[eq + 1..].to_owned();
                version = Some(ver);
            }

            idx += 1;
        }

        if let Some(ver) = version {
            assert_eq!(ver, env!("CARGO_PKG_VERSION"));
        } else {
            println!("`CARGO_PKG_VERSION` not sets");
        }
    }
}

#![allow(clippy::similar_names, reason = "C style")]
#![cfg(all(target_os = "linux", target_env = "gnu"))]

use core::{
    ffi::{c_char, c_int},
    sync::atomic::Ordering::Relaxed,
};

use crate::unistd;

type InitArrayFn = extern "C" fn(c_int, *const *const c_char, *const *const c_char);

/// glibc passes argc, argv, and envp to functions in `.init_array`, as a
/// non-standard extension
#[used]
#[unsafe(link_section = ".init_array.00099")]
static INIT_ARRAY_00099: InitArrayFn = init_array_wrapper;

extern "C" fn init_array_wrapper(
    argc: c_int,
    argv: *const *const c_char,
    envp: *const *const c_char,
) {
    init(argc as usize, argv, envp);
}

fn init(argc: usize, argv: *const *const c_char, envp: *const *const c_char) {
    unistd::ARGC.store(argc, Relaxed);
    unistd::ARGV.store(argv.cast_mut(), Relaxed);
    unistd::ENVP.store(envp.cast_mut(), Relaxed);
}

#[cfg(test)]
mod tests {
    use core::{ptr, sync::atomic::Ordering::Relaxed};

    use crate::unistd;

    #[test]
    fn argc_argv_initialized() {
        assert_ne!(unistd::ARGC.load(Relaxed), 0);
        assert_ne!(unistd::ARGV.load(Relaxed), ptr::null_mut());
    }
}

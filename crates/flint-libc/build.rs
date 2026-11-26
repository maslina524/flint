use std::env;

macro_rules! enum_libctype {
    ($($name:ident => $string:literal),+ $(,)?) => {
        #[allow(unused, reason = "Unused macro")]
        enum LibcType {
            $($name),+
        }

        impl LibcType {
            #[allow(unused, reason = "Unused fn")]
            pub const fn to_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $string),+
                }
            }

            #[allow(unused, reason = "Unused fn")]
            pub const fn separated_names() -> &'static str {
                ::core::concat!(
                    $("\"", $string, "\", "),+
                )
            }
        }

        impl ::core::fmt::Display for LibcType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{}", self.to_str())
            }
        }

        impl ::core::fmt::Debug for LibcType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "\"{}\"", self.to_str())
            }
        }
    };
}

enum_libctype!(
    Glibc => "glibc",
    Musl => "musl",
    UClibc => "uclibc",
    NewLib => "newlib",
    Msvc => "msvc",
    Bionic => "bionic",
    Apple => "apple",
    Bsd => "bsd",
    Wasi => "wasi",
    None => "none",
    Unknown => "unknown",
);

fn match_libc(os: &str, env: &str) -> LibcType {
    use LibcType as T;

    match (os, env) {
        (_, "gnu") => T::Glibc,
        (_, "musl") => T::Musl,
        (_, "uclibc") => T::UClibc,
        (_, "newlib") => T::NewLib,
        (_, "msvc") => T::Msvc,
        ("android", _) => T::Bionic,
        ("macos" | "ios" | "tvos" | "watchos" | "visionos", _) => T::Apple,
        ("freebsd" | "openbsd" | "netbsd" | "dragonfly", _) => T::Bsd,
        ("wasi", _) => T::Wasi,
        ("none", _) => T::None,
        _ => T::Unknown,
    }
}

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .unwrap_or_else(|_| panic!("`CARGO_CFG_TARGET_OS` not sets!"));
    let target_env = env::var("CARGO_CFG_TARGET_ENV")
        .unwrap_or_else(|_| panic!("`CARGO_CFG_TARGET_ENV` not sets!"));

    let libc = match_libc(&target_os, &target_env);
    println!(
        "cargo::rustc-check-cfg=cfg(target_libc, values({}))",
        LibcType::separated_names()
    );
    println!("cargo::rustc-cfg=target_libc={libc:?}");

    println!("cargo::rerun-if-changed=build.rs");
}

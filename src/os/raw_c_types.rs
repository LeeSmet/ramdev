/// Code here is taken from https://doc.rust-lang.org/src/std/os/raw/mod.rs.html,
/// since we can't use the std library
// Stripped away most of the attributes such as stable as these are not supported
// outside of std
// #[cfg(any(all(target_os = "linux", any(target_arch = "aarch64",
//                                        target_arch = "arm",
//                                        target_arch = "powerpc",
//                                        target_arch = "powerpc64",
//                                        target_arch = "s390x")),
//           all(target_os = "android", any(target_arch = "aarch64",
//                                          target_arch = "arm")),
//           all(target_os = "l4re", target_arch = "x86_64"),
//           all(target_os = "openbsd", target_arch = "aarch64"),
//           all(target_os = "fuchsia", target_arch = "aarch64")))]
// pub type c_char = u8;
// #[cfg(not(any(all(target_os = "linux", any(target_arch = "aarch64",
//                                            target_arch = "arm",
//                                            target_arch = "powerpc",
//                                            target_arch = "powerpc64",
//                                            target_arch = "s390x")),
//               all(target_os = "android", any(target_arch = "aarch64",
//                                              target_arch = "arm")),
//               all(target_os = "l4re", target_arch = "x86_64"),
//               all(target_os = "openbsd", target_arch = "aarch64"),
//               all(target_os = "fuchsia", target_arch = "aarch64"))))]
// pub type c_char = i8;
//

// workaround for now to get the main module to compile
pub type c_char = u8;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
#[cfg(any(target_pointer_width = "32", windows))]
pub type c_long = i32;
#[cfg(any(target_pointer_width = "32", windows))]
pub type c_ulong = u32;
#[cfg(all(target_pointer_width = "64", not(windows)))]
pub type c_long = i64;
#[cfg(all(target_pointer_width = "64", not(windows)))]
pub type c_ulong = u64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type c_float = f32;
pub type c_double = f64;

#[repr(u8)]
pub enum c_void {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

#![allow(unused_variables)]
#![allow(dead_code)]

mod application;
mod comdef;
mod opengujian;
mod openpal3;
mod openpal4;
mod openpal5;
mod openswd5;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn android_entry() {
    openpal3::run_openpal3();
}

// MoltenVK (statically linked) calls ___isPlatformVersionAtLeast, a clang runtime
// helper. rustc passes -nodefaultlibs on iOS, so the compiler-rt runtime that
// normally provides this symbol is NOT linked, and Xcode 26's libclang_rt stub
// can't be consumed via rustc's `static=` link mode. On the target device (iOS 26)
// every @available(...) check resolves to true, so a constant-true shim is
// semantically correct and keeps the link self-contained.
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn ___isPlatformVersionAtLeast(
    _platform: u32,
    _major: u32,
    _minor: u32,
    _subminor: u32,
) -> bool {
    true
}

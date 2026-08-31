#![allow(unused_variables)]
#![allow(dead_code)]

use application::run_title_selection;
use opengujian::run_opengujian;
use openpal3::run_openpal3;
use openpal4::run_openpal4;
use openpal5::run_openpal5;
use openswd5::run_openswd5;
use playground::run_test;
use shared::video::register_opengb_video_decoders;

mod application;
mod comdef;
mod opengujian;
mod openpal3;
mod openpal4;
mod openpal5;
mod openswd5;
mod playground;

pub fn main() {
    radiance::application::Application::set_panic_hook();
    init_logger();
    register_opengb_video_decoders();

    // Keep the MoltenVK-referenced ___isPlatformVersionAtLeast shim alive (see
    // definition above). Volatile store from the live entry point guarantees the
    // symbol survives both LLVM DCE and ld -dead_strip.
    #[cfg(target_os = "ios")]
    unsafe {
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!(IOS_SHIM_KEEP),
            ___isPlatformVersionAtLeast as *const (),
        );
    }

    #[cfg(vita)]
    {
        run_openpal4();
    }

    #[cfg(not(vita))]
    {
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() <= 1 {
            run_title_selection();
        } else {
            match args[1].as_str() {
                "--pal3" => {
                    run_openpal3();
                }
                "--pal4" => {
                    run_openpal4();
                }
                "--pal5" => {
                    run_openpal5();
                }
                "--pal5q" => {
                    run_openpal5();
                }
                "--swd5" => {
                    run_openswd5();
                }
                "--gujian" => {
                    run_opengujian();
                }
                "--test" => {
                    run_test();
                }
                &_ => {}
            }
        }
    }
}

fn init_logger() {
    #[cfg(any(windows, linux, macos, android, ios))]
    {
        let logger = simple_logger::SimpleLogger::new();
        // workaround panic on Linux for 'Could not determine the UTC offset on this system'
        // see: https://github.com/borntyping/rust-simple_logger/issues/47
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "android", target_os = "ios"))]
        let logger = logger.with_utc_timestamps();
        logger.init().unwrap();
    }

    #[cfg(vita)]
    {
        let logger = simplelog::WriteLogger::new(
            simplelog::LevelFilter::Error,
            simplelog::Config::default(),
            std::fs::File::create("ux0:data/yaobow.log").unwrap(),
        );

        simplelog::CombinedLogger::init(vec![logger]).unwrap();
    }
}

#[used]
#[export_name = "_newlib_heap_size_user"]
pub static _NEWLIB_HEAP_SIZE_USER: u32 = 216 * 1024 * 1024;

// MoltenVK (statically linked) calls ___isPlatformVersionAtLeast, a clang runtime
// helper. rustc passes -nodefaultlibs on iOS, so the compiler-rt runtime that
// normally provides this symbol is NOT linked, and Xcode 26's libclang_rt stub
// can't be consumed via rustc's `static=` link mode. On the target device (iOS 26)
// every @available(...) check resolves to true, so a constant-true shim is
// semantically correct and keeps the link self-contained. Lives in the bin crate
// (main.rs) because the binary does not link the lib crate.
#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn ___isPlatformVersionAtLeast(
    _platform: u32,
    _major: u32,
    _minor: u32,
    _subminor: u32,
) -> i32 {
    1
}

// Force the linker to keep the shim above. The shim is referenced by the
// statically linked libMoltenVK.a, but nothing in Rust calls it, so both LLVM
// (release DCE) and ld -dead_strip would discard it, leaving the symbol
// undefined. main() writes its address into this #[used] static via a volatile
// store, making the symbol reachable from the live entry point so neither the
// compiler nor the linker can drop it.
#[cfg(target_os = "ios")]
#[used]
static mut IOS_SHIM_KEEP: *const () = core::ptr::null();

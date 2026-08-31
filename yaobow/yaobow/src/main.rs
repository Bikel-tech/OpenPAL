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

// On iOS a panic otherwise just closes the app with no explanation. Capture the
// message to a file in the app's Documents (exposed via UIFileSharingEnabled) and
// to stderr (visible in the device console) so we can diagnose launch crashes.
#[cfg(target_os = "ios")]
fn install_ios_crash_logger() {
    std::panic::set_hook(Box::new(|info| {
        let loc = info.location().map(|l| l.to_string()).unwrap_or_default();
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            (*s).to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            format!("{:?}", info.payload())
        };
        let msg = format!("[yaobow panic] {} @ {}\n", payload, loc);
        eprintln!("{}", msg);
        if let Ok(home) = std::env::var("HOME") {
            let _ = std::fs::create_dir_all(format!("{}/Documents", home));
            let path = format!("{}/Documents/yaobow_crash.log", home);
            let _ = std::fs::write(&path, &msg);
        }
    }));
}

pub fn main() {
    radiance::application::Application::set_panic_hook();
    install_ios_crash_logger();
    init_logger();
    register_opengb_video_decoders();

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

// MoltenVK (statically linked) references ___isPlatformVersionAtLeast, a clang
// (compiler-rt) runtime helper that rustc does NOT link on iOS because it passes
// -nodefaultlibs. Defining it via global_asm guarantees the symbol is emitted into
// the object file unconditionally (Rust's LLVM release-DCE would otherwise drop an
// unused #[no_mangle] fn, leaving the symbol undefined at link time). On iOS 26
// every @available(...) check resolves true, so returning 1 is semantically
// correct. The linker is told to export the symbol (see -exported_symbol in
// ios-build.yml) so -dead_strip cannot remove it.
#[cfg(target_os = "ios")]
core::arch::global_asm!(
    ".globl ___isPlatformVersionAtLeast",
    ".p2align 2",
    "___isPlatformVersionAtLeast:",
    "mov w0, #1",
    "ret",
);

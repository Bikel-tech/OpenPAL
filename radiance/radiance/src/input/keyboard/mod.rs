#[cfg(vita)]
mod nop;
#[cfg(windows)]
mod windows;
#[cfg(any(linux, macos, android, ios))]
mod winit;

#[cfg(any(linux, macos, android, ios))]
pub use self::winit::KeyboardInput;
#[cfg(vita)]
pub use nop::KeyboardInput;
#[cfg(windows)]
pub use windows::KeyboardInput;

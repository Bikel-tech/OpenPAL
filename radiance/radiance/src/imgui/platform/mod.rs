#[cfg(vita)]
mod vita;
#[cfg(windows)]
mod windows;
#[cfg(any(linux, macos, android, ios))]
mod winit;

#[cfg(any(linux, macos, android, ios))]
pub use self::winit::ImguiPlatform;
#[cfg(vita)]
pub use vita::ImguiPlatform;
#[cfg(windows)]
pub use windows::ImguiPlatform;

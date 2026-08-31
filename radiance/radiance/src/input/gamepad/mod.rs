#[cfg(any(windows, linux, macos, android, ios))]
mod gilrs;
#[cfg(vita)]
mod vita;

#[cfg(any(windows, linux, macos, android, ios))]
pub use self::gilrs::GilrsInput as GamepadInput;
#[cfg(vita)]
pub use vita::VitaGamepadInput as GamepadInput;

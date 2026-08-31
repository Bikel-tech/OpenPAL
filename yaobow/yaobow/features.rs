use cfg_aliases::cfg_aliases;

pub fn enable_features() {
    cfg_aliases! {
        // Platforms
        windows: { target_os = "windows" },
        linux: { target_os = "linux" },
        macos: { target_os = "macos" },
        android: { target_os = "android" },
        ios: { target_os = "ios" },
        vita: { target_os= "vita" },

        // Graphic Backends
        vulkan: { any(windows, linux, macos, android, ios) },
        vitagl: { vita }
    }
}

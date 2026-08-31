use std::process::Command;

mod features;

fn main() {
    features::enable_features();

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "windows" | "linux" | "macos" | "android" | "ios" => {
            build_vulkan_shader("simple_triangle.vert");
            build_vulkan_shader("simple_triangle.frag");
            build_vulkan_shader("lightmap_texture.vert");
            build_vulkan_shader("lightmap_texture.frag");
        }
        _ => {}
    }
}

#[allow(dead_code)]
fn build_vulkan_shader(shader_name: &str) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::fs::canonicalize(
        std::path::PathBuf::from(manifest_dir)
            .join("src/rendering/vulkan/shaders")
            .join(shader_name),
    )
    .unwrap();
    println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    let shader_out = format!("{}/{}.spv", out_dir, shader_name);
    let path_str = path.to_str().unwrap();

    // Preferred: glslc (Vulkan SDK). Fallback: glslangValidator (brew install glslang),
    // which writes <name>.spv into the current directory.
    let compiled = {
        if let Ok(status) = Command::new("glslc")
            .args([path_str, "-o", &shader_out])
            .status()
        {
            status.success()
        } else {
            false
        }
    };

    if compiled {
        println!("cargo:warning=compiled {} with glslc", shader_name);
        return;
    }

    // Fallback: glslangValidator (brew install glslang). It accepts -o just like
    // glslc. Without -o it writes the .spv next to the *source*, which is wrong.
    let glslang_ok = Command::new("glslangValidator")
        .arg("-V")
        .arg(path_str)
        .arg("-o")
        .arg(&shader_out)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if glslang_ok {
        println!("cargo:warning=compiled {} with glslangValidator", shader_name);
        return;
    }

    // Last resort: a precompiled .spv committed alongside the source (no compiler
    // needed at build time). Keeps CI independent of Vulkan SDK / glslang.
    let prebuilt = std::path::PathBuf::from(manifest_dir)
        .join("src/rendering/vulkan/shaders")
        .join(format!("{}.spv", shader_name));
    if prebuilt.exists() {
        std::fs::copy(&prebuilt, &shader_out).expect("copy prebuilt spv");
        println!("cargo:warning=used prebuilt {} .spv", shader_name);
        return;
    }

    panic!(
        "Failed to compile shader {}: no glslc, glslangValidator failed, and no prebuilt .spv present",
        shader_name
    );
}

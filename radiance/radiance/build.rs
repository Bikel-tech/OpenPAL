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

    let _ = Command::new("glslangValidator")
        .current_dir(&out_dir)
        .arg("-V")
        .arg(path_str)
        .status()
        .expect(&format!(
            "Failed to compile shader {} with glslc or glslangValidator",
            shader_name
        ));
}

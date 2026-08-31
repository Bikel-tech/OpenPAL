import os, re, glob

ROOT = os.path.dirname(os.path.abspath(__file__))

# 1) 通用 any(...) 列表补 ios（排除 target/）
generic = [
    (r'any\(linux, macos, android\)', 'any(linux, macos, android, ios)'),
    (r'any\(windows, linux, macos, android\)', 'any(windows, linux, macos, android, ios)'),
    (r'any\(windows, linux, macos\)', 'any(windows, linux, macos, ios)'),
    (r'any\(target_os = "linux", target_os = "macos", target_os = "android"\)',
     'any(target_os = "linux", target_os = "macos", target_os = "android", target_os = "ios")'),
    (r'any\(target_os = "macos", target_os = "android"\)',
     'any(target_os = "macos", target_os = "android", target_os = "ios")'),
]

def patch_generic(path):
    with open(path, 'r', encoding='utf-8') as f:
        s = f.read()
    orig = s
    for pat, rep in generic:
        s = re.sub(pat, rep, s)
    if s != orig:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(s)
        return sum(1 for _ in re.finditer(pat, orig) for pat, _ in [])  # rough count
    return 0

print("=== 通用 any(...) 补 ios ===")
count = 0
for p in glob.glob(os.path.join(ROOT, '**', '*.rs'), recursive=True):
    if '/target/' in p.replace('\\','/'):
        continue
    if patch_generic(p):
        count += 1
print(f"  修改文件数: {count}")

# 2) features.rs: 加 ios 平台 + vulkan 含 ios
print("=== features.rs ===")
for p in glob.glob(os.path.join(ROOT, '**', 'features.rs'), recursive=True):
    if '/target/' in p.replace('\\','/'):
        continue
    with open(p, 'r', encoding='utf-8') as f:
        s = f.read()
    s2 = s.replace(
        '        vita: { target_os= "vita" },',
        '        ios: { target_os = "ios" },\n        vita: { target_os= "vita" },')
    s2 = s2.replace(
        'vulkan: { any(windows, linux, macos, android) },',
        'vulkan: { any(windows, linux, macos, android, ios) },')
    if s2 != s:
        with open(p, 'w', encoding='utf-8') as f:
            f.write(s2)
        print(f"  已更新: {os.path.relpath(p, ROOT)}")

# 3) helpers.rs: iOS instance 扩展 = macOS (MetalSurface + portability)
print("=== helpers.rs (iOS surface) ===")
hp = os.path.join(ROOT, 'radiance', 'radiance', 'src', 'rendering', 'vulkan', 'helpers.rs')
with open(hp, 'r', encoding='utf-8') as f:
    s = f.read()
macos_block = '''#[cfg(target_os = "macos")]
pub fn instance_extension_names() -> Vec<*const c_char> {
    vec![
        ash::extensions::khr::Surface::name().as_ptr(),
        ash::extensions::ext::MetalSurface::name().as_ptr(),
        ash::extensions::ext::DebugUtils::name().as_ptr(),
        ash::vk::KhrPortabilityEnumerationFn::name().as_ptr(),
        ash::vk::KhrGetPhysicalDeviceProperties2Fn::name().as_ptr(),
    ]
}'''
ios_block = macos_block.replace('#[cfg(target_os = "macos")]', '#[cfg(target_os = "ios")]')
if macos_block in s and '#[cfg(target_os = "ios")]' not in s:
    s = s.replace(macos_block, macos_block + '\n' + ios_block)
    with open(hp, 'w', encoding='utf-8') as f:
        f.write(s)
    print("  已加 iOS instance_extension_names (MetalSurface)")
else:
    print("  (跳过: 已存在或无匹配)")

# 4) video/mod.rs: iOS 下摘掉 ffmpeg
print("=== video/mod.rs (iOS 摘 ffmpeg) ===")
vp = os.path.join(ROOT, 'yaobow', 'shared', 'src', 'video', 'mod.rs')
with open(vp, 'r', encoding='utf-8') as f:
    s = f.read()
old = '''mod ffmpeg;

pub fn register_opengb_video_decoders() {
    use radiance::video::{register_video_decoder, Codec};
    register_video_decoder(Codec::Bik, ffmpeg::VideoStreamFFmpeg::create);
}'''
new = '''#[cfg(not(target_os = "ios"))]
mod ffmpeg;

#[cfg(not(target_os = "ios"))]
pub fn register_opengb_video_decoders() {
    use radiance::video::{register_video_decoder, Codec};
    register_video_decoder(Codec::Bik, ffmpeg::VideoStreamFFmpeg::create);
}

#[cfg(target_os = "ios")]
pub fn register_opengb_video_decoders() {
    // iOS 可行性阶段: ffmpeg 媒体层尚未接入, 占位
}'''
if old in s:
    s = s.replace(old, new)
    with open(vp, 'w', encoding='utf-8') as f:
        f.write(s)
    print("  已加 iOS 占位 (摘 ffmpeg)")
else:
    print("  (跳过: 无匹配)")

# 5) imgui/mod.rs: iOS 用空剪贴板
print("=== imgui/mod.rs (iOS clipboard_nop) ===")
ip = os.path.join(ROOT, 'radiance', 'radiance', 'src', 'imgui', 'mod.rs')
with open(ip, 'r', encoding='utf-8') as f:
    s = f.read()
if 'any(android, vita)' in s and 'ios' not in s.split('any(android, vita)')[0][-40:]:
    s = s.replace('#[cfg_attr(any(android, vita), path = "clipboard_nop.rs")]',
                  '#[cfg_attr(any(android, vita, ios), path = "clipboard_nop.rs")]')
    with open(ip, 'w', encoding='utf-8') as f:
        f.write(s)
    print("  已加 iOS 到 clipboard_nop")
else:
    print("  (跳过)")

print("=== 完成 ===")

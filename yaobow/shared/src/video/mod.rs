#[cfg(not(target_os = "ios"))]
mod ffmpeg;

#[cfg(not(target_os = "ios"))]
pub fn register_opengb_video_decoders() {
    use radiance::video::{register_video_decoder, Codec};
    register_video_decoder(Codec::Bik, ffmpeg::VideoStreamFFmpeg::create);
}

#[cfg(target_os = "ios")]
pub fn register_opengb_video_decoders() {
    // iOS 可行性阶段: ffmpeg 媒体层尚未接入, 占位
}

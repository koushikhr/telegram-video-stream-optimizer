use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProbe {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub format_name: String,
    pub duration_seconds: f64,
    pub bit_rate: Option<u64>,
    pub video_streams: Vec<VideoStream>,
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub is_telegram_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStream {
    pub index: usize,
    pub codec_name: String,
    pub profile: Option<String>,
    pub width: u32,
    pub height: u32,
    pub pix_fmt: String,
    pub r_frame_rate: String,
    pub bit_rate: Option<u64>,
    pub is_10bit: bool,
    pub is_hdr: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub stream_index: usize,
    pub track_index: usize,
    pub codec_name: String,
    pub language: String,
    pub title: String,
    pub channels: u32,
    pub channel_layout: String,
    pub bit_rate: Option<u64>,
    pub is_default: bool,
    pub is_commentary: bool,
    pub is_hearing_impaired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub stream_index: Option<usize>,
    pub track_index: usize,
    pub codec_name: String,
    pub language: String,
    pub title: String,
    pub is_default: bool,
    pub is_forced: bool,
    pub is_hearing_impaired: bool,
    pub is_external: bool,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodeStrategy {
    DirectRemux,
    TranscodeH264,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareEncoder {
    Auto,
    NvidiaNvenc,
    AmdAmf,
    IntelQsv,
    AppleVideoToolbox,
    CpuX264,
}

impl HardwareEncoder {
    pub fn as_ffmpeg_encoder(&self) -> &'static str {
        match self {
            Self::NvidiaNvenc => "h264_nvenc",
            Self::AmdAmf => "h264_amf",
            Self::IntelQsv => "h264_qsv",
            Self::AppleVideoToolbox => "h264_videotoolbox",
            Self::CpuX264 | Self::Auto => "libx264",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleBurnConfig {
    pub enabled: bool,
    pub track_index: Option<usize>,
    pub font_size_pt: u32, // Default 24 (range 16 to 40)
    pub custom_margin_v: u32, // Default 28
    pub border_style: u32, // 1 = Clean Outline (Netflix), 3 = Black Box
}

impl Default for SubtitleBurnConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            track_index: None,
            font_size_pt: 24,
            custom_margin_v: 28,
            border_style: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSettings {
    pub preferred_audio_lang: String,       // e.g. "en"
    pub preferred_subtitle_lang: String,    // e.g. "en"
    pub target_size_mb: u64,                // Default 1980 (Free) or 3980 (Premium)
    pub max_resolution: Option<u32>,        // e.g. 1080, 720, or None (Original)
    pub subtitle_font_size: u32,            // Default 24
    pub hardware_encoder: HardwareEncoder,  // Default Auto
    pub audio_bitrate_kbps: u32,            // Default 160
    pub crf_quality: u32,                   // Default 19 (visually lossless)
}

impl Default for OptimizationSettings {
    fn default() -> Self {
        Self {
            preferred_audio_lang: "en".to_string(),
            preferred_subtitle_lang: "en".to_string(),
            target_size_mb: 1980,
            max_resolution: None,
            subtitle_font_size: 24,
            hardware_encoder: HardwareEncoder::Auto,
            audio_bitrate_kbps: 160,
            crf_quality: 19,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncodePlan {
    pub strategy: EncodeStrategy,
    pub target_size_mb: u64,
    pub target_video_bitrate_kbps: u64,
    pub audio_bitrate_kbps: u32,
    pub selected_audio_stream_index: usize,
    pub audio_needs_downmix: bool,
    pub subtitle_config: SubtitleBurnConfig,
    pub encoder: HardwareEncoder,
    pub output_width: Option<u32>,
    pub output_height: Option<u32>,
    pub is_visually_lossless: bool,
    pub estimated_output_size_bytes: u64,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueItemStatus {
    Queued,
    Probing,
    NeedsReview,
    Ready,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodeProgress {
    pub percent: f64,
    pub fps: f64,
    pub speed_multiplier: f64,
    pub current_time_secs: f64,
    pub total_duration_secs: f64,
    pub eta_seconds: f64,
    pub current_size_bytes: u64,
    pub target_size_bytes: u64,
    pub stage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub duration_seconds: f64,
    pub status: QueueItemStatus,
    pub probe: Option<MediaProbe>,
    pub plan: Option<EncodePlan>,
    pub progress: Option<TranscodeProgress>,
    pub error_message: Option<String>,
    pub selected_audio_track_index: Option<usize>,
    pub selected_subtitle_track_index: Option<usize>,
}

export interface VideoStream {
  index: number;
  codec_name: string;
  profile?: string;
  width: number;
  height: number;
  pix_fmt: string;
  r_frame_rate: string;
  bit_rate?: number;
  is_10bit: boolean;
  is_hdr: boolean;
}

export interface AudioTrack {
  stream_index: number;
  track_index: number;
  codec_name: string;
  language: string;
  title: string;
  channels: number;
  channel_layout: string;
  bit_rate?: number;
  is_default: boolean;
  is_commentary: boolean;
  is_hearing_impaired: boolean;
}

export interface SubtitleTrack {
  stream_index?: number;
  track_index: number;
  codec_name: string;
  language: string;
  title: string;
  is_default: boolean;
  is_forced: boolean;
  is_hearing_impaired: boolean;
  is_external: boolean;
  file_path?: string;
}

export interface MediaProbe {
  file_path: string;
  file_name: string;
  file_size_bytes: number;
  format_name: string;
  duration_seconds: number;
  bit_rate?: number;
  video_streams: VideoStream[];
  audio_tracks: AudioTrack[];
  subtitle_tracks: SubtitleTrack[];
  is_telegram_ready: boolean;
}

export type HardwareEncoder = "Auto" | "NvidiaNvenc" | "AmdAmf" | "IntelQsv" | "AppleVideoToolbox" | "CpuX264";

export interface SubtitleBurnConfig {
  enabled: boolean;
  track_index?: number;
  font_size_pt: number;
  custom_margin_v: number;
}

export interface OptimizationSettings {
  preferred_audio_lang: string;
  preferred_subtitle_lang: string;
  target_size_mb: number;
  max_resolution?: number;
  subtitle_font_size: number;
  hardware_encoder: HardwareEncoder;
  audio_bitrate_kbps: number;
  crf_quality: number;
}

export interface EncodePlan {
  strategy: "DirectRemux" | "TranscodeH264";
  target_size_mb: number;
  target_video_bitrate_kbps: number;
  audio_bitrate_kbps: number;
  selected_audio_stream_index: number;
  audio_needs_downmix: boolean;
  subtitle_config: SubtitleBurnConfig;
  encoder: HardwareEncoder;
  output_width?: number;
  output_height?: number;
  is_visually_lossless: boolean;
  estimated_output_size_bytes: number;
  reason: string;
}

export type QueueItemStatus =
  | "Queued"
  | "Probing"
  | "NeedsReview"
  | "Ready"
  | "Processing"
  | "Completed"
  | "Failed"
  | "Cancelled";

export interface TranscodeProgress {
  percent: number;
  fps: number;
  speed_multiplier: number;
  current_time_secs: number;
  total_duration_secs: number;
  eta_seconds: number;
  current_size_bytes: number;
  target_size_bytes: number;
  stage: string;
}

export interface QueueItem {
  id: string;
  input_path: string;
  output_path: string;
  file_name: string;
  file_size_bytes: number;
  duration_seconds: number;
  status: QueueItemStatus;
  probe?: MediaProbe;
  plan?: EncodePlan;
  progress?: TranscodeProgress;
  error_message?: string;
  selected_audio_track_index?: number;
  selected_subtitle_track_index?: number; // undefined means no subtitle
}

export interface HardwareCapabilities {
  has_nvidia_nvenc: boolean;
  has_amd_amf: boolean;
  has_intel_qsv: boolean;
  has_apple_videotoolbox: boolean;
  has_cpu_x264: boolean;
  recommended_encoder: HardwareEncoder;
}

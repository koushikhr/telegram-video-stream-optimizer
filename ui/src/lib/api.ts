import type {
  MediaProbe,
  HardwareCapabilities,
  OptimizationSettings,
  EncodePlan,
  QueueItem,
} from "../types";

// Helper checking if running inside Tauri
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function probeVideo(filePath: string): Promise<MediaProbe> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<MediaProbe>("probe_video", { filePath });
  }
  // Fallback demo mock
  return {
    file_path: filePath,
    file_name: filePath.split(/[/\\]/).pop() || "video.mkv",
    file_size_bytes: 2_147_483_648,
    format_name: "matroska,webm",
    duration_seconds: 2820.0,
    bit_rate: 6_090_000,
    is_telegram_ready: false,
    video_streams: [
      {
        index: 0,
        codec_name: "hevc",
        width: 1920,
        height: 1080,
        pix_fmt: "yuv420p10le",
        r_frame_rate: "24/1",
        is_10bit: true,
        is_hdr: false,
      },
    ],
    audio_tracks: [
      {
        stream_index: 1,
        track_index: 0,
        codec_name: "eac3",
        language: "en",
        title: "English Dialogue 5.1",
        channels: 6,
        channel_layout: "5.1(side)",
        is_default: true,
        is_commentary: false,
        is_hearing_impaired: false,
      },
    ],
    subtitle_tracks: [
      {
        stream_index: 2,
        track_index: 0,
        codec_name: "subrip",
        language: "en",
        title: "English Dialogue (Full)",
        is_default: true,
        is_forced: false,
        is_hearing_impaired: false,
        is_external: false,
      },
    ],
  };
}

export async function detectHardware(): Promise<HardwareCapabilities> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<HardwareCapabilities>("detect_hardware");
  }
  return {
    has_nvidia_nvenc: true,
    has_amd_amf: true,
    has_intel_qsv: false,
    has_apple_videotoolbox: false,
    has_cpu_x264: true,
    recommended_encoder: "NvidiaNvenc",
  };
}

export async function generatePreview(
  filePath: string,
  timestamp: number,
  subTrackIndex: number | undefined,
  fontSize: number,
  borderStyle: number = 1
): Promise<string> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<string>("generate_preview", {
      filePath,
      timestamp,
      subTrackIndex: subTrackIndex ?? null,
      fontSize,
      borderStyle,
    });
  }
  // Return a mock canvas placeholder if running outside Tauri
  return createMockPreviewDataUrl(fontSize);
}

export async function createPlan(
  probe: MediaProbe,
  settings: OptimizationSettings,
  subTrackIndex: number | undefined
): Promise<EncodePlan> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<EncodePlan>("create_plan", {
      probe,
      settings,
      subTrackIndex: subTrackIndex ?? null,
    });
  }
  return {
    strategy: "TranscodeH264",
    target_size_mb: settings.target_size_mb,
    target_video_bitrate_kbps: 5600,
    audio_bitrate_kbps: 160,
    selected_audio_stream_index: 1,
    audio_needs_downmix: true,
    subtitle_config: {
      enabled: subTrackIndex !== undefined,
      track_index: subTrackIndex,
      font_size_pt: settings.subtitle_font_size,
      custom_margin_v: 28,
    },
    encoder: settings.hardware_encoder,
    is_visually_lossless: true,
    reason: "Compressing to fit Telegram 1,980 MB cap with burned dialogue subtitles",
  };
}

export async function openVideoFiles(): Promise<string[]> {
  if (isTauri()) {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const res = await open({
      multiple: true,
      filters: [
        {
          name: "Video Files",
          extensions: ["mkv", "mp4", "avi", "mov", "webm", "ts", "m2ts", "flv", "wmv"],
        },
      ],
    });
    if (!res) return [];
    return Array.isArray(res) ? res : [res];
  }
  return [];
}

export async function openVideoFolder(): Promise<string[]> {
  if (isTauri()) {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const dir = await open({
      directory: true,
      multiple: false,
    });
    if (!dir || typeof dir !== "string") return [];
    const { invoke } = await import("@tauri-apps/api/core");
    return await invoke<string[]>("scan_folder_for_videos", { folderPath: dir });
  }
  return [];
}

export async function triggerPcSleep(): Promise<void> {
  if (isTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("trigger_pc_sleep");
  }
}

function createMockPreviewDataUrl(fontSize: number): string {
  const canvas = document.createElement("canvas");
  canvas.width = 960;
  canvas.height = 540;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.fillStyle = "#0f172a";
    ctx.fillRect(0, 0, 960, 540);
    ctx.fillStyle = "#38bdf8";
    ctx.font = "bold 24px sans-serif";
    ctx.textAlign = "center";
    ctx.fillText("Preview Frame — Live Subtitle Rendering", 480, 240);

    ctx.fillStyle = "#ffffff";
    ctx.font = `bold ${fontSize}px sans-serif`;
    ctx.strokeStyle = "#000000";
    ctx.lineWidth = 4;
    ctx.strokeText("This is how your dialogue subtitles will look on Telegram", 480, 480);
    ctx.fillText("This is how your dialogue subtitles will look on Telegram", 480, 480);
  }
  return canvas.toDataURL("image/jpeg");
}

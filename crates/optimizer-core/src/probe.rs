use crate::models::{AudioTrack, MediaProbe, SubtitleTrack, VideoStream};
use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;
use tokio::process::Command;

pub async fn probe_file<P: AsRef<Path>>(path: P) -> Result<MediaProbe> {
    let path = path.as_ref().to_path_buf();
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let output = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(&path)
        .output()
        .await
        .with_context(|| format!("Failed to execute ffprobe on {}", path.display()))?;

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("ffprobe failed on {}: {}", path.display(), err_msg);
    }

    let json: Value = serde_json::from_slice(&output.stdout)
        .with_context(|| "Failed to parse ffprobe JSON output")?;

    let format_val = json.get("format").context("Missing format section in ffprobe output")?;
    let format_name = format_val
        .get("format_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let duration_seconds: f64 = format_val
        .get("duration")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);

    let file_size_bytes: u64 = format_val
        .get("size")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)
        });

    let bit_rate: Option<u64> = format_val
        .get("bit_rate")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok());

    let mut video_streams = Vec::new();
    let mut audio_tracks = Vec::new();
    let mut subtitle_tracks = Vec::new();

    if let Some(streams) = json.get("streams").and_then(|v| v.as_array()) {
        let mut audio_counter = 0;
        let mut subtitle_counter = 0;

        for stream in streams {
            let codec_type = stream.get("codec_type").and_then(|v| v.as_str()).unwrap_or("");
            let index = stream.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            let codec_name = stream
                .get("codec_name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            let tags = stream.get("tags");
            let raw_lang = tags
                .and_then(|t| t.get("language").or_else(|| t.get("lang")))
                .and_then(|v| v.as_str())
                .unwrap_or("und");
            let language = crate::heuristics::normalize_language_code(raw_lang);

            let title = tags
                .and_then(|t| t.get("title").or_else(|| t.get("description")))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let disposition = stream.get("disposition");
            let is_default = disposition
                .and_then(|d| d.get("default"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                == 1;
            let is_forced = disposition
                .and_then(|d| d.get("forced"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                == 1;
            let is_hearing_impaired = disposition
                .and_then(|d| d.get("hearing_impaired"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                == 1
                || title.to_lowercase().contains("sdh")
                || title.to_lowercase().contains("cc");
            let is_commentary = disposition
                .and_then(|d| d.get("commentary"))
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                == 1
                || title.to_lowercase().contains("commentary")
                || title.to_lowercase().contains("director");

            match codec_type {
                "video" => {
                    let width = stream.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    let height = stream.get("height").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                    let pix_fmt = stream
                        .get("pix_fmt")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let profile = stream
                        .get("profile")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let r_frame_rate = stream
                        .get("r_frame_rate")
                        .and_then(|v| v.as_str())
                        .unwrap_or("30/1")
                        .to_string();
                    let bit_rate = stream
                        .get("bit_rate")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok());

                    let is_10bit = pix_fmt.contains("10")
                        || pix_fmt.contains("12")
                        || stream.get("bits_per_raw_sample").and_then(|v| v.as_str()).map(|s| s == "10").unwrap_or(false);

                    let color_primaries = stream.get("color_primaries").and_then(|v| v.as_str()).unwrap_or("");
                    let color_transfer = stream.get("color_transfer").and_then(|v| v.as_str()).unwrap_or("");
                    let is_hdr = color_primaries.contains("2020")
                        || color_transfer.contains("smpte2084")
                        || color_transfer.contains("arib-std-b67");

                    video_streams.push(VideoStream {
                        index,
                        codec_name,
                        profile,
                        width,
                        height,
                        pix_fmt,
                        r_frame_rate,
                        bit_rate,
                        is_10bit,
                        is_hdr,
                    });
                }
                "audio" => {
                    let channels = stream.get("channels").and_then(|v| v.as_u64()).unwrap_or(2) as u32;
                    let channel_layout = stream
                        .get("channel_layout")
                        .and_then(|v| v.as_str())
                        .unwrap_or("stereo")
                        .to_string();
                    let bit_rate = stream
                        .get("bit_rate")
                        .and_then(|v| v.as_str())
                        .and_then(|s| s.parse().ok());

                    audio_tracks.push(AudioTrack {
                        stream_index: index,
                        track_index: audio_counter,
                        codec_name,
                        language,
                        title,
                        channels,
                        channel_layout,
                        bit_rate,
                        is_default,
                        is_commentary,
                        is_hearing_impaired,
                    });
                    audio_counter += 1;
                }
                "subtitle" => {
                    subtitle_tracks.push(SubtitleTrack {
                        stream_index: Some(index),
                        track_index: subtitle_counter,
                        codec_name,
                        language,
                        title,
                        is_default,
                        is_forced,
                        is_hearing_impaired,
                        is_external: false,
                        file_path: None,
                    });
                    subtitle_counter += 1;
                }
                _ => {}
            }
        }
    }

    // Discover external sidecar subtitles
    discover_external_subtitles(&path, &mut subtitle_tracks);

    // Telegram compatibility check:
    let is_mp4 = format_name.contains("mp4") || format_name.contains("mov");
    let has_compatible_video = video_streams.first().map(|v| {
        (v.codec_name == "h264" || v.codec_name == "avc1")
            && v.pix_fmt == "yuv420p"
            && !v.is_10bit
    }).unwrap_or(false);

    let has_compatible_audio = audio_tracks.first().map(|a| {
        (a.codec_name == "aac" || a.codec_name == "mp3") && a.channels <= 2
    }).unwrap_or(true);

    let is_under_size = file_size_bytes <= 1_980 * 1024 * 1024;
    let is_telegram_ready = is_mp4 && has_compatible_video && has_compatible_audio && is_under_size;

    Ok(MediaProbe {
        file_path: path,
        file_name,
        file_size_bytes,
        format_name,
        duration_seconds,
        bit_rate,
        video_streams,
        audio_tracks,
        subtitle_tracks,
        is_telegram_ready,
    })
}

fn discover_external_subtitles(video_path: &Path, subtitles: &mut Vec<SubtitleTrack>) {
    let parent_dir = match video_path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => Path::new("."),
    };

    let stem = match video_path.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s.to_lowercase(),
        None => return,
    };

    let Ok(entries) = std::fs::read_dir(parent_dir) else {
        return;
    };

    let mut ext_counter = subtitles.len();

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        if !matches!(ext.as_str(), "srt" | "ass" | "ssa" | "vtt") {
            continue;
        }

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Check if subtitle filename starts with or matches the video stem
        if file_stem.starts_with(&stem) || file_stem == stem {
            // Extract language suffix if present, e.g. "movie.en.srt" -> "en"
            let remaining = file_stem.strip_prefix(&stem).unwrap_or("");
            let raw_lang = remaining
                .trim_start_matches(|c| c == '.' || c == '_' || c == '-')
                .split('.')
                .next()
                .filter(|s| !s.is_empty())
                .unwrap_or("und");
            let lang = crate::heuristics::normalize_language_code(raw_lang);

            let title = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("External Subtitle")
                .to_string();

            subtitles.push(SubtitleTrack {
                stream_index: None,
                track_index: ext_counter,
                codec_name: ext,
                language: lang,
                title,
                is_default: false,
                is_forced: false,
                is_hearing_impaired: false,
                is_external: true,
                file_path: Some(path),
            });
            ext_counter += 1;
        }
    }
}

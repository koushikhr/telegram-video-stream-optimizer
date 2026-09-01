use crate::models::SubtitleTrack;
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::path::Path;
use tracing::info;

pub fn escape_ffmpeg_filter_path(path: &Path) -> String {
    let s = path.to_string_lossy().to_string();
    // Replace backslashes with forward slashes
    let with_forward_slashes = s.replace('\\', "/");
    // Escape colons (e.g. C: -> C\:)
    with_forward_slashes.replace(':', "\\:")
}

pub async fn generate_preview_frame(
    video_path: &Path,
    timestamp_secs: f64,
    subtitle: Option<&SubtitleTrack>,
    font_size: u32,
    border_style: Option<u32>,
) -> Result<String> {
    let border_style = border_style.unwrap_or(1);
    let mut cmd = crate::create_silent_command("ffmpeg");

    // Seek before input for fast lookup, with -copyts to preserve original subtitle timestamps
    let seek_time = format!("{:.2}", timestamp_secs.max(0.0));
    cmd.args(["-ss", &seek_time, "-copyts"]);
    cmd.arg("-i").arg(video_path);

    let font_size = font_size.clamp(8, 48);
    let outline = (font_size as f64 * 0.055).clamp(0.8, 1.8);
    let shadow = (outline * 0.4).clamp(0.3, 0.9);

    // Build filter complex
    let filter = if let Some(sub) = subtitle {
        let style = if border_style == 3 {
            format!(
                "force_style='FontSize={}\\,PrimaryColour=&H00FFFFFF\\,OutlineColour=&H00000000\\,BorderStyle=3\\,Outline=2.0\\,Shadow=0\\,MarginV=28'",
                font_size
            )
        } else {
            format!(
                "force_style='FontSize={}\\,PrimaryColour=&H00FFFFFF\\,OutlineColour=&H00000000\\,BorderStyle=1\\,Outline={:.2}\\,Shadow={:.2}\\,MarginV=28'",
                font_size, outline, shadow
            )
        };

        if let Some(ext_path) = &sub.file_path {
            let escaped = escape_ffmpeg_filter_path(ext_path);
            format!("[0:v]subtitles='{}':{}[v]", escaped, style)
        } else if let Some(_stream_idx) = sub.stream_index {
            let escaped = escape_ffmpeg_filter_path(video_path);
            format!("[0:v]subtitles='{}':si={}:{},format=yuv420p[v]", escaped, sub.track_index, style)
        } else {
            "[0:v]format=yuv420p[v]".to_string()
        }
    } else {
        "[0:v]format=yuv420p[v]".to_string()
    };

    cmd.args([
        "-filter_complex",
        &filter,
        "-map",
        "[v]",
        "-vframes",
        "1",
        "-q:v",
        "2",
        "-f",
        "image2",
        "pipe:1",
    ]);

    info!(
        "Generating preview frame for {} at {}s with font size {}",
        video_path.display(),
        seek_time,
        font_size
    );

    let output = cmd
        .output()
        .await
        .with_context(|| "Failed to execute ffmpeg for preview frame")?;

    if !output.status.success() || output.stdout.is_empty() {
        let err = String::from_utf8_lossy(&output.stderr);
        // Fallback: generate raw frame without subtitle filter in case filter syntax failed
        info!("Subtitle filter preview failed, falling back to clean frame: {}", err);
        let fallback_output = crate::create_silent_command("ffmpeg")
            .args([
                "-ss",
                &seek_time,
                "-i",
            ])
            .arg(video_path)
            .args([
                "-vframes",
                "1",
                "-q:v",
                "2",
                "-f",
                "image2",
                "pipe:1",
            ])
            .output()
            .await?;

        if !fallback_output.status.success() || fallback_output.stdout.is_empty() {
            anyhow::bail!("Failed to generate frame preview: {}", String::from_utf8_lossy(&fallback_output.stderr));
        }

        let b64 = BASE64.encode(&fallback_output.stdout);
        return Ok(format!("data:image/jpeg;base64,{}", b64));
    }

    let b64 = BASE64.encode(&output.stdout);
    Ok(format!("data:image/jpeg;base64,{}", b64))
}

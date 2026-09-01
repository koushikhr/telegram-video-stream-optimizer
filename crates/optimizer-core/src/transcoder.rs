use crate::faststart::is_mp4_faststart;
use crate::models::{
    EncodePlan, EncodeStrategy, HardwareEncoder, MediaProbe, SubtitleTrack, TranscodeProgress,
};
use crate::preview::escape_ffmpeg_filter_path;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::broadcast;
use tracing::{info, warn};

pub struct TranscodeJob {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub probe: MediaProbe,
    pub plan: EncodePlan,
    pub selected_subtitle: Option<SubtitleTrack>,
    pub cancel_flag: Arc<AtomicBool>,
}

pub async fn run_transcode(
    job: TranscodeJob,
    progress_tx: Option<broadcast::Sender<TranscodeProgress>>,
) -> Result<PathBuf> {
    let temp_output = job
        .output_path
        .with_extension("tg_temp.mp4");

    // Clean any prior temp file
    if temp_output.exists() {
        let _ = std::fs::remove_file(&temp_output);
    }

    let result = execute_transcode_inner(&job, &temp_output, progress_tx).await;

    match result {
        Ok(()) => {
            // Verify and atomically rename
            if temp_output.exists() {
                if let Err(e) = std::fs::rename(&temp_output, &job.output_path) {
                    let _ = std::fs::remove_file(&temp_output);
                    anyhow::bail!("Failed to rename temp file to final destination: {}", e);
                }
                info!("Transcode completed successfully: {}", job.output_path.display());
                Ok(job.output_path)
            } else {
                anyhow::bail!("Transcode finished but temporary file was not found");
            }
        }
        Err(e) => {
            // Cleanup incomplete file on error or cancellation
            if temp_output.exists() {
                warn!("Cleaning up incomplete temporary file {}", temp_output.display());
                let _ = std::fs::remove_file(&temp_output);
            }
            Err(e)
        }
    }
}

async fn execute_transcode_inner(
    job: &TranscodeJob,
    temp_output: &Path,
    progress_tx: Option<broadcast::Sender<TranscodeProgress>>,
) -> Result<()> {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-y", "-hide_banner"]);

    // Input file
    cmd.arg("-i").arg(&job.input_path);

    let total_duration = job.probe.duration_seconds.max(1.0);
    let target_size_bytes = job.plan.target_size_mb * 1024 * 1024;

    match job.plan.strategy {
        EncodeStrategy::DirectRemux => {
            info!("Direct Remuxing {}", job.input_path.display());
            cmd.args([
                "-c",
                "copy",
                "-movflags",
                "+faststart",
                "-progress",
                "pipe:1",
            ]);
            cmd.arg(temp_output);
        }
        EncodeStrategy::TranscodeH264 => {
            info!(
                "Transcoding {} (Video Bitrate: {} kbps, Audio: {} kbps, Encoder: {:?})",
                job.input_path.display(),
                job.plan.target_video_bitrate_kbps,
                job.plan.audio_bitrate_kbps,
                job.plan.encoder
            );

            // Select video and audio streams
            // Audio mapping: map selected audio stream
            let audio_map = format!("0:{}", job.plan.selected_audio_stream_index);

            // Subtitle filter
            let mut filter_chain = Vec::new();

            if job.plan.subtitle_config.enabled {
                if let Some(sub) = &job.selected_subtitle {
                    let font_size = job.plan.subtitle_config.font_size_pt.clamp(14, 48);
                    let margin_v = job.plan.subtitle_config.custom_margin_v.clamp(10, 80);
                    let border_style = job.plan.subtitle_config.border_style;
                    let style = if border_style == 3 {
                        format!(
                            "force_style='FontSize={}\\,PrimaryColour=&H00FFFFFF\\,OutlineColour=&H00000000\\,BorderStyle=3\\,Outline=2.5\\,Shadow=0\\,MarginV={}'",
                            font_size, margin_v
                        )
                    } else {
                        format!(
                            "force_style='FontSize={}\\,PrimaryColour=&H00FFFFFF\\,OutlineColour=&H00000000\\,BorderStyle=1\\,Outline=2.4\\,Shadow=1.2\\,MarginV={}'",
                            font_size, margin_v
                        )
                    };

                    if let Some(ext_path) = &sub.file_path {
                        let escaped = escape_ffmpeg_filter_path(ext_path);
                        filter_chain.push(format!("subtitles='{}':{}", escaped, style));
                    } else if let Some(_idx) = sub.stream_index {
                        let escaped = escape_ffmpeg_filter_path(&job.input_path);
                        filter_chain.push(format!("subtitles='{}':si={}:{}", escaped, sub.track_index, style));
                    }
                }
            }

            // Resolution scale filter if requested
            if let (Some(w), Some(h)) = (job.plan.output_width, job.plan.output_height) {
                filter_chain.push(format!("scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2", w, h, w, h));
            }

            // Ensure 8-bit YUV420p for 100% Telegram compatibility
            filter_chain.push("format=yuv420p".to_string());

            let vf_arg = filter_chain.join(",");
            cmd.args(["-vf", &vf_arg]);

            // Video Encoder configuration
            let encoder = match job.plan.encoder {
                HardwareEncoder::Auto => HardwareEncoder::CpuX264,
                other => other,
            };

            let v_bitrate = job.plan.target_video_bitrate_kbps;
            let max_rate = (v_bitrate as f64 * 1.25) as u64;
            let buf_size = v_bitrate * 2;

            match encoder {
                HardwareEncoder::NvidiaNvenc => {
                    cmd.args([
                        "-c:v",
                        "h264_nvenc",
                        "-preset",
                        "p6", // High quality
                        "-tune",
                        "hq",
                        "-b:v",
                        &format!("{}k", v_bitrate),
                        "-maxrate",
                        &format!("{}k", max_rate),
                        "-bufsize",
                        &format!("{}k", buf_size),
                    ]);
                }
                HardwareEncoder::AmdAmf => {
                    cmd.args([
                        "-c:v",
                        "h264_amf",
                        "-quality",
                        "quality",
                        "-b:v",
                        &format!("{}k", v_bitrate),
                        "-maxrate",
                        &format!("{}k", max_rate),
                        "-bufsize",
                        &format!("{}k", buf_size),
                    ]);
                }
                HardwareEncoder::IntelQsv => {
                    cmd.args([
                        "-c:v",
                        "h264_qsv",
                        "-preset",
                        "medium",
                        "-b:v",
                        &format!("{}k", v_bitrate),
                        "-maxrate",
                        &format!("{}k", max_rate),
                        "-bufsize",
                        &format!("{}k", buf_size),
                    ]);
                }
                HardwareEncoder::AppleVideoToolbox => {
                    cmd.args([
                        "-c:v",
                        "h264_videotoolbox",
                        "-b:v",
                        &format!("{}k", v_bitrate),
                    ]);
                }
                HardwareEncoder::CpuX264 | HardwareEncoder::Auto => {
                    cmd.args([
                        "-c:v",
                        "libx264",
                        "-preset",
                        "faster",
                        "-b:v",
                        &format!("{}k", v_bitrate),
                        "-maxrate",
                        &format!("{}k", max_rate),
                        "-bufsize",
                        &format!("{}k", buf_size),
                    ]);
                }
            }

            // Audio configuration: stereo AAC with dialogue normalization
            cmd.args(["-map", "0:v:0", "-map", &audio_map]);
            cmd.args([
                "-c:a",
                "aac",
                "-b:a",
                &format!("{}k", job.plan.audio_bitrate_kbps),
            ]);

            if job.plan.audio_needs_downmix {
                // Downmix 5.1/7.1 to stereo with clear dialogue center-channel gain
                cmd.args(["-ac", "2", "-af", "pan=stereo|FL=0.5*FC+0.707*FL+0.707*BL+0.5*LFE|FR=0.5*FC+0.707*FR+0.707*BR+0.5*LFE"]);
            } else {
                cmd.args(["-ac", "2"]);
            }

            // Move moov atom to the front for instant streaming in Telegram!
            cmd.args([
                "-movflags",
                "+faststart",
                "-progress",
                "pipe:1",
            ]);

            cmd.arg(temp_output);
        }
    }

    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .with_context(|| "Failed to spawn ffmpeg process")?;

    let stdout = child.stdout.take().context("Failed to open ffmpeg stdout")?;
    let mut reader = BufReader::new(stdout).lines();

    let stderr = child.stderr.take();
    let stderr_lines = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let stderr_lines_clone = Arc::clone(&stderr_lines);
    if let Some(err_stream) = stderr {
        tokio::spawn(async move {
            let mut err_reader = BufReader::new(err_stream).lines();
            while let Ok(Some(err_line)) = err_reader.next_line().await {
                let mut lines = stderr_lines_clone.lock().await;
                if lines.len() > 20 {
                    lines.remove(0);
                }
                lines.push(err_line);
            }
        });
    }

    let mut current_progress = TranscodeProgress {
        percent: 0.0,
        fps: 0.0,
        speed_multiplier: 1.0,
        current_time_secs: 0.0,
        total_duration_secs: total_duration,
        eta_seconds: 0.0,
        current_size_bytes: 0,
        target_size_bytes,
        stage: match job.plan.strategy {
            EncodeStrategy::DirectRemux => "FastStart Remuxing".to_string(),
            EncodeStrategy::TranscodeH264 => "Transcoding H.264".to_string(),
        },
    };

    while let Ok(Some(line)) = reader.next_line().await {
        // Check cancellation flag
        if job.cancel_flag.load(Ordering::Relaxed) {
            info!("Cancellation signal detected. Terminating ffmpeg child process.");
            let _ = child.kill().await;
            anyhow::bail!("Transcode cancelled by user");
        }

        let trimmed = line.trim();
        if let Some(val) = trimmed.strip_prefix("out_time_ms=") {
            if let Ok(us) = val.parse::<u64>() {
                let secs = (us as f64) / 1_000_000.0;
                current_progress.current_time_secs = secs;
                let pct = (secs / total_duration * 100.0).clamp(0.0, 99.9);
                current_progress.percent = pct;

                if current_progress.speed_multiplier > 0.05 {
                    let remaining_secs = (total_duration - secs).max(0.0);
                    current_progress.eta_seconds = remaining_secs / current_progress.speed_multiplier;
                }

                if let Some(tx) = &progress_tx {
                    let _ = tx.send(current_progress.clone());
                }
            }
        } else if let Some(val) = trimmed.strip_prefix("total_size=") {
            if let Ok(sz) = val.parse::<u64>() {
                current_progress.current_size_bytes = sz;
            }
        } else if let Some(val) = trimmed.strip_prefix("speed=") {
            let s = val.trim_end_matches('x');
            if let Ok(spd) = s.parse::<f64>() {
                current_progress.speed_multiplier = spd;
            }
        } else if let Some(val) = trimmed.strip_prefix("fps=") {
            if let Ok(fps) = val.parse::<f64>() {
                current_progress.fps = fps;
            }
        }
    }

    let status = child.wait().await?;
    if !status.success() {
        let err_tail = stderr_lines.lock().await.join("\n");
        anyhow::bail!("FFmpeg exited with error status: {}. Details:\n{}", status, err_tail);
    }

    // Final check for faststart moov atom
    if let Ok(is_fast) = is_mp4_faststart(temp_output) {
        info!("FastStart verification on output: {}", is_fast);
    }

    // Send 100% completion
    current_progress.percent = 100.0;
    current_progress.eta_seconds = 0.0;
    if let Some(tx) = &progress_tx {
        let _ = tx.send(current_progress);
    }

    Ok(())
}

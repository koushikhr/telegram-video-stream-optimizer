use anyhow::Result;
use clap::{Parser, Subcommand};
use optimizer_core::{
    create_encode_plan, detect_hardware_encoders, generate_preview_frame, probe_file,
    run_transcode, select_best_subtitle_track, OptimizationSettings, TranscodeJob,
};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "optimizer-cli")]
#[command(about = "Telegram Video Stream Optimizer CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe video and print stream details
    Probe {
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },
    /// Detect available hardware acceleration encoders
    DetectHw,
    /// Generate live preview frame with subtitle burn-in
    Preview {
        #[arg(value_name = "FILE")]
        path: PathBuf,
        #[arg(short, long, default_value_t = 30.0)]
        timestamp: f64,
        #[arg(short, long, default_value_t = 24)]
        font_size: u32,
    },
    /// Optimize video for instant Telegram streaming
    Optimize {
        #[arg(value_name = "INPUT")]
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = 1980)]
        target_mb: u64,
        #[arg(long, default_value_t = 24)]
        font_size: u32,
        #[arg(long, default_value = "en")]
        audio_lang: String,
        #[arg(long, default_value = "en")]
        sub_lang: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Probe { path } => {
            println!("Probing file: {}", path.display());
            let probe = probe_file(&path).await?;
            println!("\n=== Media Probe Summary ===");
            println!("File Name: {}", probe.file_name);
            println!("Format: {}", probe.format_name);
            println!("Size: {:.2} MB", probe.file_size_bytes as f64 / (1024.0 * 1024.0));
            println!("Duration: {:.1} seconds ({:.2} minutes)", probe.duration_seconds, probe.duration_seconds / 60.0);
            println!("Telegram Ready (Instant Streamable): {}", probe.is_telegram_ready);

            println!("\n--- Video Streams ({}) ---", probe.video_streams.len());
            for v in &probe.video_streams {
                println!(
                    "  [Stream #{}] Codec: {}, Res: {}x{}, PixFmt: {}, 10-bit: {}, HDR: {}",
                    v.index, v.codec_name, v.width, v.height, v.pix_fmt, v.is_10bit, v.is_hdr
                );
            }

            println!("\n--- Audio Tracks ({}) ---", probe.audio_tracks.len());
            for a in &probe.audio_tracks {
                println!(
                    "  [Track #{}] Lang: {}, Codec: {}, Channels: {} ({}), Title: '{}', Default: {}, Commentary: {}",
                    a.track_index, a.language, a.codec_name, a.channels, a.channel_layout, a.title, a.is_default, a.is_commentary
                );
            }

            println!("\n--- Subtitle Tracks ({}) ---", probe.subtitle_tracks.len());
            for s in &probe.subtitle_tracks {
                println!(
                    "  [Track #{}] Lang: {}, Codec: {}, Title: '{}', External: {}, Default: {}, Forced: {}, SDH: {}",
                    s.track_index, s.language, s.codec_name, s.title, s.is_external, s.is_default, s.is_forced, s.is_hearing_impaired
                );
            }
        }
        Commands::DetectHw => {
            let hw = detect_hardware_encoders().await;
            println!("\n=== Hardware Acceleration Probing ===");
            println!("NVIDIA NVENC (h264_nvenc): {}", if hw.has_nvidia_nvenc { "Available" } else { "Not found" });
            println!("AMD AMF (h264_amf): {}", if hw.has_amd_amf { "Available" } else { "Not found" });
            println!("Intel QuickSync (h264_qsv): {}", if hw.has_intel_qsv { "Available" } else { "Not found" });
            println!("Apple VideoToolbox: {}", if hw.has_apple_videotoolbox { "Available" } else { "Not found" });
            println!("CPU libx264: {}", if hw.has_cpu_x264 { "Available" } else { "Not found" });
            println!("\nRecommended Default: {:?}", hw.recommended_encoder);
        }
        Commands::Preview { path, timestamp, font_size } => {
            println!("Generating preview frame for {} at {:.1}s with font size {}...", path.display(), timestamp, font_size);
            let probe = probe_file(&path).await?;
            let sub_res = select_best_subtitle_track(&probe.subtitle_tracks, "en", None);
            let data_url = generate_preview_frame(&path, timestamp, sub_res.selected_track, font_size, None).await?;
            println!("Preview generated successfully! (Data URL length: {} chars)", data_url.len());
            println!("Preview prefix: {}...", &data_url[..50.min(data_url.len())]);
        }
        Commands::Optimize { input, output, target_mb, font_size, audio_lang, sub_lang } => {
            println!("Starting optimization for {}", input.display());
            let probe = probe_file(&input).await?;
            let hw = detect_hardware_encoders().await;

            let settings = OptimizationSettings {
                preferred_audio_lang: audio_lang,
                preferred_subtitle_lang: sub_lang,
                target_size_mb: target_mb,
                max_resolution: None,
                subtitle_font_size: font_size,
                hardware_encoder: hw.recommended_encoder,
                audio_bitrate_kbps: 160,
                crf_quality: 19,
            };

            let sub_res = select_best_subtitle_track(&probe.subtitle_tracks, &settings.preferred_subtitle_lang, None);
            let plan = create_encode_plan(&probe, &settings, sub_res.selected_track);

            println!("\n=== Optimization Plan ===");
            println!("Strategy: {:?}", plan.strategy);
            println!("Target Video Bitrate: {} kbps", plan.target_video_bitrate_kbps);
            println!("Audio Bitrate: {} kbps", plan.audio_bitrate_kbps);
            println!("Subtitle Burn: {}", plan.subtitle_config.enabled);
            println!("Hardware Encoder: {:?}", plan.encoder);
            println!("Reason: {}", plan.reason);

            let out_path = output.unwrap_or_else(|| {
                let stem = input.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
                input.with_file_name(format!("[TG] {}.mp4", stem))
            });

            let selected_subtitle = sub_res.selected_track.cloned();

            let job = TranscodeJob {
                input_path: input,
                output_path: out_path,
                probe,
                plan,
                selected_subtitle,
                cancel_flag: Arc::new(AtomicBool::new(false)),
            };

            let final_path = run_transcode(job, None).await?;
            println!("\nOptimization complete! Saved to: {}", final_path.display());
        }
    }

    Ok(())
}

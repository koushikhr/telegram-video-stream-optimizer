use optimizer_core::{
    create_encode_plan, detect_hardware_encoders, generate_preview_frame,
    probe_file, put_system_to_sleep, run_transcode, EncodePlan, HardwareCapabilities, MediaProbe,
    OptimizationSettings, QueueItem, SubtitleTrack, TranscodeJob, TranscodeProgress,
};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::broadcast;
use walkdir::WalkDir;

pub struct AppState {
    pub cancel_flag: Arc<AtomicBool>,
}

#[tauri::command]
async fn probe_video(file_path: String) -> Result<MediaProbe, String> {
    probe_file(&file_path)
        .await
        .map_err(|e| format!("Probe failed: {:#}", e))
}

#[tauri::command]
async fn detect_hardware() -> HardwareCapabilities {
    detect_hardware_encoders().await
}

#[tauri::command]
async fn generate_preview(
    file_path: String,
    timestamp: f64,
    sub_track_index: Option<usize>,
    font_size: u32,
) -> Result<String, String> {
    let p = Path::new(&file_path);
    let probe = probe_file(p)
        .await
        .map_err(|e| format!("Failed to probe video: {:#}", e))?;

    let sub_track = sub_track_index.and_then(|idx| {
        probe.subtitle_tracks.iter().find(|s| s.track_index == idx)
    });

    generate_preview_frame(p, timestamp, sub_track, font_size)
        .await
        .map_err(|e| format!("Failed to generate frame preview: {:#}", e))
}

#[tauri::command]
fn create_plan(
    probe: MediaProbe,
    settings: OptimizationSettings,
    sub_track_index: Option<usize>,
) -> EncodePlan {
    let sub_track = sub_track_index.and_then(|idx| {
        probe.subtitle_tracks.iter().find(|s| s.track_index == idx)
    });
    create_encode_plan(&probe, &settings, sub_track)
}

#[tauri::command]
fn scan_folder_for_videos(folder_path: String) -> Result<Vec<String>, String> {
    let mut files = Vec::new();
    let video_extensions = ["mkv", "mp4", "avi", "mov", "webm", "ts", "m2ts", "flv", "wmv"];

    for entry in WalkDir::new(&folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()).map(|e| e.to_lowercase()) {
                if video_extensions.contains(&ext.as_str()) {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

#[tauri::command]
async fn start_transcode_item(
    app: AppHandle,
    state: State<'_, AppState>,
    item: QueueItem,
    font_sz: u32,
) -> Result<String, String> {
    let probe = match item.probe {
        Some(p) => p,
        None => probe_file(&item.input_path)
            .await
            .map_err(|e| format!("Probe failed: {:#}", e))?,
    };

    let mut plan = item.plan.unwrap_or_else(|| {
        let settings = OptimizationSettings::default();
        create_encode_plan(&probe, &settings, None)
    });

    plan.subtitle_config.font_size_pt = font_sz;

    let selected_sub: Option<SubtitleTrack> = item.selected_subtitle_track_index.and_then(|idx| {
        probe.subtitle_tracks.iter().find(|s| s.track_index == idx).cloned()
    });

    state.cancel_flag.store(false, Ordering::Relaxed);

    let (progress_tx, mut progress_rx) = broadcast::channel::<TranscodeProgress>(32);

    let app_clone = app.clone();
    let item_id = item.id.clone();
    tokio::spawn(async move {
        while let Ok(prog) = progress_rx.recv().await {
            let _ = app_clone.emit(&format!("progress_{}", item_id), &prog);
        }
    });

    let job = TranscodeJob {
        input_path: item.input_path,
        output_path: item.output_path,
        probe,
        plan,
        selected_subtitle: selected_sub,
        cancel_flag: Arc::clone(&state.cancel_flag),
    };

    let result = run_transcode(job, Some(progress_tx)).await;

    match result {
        Ok(out) => Ok(out.to_string_lossy().to_string()),
        Err(e) => Err(format!("Transcode failed: {:#}", e)),
    }
}

#[tauri::command]
fn cancel_current_transcode(state: State<'_, AppState>) -> Result<(), String> {
    state.cancel_flag.store(true, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
fn trigger_pc_sleep() -> Result<(), String> {
    put_system_to_sleep().map_err(|e| format!("Failed to sleep PC: {:#}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cancel_flag = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState { cancel_flag })
        .invoke_handler(tauri::generate_handler![
            probe_video,
            detect_hardware,
            generate_preview,
            create_plan,
            scan_folder_for_videos,
            start_transcode_item,
            cancel_current_transcode,
            trigger_pc_sleep
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

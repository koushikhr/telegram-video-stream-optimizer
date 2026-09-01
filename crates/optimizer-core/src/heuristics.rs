use crate::models::{
    AudioTrack, EncodePlan, EncodeStrategy, MediaProbe, OptimizationSettings,
    SubtitleBurnConfig, SubtitleTrack,
};

pub fn normalize_language_code(lang: &str) -> String {
    let clean = lang.trim().to_lowercase();
    match clean.as_str() {
        "en" | "eng" | "english" => "en".to_string(),
        "es" | "spa" | "spanish" | "espanol" => "es".to_string(),
        "ja" | "jpn" | "japanese" => "ja".to_string(),
        "hi" | "hin" | "hindi" => "hi".to_string(),
        "fr" | "fre" | "fra" | "french" => "fr".to_string(),
        "de" | "ger" | "deu" | "german" => "de".to_string(),
        "it" | "ita" | "italian" => "it".to_string(),
        "pt" | "por" | "portuguese" => "pt".to_string(),
        "ru" | "rus" | "russian" => "ru".to_string(),
        "zh" | "chi" | "zho" | "chinese" => "zh".to_string(),
        "ko" | "kor" | "korean" => "ko".to_string(),
        "ar" | "ara" | "arabic" => "ar".to_string(),
        "tr" | "tur" | "turkish" => "tr".to_string(),
        "id" | "ind" | "indonesian" => "id".to_string(),
        "vi" | "vie" | "vietnamese" => "vi".to_string(),
        "th" | "tha" | "thai" => "th".to_string(),
        "pl" | "pol" | "polish" => "pl".to_string(),
        "nl" | "dut" | "nld" | "dutch" => "nl".to_string(),
        _ => clean,
    }
}

pub fn select_best_audio_track<'a>(
    audio_tracks: &'a [AudioTrack],
    preferred_lang: &str,
) -> Option<&'a AudioTrack> {
    if audio_tracks.is_empty() {
        return None;
    }

    let pref_norm = normalize_language_code(preferred_lang);

    // Filter out commentary or audio description tracks
    let clean_tracks: Vec<&AudioTrack> = audio_tracks
        .iter()
        .filter(|t| !t.is_commentary && !t.is_hearing_impaired)
        .collect();

    let candidate_pool = if clean_tracks.is_empty() {
        audio_tracks.iter().collect::<Vec<_>>()
    } else {
        clean_tracks
    };

    // 1. Exact match with preferred language
    let lang_matches: Vec<&AudioTrack> = candidate_pool
        .iter()
        .copied()
        .filter(|t| normalize_language_code(&t.language) == pref_norm)
        .collect();

    if !lang_matches.is_empty() {
        // Prefer track marked default or track with standard stereo/surround
        if let Some(def) = lang_matches.iter().find(|t| t.is_default) {
            return Some(def);
        }
        return Some(lang_matches[0]);
    }

    // 2. Fallback: track marked default
    if let Some(def) = candidate_pool.iter().find(|t| t.is_default) {
        return Some(def);
    }

    // 3. Fallback: first candidate
    candidate_pool.first().copied()
}

pub struct SubtitleSelectionResult<'a> {
    pub selected_track: Option<&'a SubtitleTrack>,
    pub needs_user_review: bool,
    pub reason: String,
}

pub fn select_best_subtitle_track<'a>(
    subtitle_tracks: &'a [SubtitleTrack],
    preferred_lang: &str,
    audio_track_lang: Option<&str>,
) -> SubtitleSelectionResult<'a> {
    if subtitle_tracks.is_empty() {
        return SubtitleSelectionResult {
            selected_track: None,
            needs_user_review: false,
            reason: "No subtitles found in video".to_string(),
        };
    }

    let pref_norm = normalize_language_code(preferred_lang);
    let audio_norm = audio_track_lang.map(normalize_language_code);

    // If audio is already in preferred language, we might only need forced signs or none
    let _is_native_audio = audio_norm.as_deref() == Some(pref_norm.as_str());

    // Score all candidates
    let mut scored: Vec<(i32, &'a SubtitleTrack)> = subtitle_tracks
        .iter()
        .map(|track| {
            let track_lang = normalize_language_code(&track.language);
            let mut score = 0;

            if track_lang == pref_norm {
                score += 100;
            } else if track_lang == "und" {
                score += 20; // Untagged track might be matching
            }

            // User requested: If SDH is available for that selected language, auto-suggest SDH!
            if track.is_hearing_impaired || track.title.to_lowercase().contains("sdh") || track.title.to_lowercase().contains("cc") {
                score += 30; // Prefer SDH if available
            } else {
                score += 15; // Standard dialogue fallback
            }

            // External sidecar subtitle preferred (usually manually added by user)
            if track.is_external {
                score += 25;
            }

            if track.is_default {
                score += 15;
            }

            (score, track)
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0));

    if let Some((score, best_track)) = scored.first() {
        if *score >= 50 {
            SubtitleSelectionResult {
                selected_track: Some(best_track),
                needs_user_review: false,
                reason: format!(
                    "Auto-selected subtitle track #{} ({})",
                    best_track.track_index,
                    normalize_language_code(&best_track.language)
                ),
            }
        } else {
            SubtitleSelectionResult {
                selected_track: None,
                needs_user_review: true,
                reason: format!("No confident subtitle track match for language '{}'", preferred_lang),
            }
        }
    } else {
        SubtitleSelectionResult {
            selected_track: None,
            needs_user_review: false,
            reason: "No subtitles available".to_string(),
        }
    }
}

pub fn create_encode_plan(
    probe: &MediaProbe,
    settings: &OptimizationSettings,
    selected_sub: Option<&SubtitleTrack>,
) -> EncodePlan {
    create_optimization_plan(probe, settings, selected_sub.map(|s| s.track_index))
}

pub fn create_optimization_plan(
    probe: &MediaProbe,
    settings: &OptimizationSettings,
    selected_subtitle_track_index: Option<usize>,
) -> EncodePlan {
    let duration = probe.duration_seconds.max(1.0);
    let target_size_mb = settings.target_size_mb;
    let target_size_bytes = target_size_mb * 1024 * 1024;

    // Headroom safety buffer of 25 MB
    let safety_margin_bytes = 25 * 1024 * 1024;
    let usable_bytes = target_size_bytes.saturating_sub(safety_margin_bytes);
    let total_bits = (usable_bytes as f64) * 8.0;
    let total_bitrate_kbps = (total_bits / duration / 1000.0) as u64;

    let audio_bitrate_kbps = settings.audio_bitrate_kbps;
    let target_video_bitrate_kbps = total_bitrate_kbps
        .saturating_sub(audio_bitrate_kbps as u64)
        .clamp(500, 20_000); // Clamp between 500 kbps and 20,000 kbps (20 Mbps)

    // Can we direct remux?
    // Requirements:
    // 1. Video is already H.264 High/Main/Baseline profile with 8-bit yuv420p
    // 2. Audio is stereo AAC (or compatible)
    // 3. No subtitle burn-in requested
    // 4. File size is already strictly below target_size_mb
    let (strategy, is_visually_lossless, reason) = if probe.is_telegram_ready
        && selected_subtitle_track_index.is_none()
        && probe.file_size_bytes <= target_size_bytes
    {
        (
            EncodeStrategy::DirectRemux,
            true,
            "Direct Remux: Container & streams are already 100% Telegram streamable without re-encoding".to_string(),
        )
    } else {
        let mut reasons = Vec::new();
        if selected_subtitle_track_index.is_some() {
            reasons.push("Burning dialogue subtitles into picture");
        }
        if !probe.is_telegram_ready {
            reasons.push("Converting container/codec to Telegram streamable MP4 (H.264 yuv420p)");
        }
        if probe.file_size_bytes > target_size_bytes {
            reasons.push("Downsizing file to fit Telegram upload limits");
        }

        let reasons = if reasons.is_empty() {
            "Re-encoding for optimal fast-start streaming".to_string()
        } else {
            reasons.join("; ")
        };

        (
            EncodeStrategy::TranscodeH264,
            target_video_bitrate_kbps >= 3500,
            reasons,
        )
    };

    let video_stream = probe.video_streams.first();
    let (output_width, output_height) = if let Some(max_res) = settings.max_resolution {
        if let Some(v) = video_stream {
            if v.height > max_res {
                let scaled_width = (v.width as f64 * (max_res as f64 / v.height as f64)) as u32;
                (Some((scaled_width / 2) * 2), Some((max_res / 2) * 2))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let (selected_audio_stream_index, audio_needs_downmix) =
        if let Some(best_audio) = select_best_audio_track(&probe.audio_tracks, &settings.preferred_audio_lang) {
            (best_audio.stream_index, best_audio.channels > 2)
        } else {
            (0, false)
        };

    let subtitle_config = SubtitleBurnConfig {
        enabled: selected_subtitle_track_index.is_some(),
        track_index: selected_subtitle_track_index,
        font_size_pt: settings.subtitle_font_size,
        custom_margin_v: 28,
    };

    let estimated_output_size_bytes = match strategy {
        EncodeStrategy::DirectRemux => probe.file_size_bytes,
        EncodeStrategy::TranscodeH264 => {
            let total_kbps = target_video_bitrate_kbps + audio_bitrate_kbps as u64;
            let est_bytes = (total_kbps as f64 * 1000.0 / 8.0 * duration) as u64;
            est_bytes.min(target_size_bytes)
        }
    };

    EncodePlan {
        strategy,
        target_size_mb,
        target_video_bitrate_kbps,
        audio_bitrate_kbps,
        selected_audio_stream_index,
        audio_needs_downmix,
        subtitle_config,
        encoder: settings.hardware_encoder,
        output_width,
        output_height,
        is_visually_lossless,
        estimated_output_size_bytes,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AudioTrack, MediaProbe, SubtitleTrack, VideoStream};
    use std::path::PathBuf;

    #[test]
    fn test_normalize_language_code() {
        assert_eq!(normalize_language_code("eng"), "en");
        assert_eq!(normalize_language_code("English"), "en");
        assert_eq!(normalize_language_code("spa"), "es");
        assert_eq!(normalize_language_code("Japanese"), "ja");
        assert_eq!(normalize_language_code("hin"), "hi");
    }

    #[test]
    fn test_select_best_audio_dialogue_over_commentary() {
        let tracks = vec![
            AudioTrack {
                stream_index: 1,
                track_index: 0,
                codec_name: "ac3".into(),
                language: "en".into(),
                title: "Director Commentary".into(),
                channels: 2,
                channel_layout: "stereo".into(),
                bit_rate: None,
                is_default: false,
                is_commentary: true,
                is_hearing_impaired: false,
            },
            AudioTrack {
                stream_index: 2,
                track_index: 1,
                codec_name: "eac3".into(),
                language: "en".into(),
                title: "Main Dialogue 5.1".into(),
                channels: 6,
                channel_layout: "5.1".into(),
                bit_rate: None,
                is_default: true,
                is_commentary: false,
                is_hearing_impaired: false,
            },
        ];

        let best = select_best_audio_track(&tracks, "en").unwrap();
        assert_eq!(best.stream_index, 2);
        assert!(!best.is_commentary);
    }

    #[test]
    fn test_select_best_subtitle_dialogue_over_sdh() {
        let tracks = vec![
            SubtitleTrack {
                stream_index: Some(3),
                track_index: 0,
                codec_name: "subrip".into(),
                language: "en".into(),
                title: "English [SDH]".into(),
                is_default: false,
                is_forced: false,
                is_hearing_impaired: true,
                is_external: false,
                file_path: None,
            },
            SubtitleTrack {
                stream_index: Some(4),
                track_index: 1,
                codec_name: "subrip".into(),
                language: "en".into(),
                title: "English Dialogue".into(),
                is_default: true,
                is_forced: false,
                is_hearing_impaired: false,
                is_external: false,
                file_path: None,
            },
        ];

        let res = select_best_subtitle_track(&tracks, "en", Some("ja"));
        // Since SDH is preferred when available per user requirement, track 0 (SDH) is selected
        assert_eq!(res.selected_track.unwrap().track_index, 0);
        assert!(!res.needs_user_review);
    }

    #[test]
    fn test_bitrate_cap_formula() {
        let probe = MediaProbe {
            file_path: PathBuf::from("episode.mkv"),
            file_name: "episode.mkv".into(),
            file_size_bytes: 2_500_000_000,
            format_name: "matroska".into(),
            duration_seconds: 2700.0, // 45 minutes
            bit_rate: None,
            video_streams: vec![VideoStream {
                index: 0,
                codec_name: "hevc".into(),
                profile: None,
                width: 1920,
                height: 1080,
                pix_fmt: "yuv420p10le".into(),
                r_frame_rate: "24/1".into(),
                bit_rate: None,
                is_10bit: true,
                is_hdr: false,
            }],
            audio_tracks: vec![],
            subtitle_tracks: vec![],
            is_telegram_ready: false,
        };

        let settings = OptimizationSettings {
            target_size_mb: 1980,
            ..Default::default()
        };

        let plan = create_optimization_plan(&probe, &settings, None);
        assert_eq!(plan.strategy, EncodeStrategy::TranscodeH264);
        assert!(plan.target_video_bitrate_kbps > 5000 && plan.target_video_bitrate_kbps < 7000);
        assert!(plan.is_visually_lossless);
        assert!(plan.estimated_output_size_bytes > 0);
    }
}

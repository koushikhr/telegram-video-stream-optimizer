use crate::models::HardwareEncoder;
use tokio::process::Command;
use tracing::info;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HardwareCapabilities {
    pub has_nvidia_nvenc: bool,
    pub has_amd_amf: bool,
    pub has_intel_qsv: bool,
    pub has_apple_videotoolbox: bool,
    pub has_cpu_x264: bool,
    pub recommended_encoder: HardwareEncoder,
}

pub async fn detect_hardware_encoders() -> HardwareCapabilities {
    let output = Command::new("ffmpeg")
        .args(["-encoders"])
        .output()
        .await;

    let encoder_str = match output {
        Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
        Err(_) => String::new(),
    };

    let has_nvidia_nvenc = encoder_str.contains("h264_nvenc");
    let has_amd_amf = encoder_str.contains("h264_amf");
    let has_intel_qsv = encoder_str.contains("h264_qsv");
    let has_apple_videotoolbox = encoder_str.contains("h264_videotoolbox");
    let has_cpu_x264 = encoder_str.contains("libx264");

    // Prioritize hardware encoders: NVENC > VideoToolbox > AMF > QSV > CPU
    let recommended_encoder = if has_nvidia_nvenc {
        HardwareEncoder::NvidiaNvenc
    } else if has_apple_videotoolbox {
        HardwareEncoder::AppleVideoToolbox
    } else if has_amd_amf {
        HardwareEncoder::AmdAmf
    } else if has_intel_qsv {
        HardwareEncoder::IntelQsv
    } else {
        HardwareEncoder::CpuX264
    };

    info!(
        "Detected Hardware Encoders: NVENC={}, AMF={}, QSV={}, VideoToolbox={}, CPU={}. Recommended: {:?}",
        has_nvidia_nvenc, has_amd_amf, has_intel_qsv, has_apple_videotoolbox, has_cpu_x264, recommended_encoder
    );

    HardwareCapabilities {
        has_nvidia_nvenc,
        has_amd_amf,
        has_intel_qsv,
        has_apple_videotoolbox,
        has_cpu_x264,
        recommended_encoder,
    }
}

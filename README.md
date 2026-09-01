# Telegram Video Stream Optimizer

A universal, cross-platform desktop application designed to convert, optimize, and prepare any video (movies, TV shows, anime) so it can be uploaded to Telegram and streamed instantly inside the app like YouTube, rather than being downloaded as a slow document file.

---

## Key Features

- **Instant In-App Streaming (FastStart)**:
  - Repackages videos placing the metadata index atom (`moov`) at byte 0 (`-movflags +faststart`).
  - Standardizes video to H.264 High Profile 8-bit `yuv420p` and stereo AAC audio for 100% compatibility across Telegram mobile (Android ExoPlayer / iOS AVPlayer), Desktop, and Web.
- **Zero-Loss Smart Remuxing**:
  - Automatically identifies if a video already meets Telegram specifications and fits within your size cap.
  - Remuxes in 2–5 seconds with **zero quality degradation**.
- **Strict Size Limits (Guaranteed Upload Success)**:
  - Automatically computes duration-based bitrate limits to guarantee output is strictly below **1,980 MB (Telegram Free limit)** or **3,980 MB (Premium limit)**.
  - Never face failed uploads due to Telegram's hard file size ceilings.
- **Minimal / Perceptually Lossless 1080p Quality**:
  - Calibrated bitrate math providing ~5,900+ kbps for 45-minute 1080p episodes.
  - Leverages hardware GPU acceleration: **NVIDIA NVENC** (GeForce RTX/GTX), **AMD AMF** (Radeon), **Intel QuickSync (QSV)**, **Apple VideoToolbox**, with multi-threaded CPU fallback (`libx264`).
- **Smart Dialogue & Subtitle Engine**:
  - **Auto-Suggest Language**: Automatically picks dialogue matching your preferred language (defaulting to English, with full ISO-639 support for Spanish, Hindi, Japanese, French, etc.).
  - **Ignores Noise**: Automatically filters out commentary and audio-description tracks.
  - **Sidecar Discovery**: Automatically discovers external subtitles (`.srt`, `.ass`, `.vtt`) matching the video filename in the same folder.
  - **Burn-In with Size Slider**: Cleanly burns subtitles into picture so they display on mobile Telegram players.
- **Live Frame Quality & Subtitle Preview**:
  - Inspect video sharpness and subtitle readability before starting the queue.
  - Interactive font size slider (16pt to 38pt) updates the live preview snapshot in real-time.
- **Drag & Drop Batch Queue**:
  - Drop multiple video files or entire folders.
  - Real-time progress: percentage, encoding speed multiplier (e.g. `3.8x`), FPS, and accurate ETA.
- **Laptop & Power Controls**:
  - Set limits like *"Convert only the next 3 episodes then stop"*.
  - *"Put PC to Sleep when finished"* or shutdown.
  - Safe abort: automatically terminates background processes and purges partial `.part.mp4` files if canceled.

---

## Architecture

```
telegram-video-stream-optimizer/
├── crates/
│   ├── optimizer-core/      # Pure Rust engine: FFprobe parser, bitrate solver,
│   │                        # subtitle burner, preview frame generator, power manager.
│   └── optimizer-cli/       # Headless CLI for terminal and server usage.
├── src-tauri/               # Tauri v2 native desktop application shell.
└── ui/                      # Modern responsive UI built with Svelte 5 & Tailwind CSS.
```

---

## How to Run

### 1. Run the Desktop App (GUI)
Make sure you are in the project root:
```bash
# Start the Tauri v2 Desktop application in development mode
cargo run -p telegram-video-stream-optimizer
```
*(Or run `npm run tauri dev` from the `ui/` directory)*.

### 2. Run the Command-Line Interface (CLI)
You can probe, preview, and transcode directly from the command line:

```bash
# Detect available GPU hardware encoders
cargo run -p optimizer-cli -- detect-hw

# Probe video streams and external sidecar subtitles
cargo run -p optimizer-cli -- probe "path/to/Episode.mkv"

# Generate a live preview frame with burned subtitle at custom font size
cargo run -p optimizer-cli -- preview "path/to/Episode.mkv" --timestamp 30 --font-size 26

# Transcode and optimize a video file for Telegram
cargo run -p optimizer-cli -- optimize "path/to/Episode.mkv" --target-mb 1980 --font-size 24
```

### 3. Run Automated Tests
```bash
cargo test -p optimizer-core
```

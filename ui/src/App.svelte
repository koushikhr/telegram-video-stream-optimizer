<script lang="ts">
  import { onMount } from "svelte";
  import type {
    QueueItem,
    OptimizationSettings,
    HardwareCapabilities,
  } from "./types";
  import Header from "./components/Header.svelte";
  import DropZone from "./components/DropZone.svelte";
  import QueueItemCard from "./components/QueueItemCard.svelte";
  import PreviewModal from "./components/PreviewModal.svelte";
  import TrackSelectorModal from "./components/TrackSelectorModal.svelte";
  import SettingsModal from "./components/SettingsModal.svelte";
  import PowerControlsModal from "./components/PowerControlsModal.svelte";
  import TelemetryFooter from "./components/TelemetryFooter.svelte";
  import {
    detectHardware,
    probeVideo,
    createPlan,
    generatePreview,
    openVideoFiles,
    openVideoFolder,
    triggerPcSleep,
    isTauri,
  } from "./lib/api";

  // State
  let queue: QueueItem[] = [];
  let isRunning = false;
  let sleepOnFinish = false;
  let batchLimit = 0;
  let completedCount = 0;

  let settings: OptimizationSettings = {
    preferred_audio_lang: "en",
    preferred_subtitle_lang: "en",
    target_size_mb: 1980,
    max_resolution: undefined,
    subtitle_font_size: 24,
    hardware_encoder: "Auto",
    audio_bitrate_kbps: 160,
    crf_quality: 19,
  };

  let hwCaps: HardwareCapabilities | null = null;
  let gpuLabel = "Detecting GPU...";

  // Modals state
  let showSettings = false;
  let showPower = false;
  let activePreviewItem: QueueItem | null = null;
  let previewDataUrl = "";
  let isPreviewLoading = false;
  let previewFontSize = 24;
  let previewTimestamp = 30;

  let activeTrackItem: QueueItem | null = null;

  onMount(async () => {
    try {
      hwCaps = await detectHardware();
      if (hwCaps.has_nvidia_nvenc) {
        gpuLabel = "NVIDIA NVENC (RTX) Accelerated";
      } else if (hwCaps.has_amd_amf) {
        gpuLabel = "AMD AMF Accelerated";
      } else if (hwCaps.has_apple_videotoolbox) {
        gpuLabel = "Apple VideoToolbox Ready";
      } else if (hwCaps.has_intel_qsv) {
        gpuLabel = "Intel QuickSync Ready";
      } else {
        gpuLabel = "Software CPU (libx264)";
      }
    } catch (e) {
      gpuLabel = "GPU Prober Initialized";
    }

    // Setup Tauri file drag & drop listeners if running in Tauri
    if (isTauri()) {
      import("@tauri-apps/api/event").then(({ listen }) => {
        listen<{ paths: string[] }>("tauri://drag-drop", async (event) => {
          if (event.payload?.paths?.length) {
            await handleAddPaths(event.payload.paths);
          }
        });
      });
    }
  });

  async function handleAddPaths(paths: string[]) {
    for (const p of paths) {
      const id = Math.random().toString(36).substring(2, 9);
      const name = p.split(/[/\\]/).pop() || "video.mp4";

      const newItem: QueueItem = {
        id,
        input_path: p,
        output_path: p.replace(/\.[^/.]+$/, "") + " [TG].mp4",
        file_name: name,
        file_size_bytes: 0,
        duration_seconds: 0,
        status: "Probing",
      };

      queue = [...queue, newItem];

      // Probe item asynchronously
      try {
        const probe = await probeVideo(p);
        newItem.probe = probe;
        newItem.file_size_bytes = probe.file_size_bytes;
        newItem.duration_seconds = probe.duration_seconds;

        // Smart auto-select audio track
        const prefAudio = probe.audio_tracks.find(
          (a) => a.language.toLowerCase() === settings.preferred_audio_lang.toLowerCase() && !a.is_commentary
        ) || probe.audio_tracks[0];
        newItem.selected_audio_track_index = prefAudio?.track_index;

        // Smart auto-select subtitle track
        const prefSub = probe.subtitle_tracks.find(
          (s) => s.language.toLowerCase() === settings.preferred_subtitle_lang.toLowerCase()
        );
        newItem.selected_subtitle_track_index = prefSub?.track_index;

        if (probe.subtitle_tracks.length > 0 && !prefSub) {
          newItem.status = "NeedsReview";
        } else {
          newItem.status = "Ready";
        }

        const plan = await createPlan(probe, settings, newItem.selected_subtitle_track_index);
        newItem.plan = plan;
      } catch (err: any) {
        newItem.status = "Failed";
        newItem.error_message = err?.message || "Failed to analyze video file";
      }

      queue = [...queue];
    }
  }

  async function handleAddFiles() {
    const paths = await openVideoFiles();
    if (paths.length) {
      await handleAddPaths(paths);
    }
  }

  async function handleAddFolder() {
    const paths = await openVideoFolder();
    if (paths.length) {
      await handleAddPaths(paths);
    }
  }

  function handleRemoveItem(id: string) {
    queue = queue.filter((item) => item.id !== id);
  }

  async function handleOpenPreview(item: QueueItem) {
    activePreviewItem = item;
    previewFontSize = item.plan?.subtitle_config.font_size_pt || settings.subtitle_font_size || 24;
    previewTimestamp = Math.min(item.duration_seconds * 0.1 || 30, 120);
    await refreshPreview(item, previewFontSize, previewTimestamp);
  }

  async function refreshPreview(item: QueueItem, fontSz: number, timeSec: number) {
    isPreviewLoading = true;
    try {
      previewDataUrl = await generatePreview(
        item.input_path,
        timeSec,
        item.selected_subtitle_track_index,
        fontSz
      );
    } catch (e: any) {
      console.error("Preview error:", e);
    } finally {
      isPreviewLoading = false;
    }
  }

  function handleApplyFontSize(fontSz: number) {
    if (activePreviewItem) {
      if (activePreviewItem.plan) {
        activePreviewItem.plan.subtitle_config.font_size_pt = fontSz;
      }
      queue = [...queue];
    }
    settings.subtitle_font_size = fontSz;
  }

  function handleOpenTracks(item: QueueItem) {
    activeTrackItem = item;
  }

  async function handleSaveTracks(audioIdx: number, subIdx: number | undefined) {
    if (activeTrackItem && activeTrackItem.probe) {
      activeTrackItem.selected_audio_track_index = audioIdx;
      activeTrackItem.selected_subtitle_track_index = subIdx;
      activeTrackItem.status = "Ready";

      const plan = await createPlan(activeTrackItem.probe, settings, subIdx);
      activeTrackItem.plan = plan;
      queue = [...queue];
    }
  }

  async function handleStartQueue() {
    if (isRunning || queue.length === 0) return;
    isRunning = true;

    let processedInThisRun = 0;

    for (let i = 0; i < queue.length; i++) {
      if (!isRunning) break;
      const item = queue[i];
      if (item.status === "Completed") continue;

      if (batchLimit > 0 && processedInThisRun >= batchLimit) {
        break;
      }

      item.status = "Processing";
      queue = [...queue];

      try {
        if (isTauri()) {
          const { invoke } = await import("@tauri-apps/api/core");
          await invoke("start_transcode_item", {
            item,
            fontSz: item.plan?.subtitle_config.font_size_pt || settings.subtitle_font_size,
          });
        } else {
          // Browser simulation: simulate smooth progress
          for (let p = 0; p <= 100; p += 10) {
            if (!isRunning) break;
            item.progress = {
              percent: p,
              fps: 145.0,
              speed_multiplier: 3.8,
              current_time_secs: (item.duration_seconds * p) / 100,
              total_duration_secs: item.duration_seconds || 2400,
              eta_seconds: ((100 - p) / 100) * 60,
              current_size_bytes: (p / 100) * (item.plan?.target_size_mb || 1980) * 1024 * 1024,
              target_size_bytes: (item.plan?.target_size_mb || 1980) * 1024 * 1024,
              stage: item.plan?.strategy === "DirectRemux" ? "Lossless Remuxing" : "Transcoding H.264",
            };
            queue = [...queue];
            await new Promise((r) => setTimeout(r, 200));
          }
        }

        item.status = "Completed";
        completedCount++;
        processedInThisRun++;
      } catch (err: any) {
        item.status = "Failed";
        item.error_message = err?.message || "Transcode error";
      }

      queue = [...queue];
    }

    isRunning = false;

    // Check sleep on finish
    if (sleepOnFinish && completedCount === queue.length) {
      await triggerPcSleep();
    }
  }

  function handlePauseQueue() {
    isRunning = false;
  }

  function handleCancelQueue() {
    isRunning = false;
    if (isTauri()) {
      import("@tauri-apps/api/core").then(({ invoke }) => {
        invoke("cancel_current_transcode");
      });
    }
  }
</script>

<main class="flex flex-col h-screen w-screen overflow-hidden bg-slate-950 text-slate-100 font-sans">
  <Header
    onAddFiles={handleAddFiles}
    onAddFolder={handleAddFolder}
    onOpenSettings={() => (showSettings = true)}
    onOpenPower={() => (showPower = true)}
    {gpuLabel}
    {sleepOnFinish}
    {batchLimit}
  />

  <!-- Main Content Area -->
  <div class="flex-1 overflow-y-auto flex flex-col min-h-0">
    {#if queue.length === 0}
      <DropZone onAddFiles={handleAddFiles} onAddFolder={handleAddFolder} />
    {:else}
      <div class="p-6 space-y-3 max-w-5xl mx-auto w-full">
        <div class="flex items-center justify-between mb-2">
          <h2 class="text-xs font-semibold uppercase tracking-wider text-slate-400">
            Batch Optimization Queue ({queue.length})
          </h2>
          <button
            on:click={() => (queue = [])}
            class="text-[11px] text-slate-500 hover:text-rose-400 transition"
          >
            Clear All
          </button>
        </div>

        {#each queue as item (item.id)}
          <QueueItemCard
            {item}
            onPreview={handleOpenPreview}
            onSelectTracks={handleOpenTracks}
            onRemove={handleRemoveItem}
          />
        {/each}
      </div>
    {/if}
  </div>

  <TelemetryFooter
    totalCount={queue.length}
    {completedCount}
    {isRunning}
    onStart={handleStartQueue}
    onPause={handlePauseQueue}
    onCancel={handleCancelQueue}
  />

  <!-- Modals -->
  {#if activePreviewItem}
    <PreviewModal
      item={activePreviewItem}
      {previewDataUrl}
      isLoading={isPreviewLoading}
      fontSize={previewFontSize}
      timestamp={previewTimestamp}
      onClose={() => (activePreviewItem = null)}
      onRefresh={(f, t) => activePreviewItem && refreshPreview(activePreviewItem, f, t)}
      onApplyFontSize={handleApplyFontSize}
    />
  {/if}

  {#if activeTrackItem}
    <TrackSelectorModal
      item={activeTrackItem}
      onClose={() => (activeTrackItem = null)}
      onSave={handleSaveTracks}
    />
  {/if}

  {#if showSettings}
    <SettingsModal
      {settings}
      {hwCaps}
      onClose={() => (showSettings = false)}
      onSave={(newS) => (settings = newS)}
    />
  {/if}

  {#if showPower}
    <PowerControlsModal
      {sleepOnFinish}
      {batchLimit}
      queueLength={queue.length}
      onClose={() => (showPower = false)}
      onSave={(sleep, limit) => {
        sleepOnFinish = sleep;
        batchLimit = limit;
      }}
    />
  {/if}
</main>

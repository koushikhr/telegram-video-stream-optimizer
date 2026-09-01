<script lang="ts">
  import type { QueueItem } from "../types";
  import {
    Film,
    Volume2,
    Subtitles,
    Eye,
    SlidersHorizontal,
    Trash2,
    CheckCircle2,
    AlertCircle,
    Loader2,
    Sparkles,
    Zap,
  } from "lucide-svelte";
  import { getLanguageName } from "../lib/languages";

  export let item: QueueItem;
  export let onPreview: (item: QueueItem) => void;
  export let onSelectTracks: (item: QueueItem) => void;
  export let onRemove: (id: string) => void;

  function formatBytes(bytes: number): string {
    if (!bytes || bytes === 0) return "0 MB";
    const mb = bytes / (1024 * 1024);
    if (mb >= 1024) {
      return `${(mb / 1024).toFixed(2)} GB`;
    }
    return `${mb.toFixed(1)} MB`;
  }

  function formatTime(secs: number): string {
    if (!secs || isNaN(secs) || secs <= 0) return "0s";
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    if (m > 60) {
      const h = Math.floor(m / 60);
      return `${h}h ${m % 60}m`;
    }
    return m > 0 ? `${m}m ${s}s` : `${s}s`;
  }

  $: currentAudio = item.probe?.audio_tracks.find(
    (a) => a.track_index === item.selected_audio_track_index
  ) || item.probe?.audio_tracks[0];

  $: currentSub = item.probe?.subtitle_tracks.find(
    (s) => s.track_index === item.selected_subtitle_track_index
  );
</script>

<div class="bg-slate-900/70 border border-slate-800 hover:border-slate-700/80 rounded-xl p-4 transition-all duration-200 shadow-sm relative overflow-hidden group">
  <!-- Top row: icon, name, sizes, actions -->
  <div class="flex items-start justify-between gap-3">
    <div class="flex items-start gap-3 min-w-0 flex-1">
      <div class="w-10 h-10 rounded-lg bg-slate-800 border border-slate-700/80 flex items-center justify-center shrink-0 text-slate-400 group-hover:text-sky-400 transition-colors">
        {#if item.status === "Processing"}
          <Loader2 class="w-5 h-5 text-sky-400 animate-spin" />
        {:else if item.status === "Completed"}
          <CheckCircle2 class="w-5 h-5 text-emerald-400" />
        {:else if item.status === "Failed"}
          <AlertCircle class="w-5 h-5 text-rose-400" />
        {:else}
          <Film class="w-5 h-5" />
        {/if}
      </div>

      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-2 mb-1">
          <span class="font-medium text-sm text-slate-100 truncate block title={item.file_name}">
            {item.file_name}
          </span>
        </div>

        <div class="flex items-center gap-2.5 text-xs text-slate-400 flex-wrap">
          <span>Input: <strong class="text-slate-200 font-medium">{formatBytes(item.file_size_bytes)}</strong></span>
          <span class="text-slate-600">•</span>
          <span>{formatTime(item.duration_seconds)}</span>
          {#if item.plan}
            <span class="text-slate-600">•</span>
            <span class="px-2 py-0.5 rounded-md bg-sky-500/10 border border-sky-500/20 text-sky-400 font-medium flex items-center gap-1">
              <span>Est. Output:</span>
              <strong class="font-mono font-semibold text-sky-300">
                ~{formatBytes(item.plan.estimated_output_size_bytes || item.file_size_bytes)}
              </strong>
            </span>
            <span class="text-slate-500 text-[11px] font-mono">
              (Cap: {item.plan.target_size_mb} MB)
            </span>
          {/if}
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-1 shrink-0">
      <button
        on:click={() => onPreview(item)}
        class="p-1.5 text-slate-400 hover:text-sky-300 hover:bg-slate-800 rounded-lg transition active:scale-95 cursor-pointer"
        title="Live video quality & subtitle preview"
      >
        <Eye class="w-4 h-4" />
      </button>

      <button
        on:click={() => onSelectTracks(item)}
        class="p-1.5 text-slate-400 hover:text-slate-200 hover:bg-slate-800 rounded-lg transition active:scale-95 cursor-pointer"
        title="Choose audio & subtitle tracks"
      >
        <SlidersHorizontal class="w-4 h-4" />
      </button>

      {#if item.status !== "Processing"}
        <button
          on:click={() => onRemove(item.id)}
          class="p-1.5 text-slate-500 hover:text-rose-400 hover:bg-rose-500/10 rounded-lg transition active:scale-95 cursor-pointer"
          title="Remove from queue"
        >
          <Trash2 class="w-4 h-4" />
        </button>
      {/if}
    </div>
  </div>

  <!-- Middle badges row: Audio, Subtitle, Strategy -->
  <div class="mt-3 flex flex-wrap items-center gap-2">
    <!-- Strategy Badge -->
    {#if item.plan?.strategy === "DirectRemux"}
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
        <Zap class="w-3 h-3" />
        <span>Lossless Remux (0% Quality Loss)</span>
      </span>
    {:else if item.plan?.strategy === "TranscodeH264"}
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-sky-500/10 text-sky-400 border border-sky-500/20">
        <Sparkles class="w-3 h-3" />
        <span>H.264 High-Quality ({item.plan.target_video_bitrate_kbps}k)</span>
      </span>
    {/if}

    <!-- Audio Badge -->
    {#if currentAudio}
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-slate-800 text-slate-300 border border-slate-700">
        <Volume2 class="w-3 h-3 text-sky-400" />
        <span>{getLanguageName(currentAudio.language)}</span>
        {#if currentAudio.channels > 2}
          <span class="text-[10px] text-amber-400">-> Stereo</span>
        {/if}
      </span>
    {/if}

    <!-- Subtitle Badge -->
    {#if currentSub}
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-indigo-500/10 text-indigo-300 border border-indigo-500/20">
        <Subtitles class="w-3 h-3 text-indigo-400" />
        <span>Burn: {getLanguageName(currentSub.language)} ({item.plan?.subtitle_config.font_size_pt || 24}pt)</span>
      </span>
    {:else if item.status === "NeedsReview"}
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-amber-500/10 text-amber-300 border border-amber-500/20">
        <AlertCircle class="w-3 h-3 text-amber-400" />
        <span>Subtitle Review Suggested</span>
      </span>
    {:else}
      <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[11px] font-medium bg-slate-800 text-slate-400 border border-slate-700/60">
        <Subtitles class="w-3 h-3 opacity-50" />
        <span>No Subtitles</span>
      </span>
    {/if}
  </div>

  <!-- Progress bar if processing -->
  {#if item.status === "Processing" && item.progress}
    <div class="mt-3 pt-3 border-t border-slate-800">
      <div class="flex items-center justify-between text-xs mb-1.5 font-mono">
        <span class="text-sky-400 font-semibold">{item.progress.stage} ({item.progress.percent.toFixed(1)}%)</span>
        <div class="flex items-center gap-3 text-slate-400">
          <span>{item.progress.fps.toFixed(0)} fps</span>
          <span class="text-sky-400 font-semibold">{item.progress.speed_multiplier.toFixed(1)}x</span>
          <span>ETA: {formatTime(item.progress.eta_seconds)}</span>
        </div>
      </div>
      <div class="w-full h-1.5 bg-slate-800 rounded-full overflow-hidden">
        <div
          class="h-full bg-gradient-to-r from-sky-500 to-blue-600 rounded-full transition-all duration-300"
          style="width: {item.progress.percent}%"
        ></div>
      </div>
    </div>
  {/if}

  <!-- Error message if failed -->
  {#if item.status === "Failed" && item.error_message}
    <div class="mt-3 p-2 bg-rose-500/10 border border-rose-500/20 rounded-lg text-xs text-rose-300 flex items-start gap-2">
      <AlertCircle class="w-4 h-4 shrink-0 mt-0.5" />
      <span>{item.error_message}</span>
    </div>
  {/if}
</div>

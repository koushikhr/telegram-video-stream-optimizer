<script lang="ts">
  import type { QueueItem } from "../types";
  import { X, RefreshCw, Eye, AlertCircle, Maximize2, Minimize2 } from "lucide-svelte";
  import { getLanguageName } from "../lib/languages";

  export let item: QueueItem;
  export let previewDataUrl: string = "";
  export let isLoading: boolean = false;
  export let fontSize: number = 24;
  export let timestamp: number = 30;
  export let borderStyle: number = 1; // 1 = Clean Outline, 3 = Black Box
  export let onClose: () => void;
  export let onRefresh: (fontSize: number, timestamp: number, borderStyle: number) => void;
  export let onApplySubtitleConfig: (fontSize: number, borderStyle: number) => void;

  let isFullscreenView = false;

  function formatBytes(bytes?: number): string {
    if (!bytes || bytes === 0) return "0 MB";
    const mb = bytes / (1024 * 1024);
    if (mb >= 1024) {
      return `${(mb / 1024).toFixed(2)} GB`;
    }
    return `${mb.toFixed(1)} MB`;
  }

  function handleSliderChange(e: Event) {
    const val = parseInt((e.target as HTMLInputElement).value, 10);
    fontSize = val;
  }

  function handleTimeChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    timestamp = val;
  }

  function jumpToTime(t: number) {
    timestamp = t;
    onRefresh(fontSize, timestamp, borderStyle);
  }

  function handleApply() {
    onApplySubtitleConfig(fontSize, borderStyle);
    onClose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (isFullscreenView) {
        isFullscreenView = false;
      } else {
        onClose();
      }
    }
  }

  $: currentSub = item.probe?.subtitle_tracks.find(
    (s) => s.track_index === item.selected_subtitle_track_index
  );
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm flex items-center justify-center p-6 animate-in fade-in duration-150">
  <div class="bg-[#16181d] border border-[#2b2e38] rounded-xl w-full max-w-4xl max-h-[90vh] flex flex-col shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-5 py-3.5 border-b border-[#252830] flex items-center justify-between shrink-0 bg-[#121418]">
      <div class="flex items-center gap-3">
        <div class="w-7 h-7 rounded-lg bg-[#2481cc]/15 border border-[#2481cc]/30 flex items-center justify-center text-[#3897e0]">
          <Eye class="w-3.5 h-3.5" />
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h3 class="font-semibold text-xs text-slate-100 uppercase tracking-wide">Video & Subtitle Preview</h3>
            {#if item.plan?.estimated_output_size_bytes}
              <span class="px-2 py-0.5 text-[10px] font-medium bg-[#1e222b] text-slate-300 border border-[#2c303c] rounded">
                Est. Size: ~{formatBytes(item.plan.estimated_output_size_bytes)}
              </span>
            {/if}
          </div>
          <p class="text-[11px] text-slate-400 truncate max-w-lg mt-0.5">{item.file_name}</p>
        </div>
      </div>
      <button
        on:click={onClose}
        class="p-1.5 text-slate-400 hover:text-slate-200 hover:bg-[#20232b] rounded-md transition cursor-pointer"
      >
        <X class="w-4 h-4" />
      </button>
    </div>

    <!-- Image Display Area -->
    <div class="flex-1 bg-[#0a0b0d] relative flex items-center justify-center overflow-hidden min-h-[380px] p-4">
      {#if isLoading}
        <div class="flex flex-col items-center gap-2.5 text-slate-400">
          <RefreshCw class="w-6 h-6 animate-spin text-[#3897e0]" />
          <span class="text-xs">Rendering video frame...</span>
        </div>
      {:else if previewDataUrl}
        <img
          src={previewDataUrl}
          alt="Video Preview Frame"
          class="max-w-full max-h-[50vh] object-contain rounded border border-[#23262f] shadow-lg"
        />
      {:else}
        <div class="flex flex-col items-center gap-2 text-slate-500">
          <AlertCircle class="w-6 h-6" />
          <span class="text-xs">No preview frame available</span>
        </div>
      {/if}

      <!-- Full Screen Trigger Button -->
      {#if previewDataUrl && !isLoading}
        <button
          type="button"
          on:click={() => (isFullscreenView = true)}
          class="absolute top-4 right-4 bg-[#14161c]/90 hover:bg-[#1e222b] text-slate-200 hover:text-white border border-[#2b2e38] rounded-md px-2.5 py-1.5 text-xs font-medium flex items-center gap-1.5 shadow transition cursor-pointer backdrop-blur active:scale-95"
          title="Open in Full Screen (Inspect quality & subtitle scale)"
        >
          <Maximize2 class="w-3.5 h-3.5 text-[#3897e0]" />
          <span>Full Screen</span>
        </button>
      {/if}

      {#if currentSub}
        <div class="absolute top-4 left-4 bg-[#14161c]/90 backdrop-blur border border-[#2b2e38] rounded-md px-2.5 py-1 text-[11px] text-slate-300 flex items-center gap-1.5 shadow">
          <span class="w-1.5 h-1.5 rounded-full bg-[#3897e0]"></span>
          <span>{getLanguageName(currentSub.language)} {currentSub.is_hearing_impaired || currentSub.title.toLowerCase().includes("sdh") ? "(SDH)" : ""} • {fontSize}pt ({borderStyle === 1 ? 'Outline' : 'Box'})</span>
        </div>
      {:else}
        <div class="absolute top-4 left-4 bg-[#14161c]/90 backdrop-blur border border-[#2b2e38] rounded-md px-2.5 py-1 text-[11px] text-slate-400">
          No subtitle track selected
        </div>
      {/if}
    </div>

    <!-- Controls Area -->
    <div class="p-5 border-t border-[#252830] bg-[#121418] space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Subtitle Size & Style Control -->
        <div class="space-y-3">
          <div class="flex items-center justify-between text-xs">
            <span class="text-slate-300 font-medium">Subtitle Style</span>
            <div class="flex items-center gap-1 bg-[#1a1c23] p-0.5 rounded-lg border border-[#282b36]">
              <button
                type="button"
                on:click={() => { borderStyle = 1; onRefresh(fontSize, timestamp, 1); }}
                class="px-2.5 py-1 text-[11px] font-medium rounded transition cursor-pointer {borderStyle === 1 ? 'bg-[#2481cc] text-white' : 'text-slate-400 hover:text-slate-200'}"
              >
                Clean Outline
              </button>
              <button
                type="button"
                on:click={() => { borderStyle = 3; onRefresh(fontSize, timestamp, 3); }}
                class="px-2.5 py-1 text-[11px] font-medium rounded transition cursor-pointer {borderStyle === 3 ? 'bg-[#2481cc] text-white' : 'text-slate-400 hover:text-slate-200'}"
              >
                Black Box
              </button>
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between text-xs mb-1.5">
              <span class="text-slate-300 font-medium">Subtitle Font Size</span>
              <span class="text-[#3897e0] font-mono font-semibold">{fontSize} pt</span>
            </div>
            <input
              type="range"
              min="8"
              max="36"
              step="1"
              value={fontSize}
              on:input={handleSliderChange}
              class="w-full accent-[#2481cc] bg-[#222530] h-1.5 rounded-lg appearance-none cursor-pointer"
            />
            <div class="flex justify-between text-[10px] text-slate-500 mt-1 font-mono">
              <span>8pt (Mini)</span>
              <span>12pt</span>
              <span>16pt</span>
              <span>22pt</span>
              <span>28pt</span>
              <span>36pt (Max)</span>
            </div>
          </div>
        </div>

        <!-- Scene Timestamp Seek Control -->
        <div>
          <div class="flex items-center justify-between text-xs mb-1.5">
            <span class="text-slate-300 font-medium">Scene Timestamp</span>
            <span class="text-slate-400 font-mono">{timestamp.toFixed(0)}s ({Math.floor(timestamp / 60)}m {Math.floor(timestamp % 60)}s)</span>
          </div>
          <input
            type="range"
            min="5"
            max={Math.min(item.duration_seconds || 1200, 1200)}
            step="5"
            value={timestamp}
            on:input={handleTimeChange}
            class="w-full accent-[#2481cc] bg-[#222530] h-1.5 rounded-lg appearance-none cursor-pointer"
          />
          <!-- Quick dialogue scene jump buttons -->
          <div class="flex items-center gap-1.5 mt-2">
            <span class="text-[10px] text-slate-500">Jump to:</span>
            <button
              type="button"
              on:click={() => jumpToTime(30)}
              class="px-2 py-0.5 text-[10px] bg-[#1a1c23] hover:bg-[#222530] text-slate-300 rounded border border-[#282b36] transition cursor-pointer"
            >
              30s
            </button>
            <button
              type="button"
              on:click={() => jumpToTime(60)}
              class="px-2 py-0.5 text-[10px] bg-[#1a1c23] hover:bg-[#222530] text-slate-300 rounded border border-[#282b36] transition cursor-pointer"
            >
              1m
            </button>
            <button
              type="button"
              on:click={() => jumpToTime(120)}
              class="px-2 py-0.5 text-[10px] bg-[#1a1c23] hover:bg-[#222530] text-slate-300 rounded border border-[#282b36] transition cursor-pointer"
            >
              2m
            </button>
            <button
              type="button"
              on:click={() => jumpToTime(300)}
              class="px-2 py-0.5 text-[10px] bg-[#1a1c23] hover:bg-[#222530] text-slate-300 rounded border border-[#282b36] transition cursor-pointer"
            >
              5m
            </button>
            <button
              type="button"
              on:click={() => jumpToTime(600)}
              class="px-2 py-0.5 text-[10px] bg-[#1a1c23] hover:bg-[#222530] text-slate-300 rounded border border-[#282b36] transition cursor-pointer"
            >
              10m
            </button>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center justify-between pt-2 border-t border-[#252830]">
        <button
          type="button"
          on:click={() => onRefresh(fontSize, timestamp, borderStyle)}
          disabled={isLoading}
          class="px-3 py-1.5 text-xs font-medium bg-[#1e222b] hover:bg-[#272c38] text-slate-200 border border-[#2c303d] rounded-md flex items-center gap-1.5 transition active:scale-95 disabled:opacity-50 cursor-pointer"
        >
          <RefreshCw class={`w-3.5 h-3.5 ${isLoading ? "animate-spin text-[#3897e0]" : ""}`} />
          <span>Update Preview</span>
        </button>

        <div class="flex items-center gap-2">
          <button
            type="button"
            on:click={onClose}
            class="px-3.5 py-1.5 text-xs font-medium text-slate-400 hover:text-slate-200 transition cursor-pointer"
          >
            Cancel
          </button>
          <button
            type="button"
            on:click={handleApply}
            class="px-4 py-1.5 text-xs font-medium bg-[#2481cc] hover:bg-[#2075b8] text-white rounded-md transition active:scale-95 cursor-pointer shadow-sm"
          >
            Apply & Save Size
          </button>
        </div>
      </div>
    </div>
  </div>
</div>

{#if isFullscreenView && previewDataUrl}
  <div
    class="fixed inset-0 z-[100] bg-black flex flex-col items-center justify-center p-0 animate-in fade-in duration-150 select-none cursor-pointer"
    on:click={() => (isFullscreenView = false)}
    on:keydown={(e) => { if (e.key === "Escape") isFullscreenView = false; }}
    role="dialog"
    aria-modal="true"
    tabindex="0"
  >
    <!-- Top float bar -->
    <div class="absolute top-4 left-4 right-4 flex items-center justify-between pointer-events-none z-10">
      <div class="bg-[#121418]/90 backdrop-blur border border-white/10 rounded-md px-3 py-1.5 text-xs text-slate-300 pointer-events-auto flex items-center gap-2 shadow-xl">
        <span class="font-medium text-white">{item.file_name}</span>
        <span class="text-slate-600">•</span>
        <span class="text-[#3897e0] font-mono">{fontSize} pt</span>
        <span class="text-slate-600">•</span>
        <span class="text-slate-400 text-[11px]">Click anywhere or press ESC to exit</span>
      </div>
      <button
        type="button"
        on:click|stopPropagation={() => (isFullscreenView = false)}
        class="p-2 bg-[#121418]/90 hover:bg-white/10 text-white rounded-md border border-white/10 transition pointer-events-auto cursor-pointer shadow-xl"
        title="Exit Full Screen (ESC)"
      >
        <Minimize2 class="w-4 h-4" />
      </button>
    </div>

    <!-- Full screen image -->
    <img
      src={previewDataUrl}
      alt="Full Screen Video Snapshot"
      class="w-full h-full object-contain max-h-screen cursor-pointer"
    />
  </div>
{/if}

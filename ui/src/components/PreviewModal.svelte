<script lang="ts">
  import type { QueueItem } from "../types";
  import { X, RefreshCw, Sliders, Eye, Sparkles, AlertCircle } from "lucide-svelte";
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

  $: currentSub = item.probe?.subtitle_tracks.find(
    (s) => s.track_index === item.selected_subtitle_track_index
  );
</script>

<div class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-md flex items-center justify-center p-6 animate-in fade-in duration-200">
  <div class="bg-slate-900 border border-slate-800 rounded-2xl w-full max-w-4xl max-h-[90vh] flex flex-col shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-6 py-4 border-b border-slate-800 flex items-center justify-between shrink-0">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-sky-500/10 border border-sky-500/20 flex items-center justify-center text-sky-400">
          <Eye class="w-4 h-4" />
        </div>
        <div>
          <div class="flex items-center gap-2">
            <h3 class="font-semibold text-sm text-slate-100">Live Video & Subtitle Preview</h3>
            {#if item.plan?.estimated_output_size_bytes}
              <span class="px-2 py-0.5 text-[10px] font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded-md">
                Est. Final Size: ~{formatBytes(item.plan.estimated_output_size_bytes)}
              </span>
            {/if}
          </div>
          <p class="text-xs text-slate-400 truncate max-w-lg">{item.file_name}</p>
        </div>
      </div>
      <button
        on:click={onClose}
        class="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition"
      >
        <X class="w-4 h-4" />
      </button>
    </div>

    <!-- Image Display Area -->
    <div class="flex-1 bg-black/60 relative flex items-center justify-center overflow-hidden min-h-[360px] p-4">
      {#if isLoading}
        <div class="flex flex-col items-center gap-3 text-slate-400">
          <RefreshCw class="w-8 h-8 animate-spin text-sky-400" />
          <span class="text-xs">Rendering preview snapshot with burned subtitle...</span>
        </div>
      {:else if previewDataUrl}
        <img
          src={previewDataUrl}
          alt="Video Preview Frame"
          class="max-w-full max-h-[50vh] object-contain rounded-lg shadow-2xl border border-slate-800/80"
        />
      {:else}
        <div class="flex flex-col items-center gap-2 text-slate-500">
          <AlertCircle class="w-8 h-8" />
          <span class="text-xs">No preview frame available</span>
        </div>
      {/if}

      {#if currentSub}
        <div class="absolute top-4 left-4 bg-slate-900/80 backdrop-blur border border-slate-800 rounded-lg px-2.5 py-1 text-[11px] text-slate-300 flex items-center gap-1.5 shadow">
          <span class="w-2 h-2 rounded-full bg-indigo-500"></span>
          <span>Subtitle: {getLanguageName(currentSub.language)} {currentSub.is_hearing_impaired || currentSub.title.toLowerCase().includes("sdh") ? "(SDH)" : ""} ({fontSize}pt • {borderStyle === 1 ? 'Outline' : 'Box'})</span>
        </div>
      {:else}
        <div class="absolute top-4 left-4 bg-slate-900/80 backdrop-blur border border-slate-800 rounded-lg px-2.5 py-1 text-[11px] text-slate-400">
          No subtitle track selected
        </div>
      {/if}
    </div>

    <!-- Controls Area -->
    <div class="p-5 border-t border-slate-800 bg-slate-900/90 space-y-4">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Subtitle Size & Style Control -->
        <div class="space-y-3">
          <div class="flex items-center justify-between text-xs">
            <span class="text-slate-300 font-medium">Subtitle Style</span>
            <div class="flex items-center gap-1 bg-slate-800 p-0.5 rounded-lg border border-slate-700/80">
              <button
                type="button"
                on:click={() => { borderStyle = 1; onRefresh(fontSize, timestamp, 1); }}
                class="px-2.5 py-1 text-[11px] font-medium rounded-md transition {borderStyle === 1 ? 'bg-sky-600 text-white shadow-sm' : 'text-slate-400 hover:text-slate-200'}"
              >
                Clean Outline
              </button>
              <button
                type="button"
                on:click={() => { borderStyle = 3; onRefresh(fontSize, timestamp, 3); }}
                class="px-2.5 py-1 text-[11px] font-medium rounded-md transition {borderStyle === 3 ? 'bg-sky-600 text-white shadow-sm' : 'text-slate-400 hover:text-slate-200'}"
              >
                Black Box
              </button>
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between text-xs mb-1.5">
              <span class="text-slate-300 font-medium">Subtitle Font Size</span>
              <span class="text-sky-400 font-mono font-semibold">{fontSize} pt</span>
            </div>
            <input
              type="range"
              min="16"
              max="40"
              step="2"
              value={fontSize}
              on:input={handleSliderChange}
              class="w-full accent-sky-500 bg-slate-800 h-1.5 rounded-lg appearance-none cursor-pointer"
            />
            <div class="flex justify-between text-[10px] text-slate-500 mt-1">
              <span>Small (16pt)</span>
              <span>Standard (24pt)</span>
              <span>Large (32pt)</span>
              <span>Huge (40pt)</span>
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
            class="w-full accent-indigo-500 bg-slate-800 h-1.5 rounded-lg appearance-none cursor-pointer"
          />
          <!-- Quick dialogue scene jump buttons -->
          <div class="flex items-center gap-1.5 mt-2">
            <span class="text-[10px] text-slate-500">Jump to:</span>
            <button
              on:click={() => jumpToTime(30)}
              class="px-2 py-0.5 text-[10px] bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-700/80 transition"
            >
              30s
            </button>
            <button
              on:click={() => jumpToTime(60)}
              class="px-2 py-0.5 text-[10px] bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-700/80 transition"
            >
              1m
            </button>
            <button
              on:click={() => jumpToTime(120)}
              class="px-2 py-0.5 text-[10px] bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-700/80 transition"
            >
              2m
            </button>
            <button
              on:click={() => jumpToTime(300)}
              class="px-2 py-0.5 text-[10px] bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-700/80 transition"
            >
              5m
            </button>
            <button
              on:click={() => jumpToTime(600)}
              class="px-2 py-0.5 text-[10px] bg-slate-800 hover:bg-slate-700 text-slate-300 rounded border border-slate-700/80 transition"
            >
              10m
            </button>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center justify-between pt-2 border-t border-slate-800/80">
        <button
          on:click={() => onRefresh(fontSize, timestamp)}
          disabled={isLoading}
          class="px-3.5 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-lg flex items-center gap-1.5 transition active:scale-95 disabled:opacity-50 cursor-pointer"
        >
          <RefreshCw class={`w-3.5 h-3.5 ${isLoading ? "animate-spin" : ""}`} />
          <span>Update Preview</span>
        </button>

        <div class="flex items-center gap-2">
          <button
            on:click={onClose}
            class="px-3.5 py-1.5 text-xs font-medium text-slate-400 hover:text-slate-200 transition cursor-pointer"
          >
            Cancel
          </button>
          <button
            on:click={handleApply}
            class="px-4 py-1.5 text-xs font-semibold bg-sky-600 hover:bg-sky-500 text-white rounded-lg shadow-md shadow-sky-600/20 transition active:scale-95 cursor-pointer"
          >
            Apply & Save Size
          </button>
        </div>
      </div>
    </div>
  </div>
</div>

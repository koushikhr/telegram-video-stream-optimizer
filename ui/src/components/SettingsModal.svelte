<script lang="ts">
  import type { OptimizationSettings, HardwareCapabilities } from "../types";
  import { X, Settings, Sparkles, Cpu, HardDrive, Subtitles, Volume2 } from "lucide-svelte";
  import { POPULAR_LANGUAGES } from "../lib/languages";

  export let settings: OptimizationSettings;
  export let hwCaps: HardwareCapabilities | null = null;
  export let onClose: () => void;
  export let onSave: (newSettings: OptimizationSettings) => void;

  let localSettings: OptimizationSettings = { ...settings };
  let customSizeInput = localSettings.target_size_mb;

  function handleSave() {
    localSettings.target_size_mb = customSizeInput;
    onSave(localSettings);
    onClose();
  }
</script>

<div class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-md flex items-center justify-center p-6 animate-in fade-in duration-200">
  <div class="bg-slate-900 border border-slate-800 rounded-2xl w-full max-w-lg max-h-[90vh] flex flex-col shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-6 py-4 border-b border-slate-800 flex items-center justify-between shrink-0">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-slate-800 border border-slate-700 flex items-center justify-center text-slate-300">
          <Settings class="w-4 h-4" />
        </div>
        <div>
          <h3 class="font-semibold text-sm text-slate-100">Preferences & Defaults</h3>
          <p class="text-xs text-slate-400">Configure global auto-suggestion and limits</p>
        </div>
      </div>
      <button
        on:click={onClose}
        class="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition"
      >
        <X class="w-4 h-4" />
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-y-auto p-6 space-y-5 text-xs">
      <!-- Preferred Audio Language -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1.5 flex items-center gap-1.5">
          <Volume2 class="w-3.5 h-3.5 text-sky-400" />
          <span>Preferred Audio Language</span>
        </label>
        <p class="text-[11px] text-slate-400 mb-2">
          Automatically select this dialogue audio track when adding episodes.
        </p>
        <select
          bind:value={localSettings.preferred_audio_lang}
          class="w-full bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200 focus:outline-none focus:border-sky-500"
        >
          {#each POPULAR_LANGUAGES as lang}
            <option value={lang.code}>{lang.name} ({lang.nativeName})</option>
          {/each}
        </select>
      </div>

      <!-- Preferred Subtitle Language -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1.5 flex items-center gap-1.5">
          <Subtitles class="w-3.5 h-3.5 text-indigo-400" />
          <span>Preferred Subtitle Language</span>
        </label>
        <p class="text-[11px] text-slate-400 mb-2">
          Automatically select and burn this subtitle dialogue track.
        </p>
        <select
          bind:value={localSettings.preferred_subtitle_lang}
          class="w-full bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200 focus:outline-none focus:border-indigo-500"
        >
          {#each POPULAR_LANGUAGES as lang}
            <option value={lang.code}>{lang.name} ({lang.nativeName})</option>
          {/each}
        </select>
      </div>

      <!-- Target Size Limit -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1.5 flex items-center gap-1.5">
          <HardDrive class="w-3.5 h-3.5 text-emerald-400" />
          <span>Strict Telegram Upload Size Cap</span>
        </label>
        <div class="grid grid-cols-2 gap-2 mb-2">
          <button
            type="button"
            on:click={() => (customSizeInput = 1980)}
            class={`p-2.5 rounded-xl border text-left transition cursor-pointer ${
              customSizeInput === 1980
                ? "bg-emerald-500/10 border-emerald-500/30 text-emerald-400"
                : "bg-slate-800/60 hover:bg-slate-800 border-slate-700 text-slate-300"
            }`}
          >
            <div class="font-bold text-xs">Telegram Free</div>
            <div class="text-[10px] opacity-75">1,980 MB (Guaranteed safe)</div>
          </button>

          <button
            type="button"
            on:click={() => (customSizeInput = 3980)}
            class={`p-2.5 rounded-xl border text-left transition cursor-pointer ${
              customSizeInput === 3980
                ? "bg-purple-500/10 border-purple-500/30 text-purple-400"
                : "bg-slate-800/60 hover:bg-slate-800 border-slate-700 text-slate-300"
            }`}
          >
            <div class="font-bold text-xs">Telegram Premium</div>
            <div class="text-[10px] opacity-75">3,980 MB (4 GB limit)</div>
          </button>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-slate-400 text-[11px]">Custom Cap (MB):</span>
          <input
            type="number"
            min="100"
            max="4000"
            bind:value={customSizeInput}
            class="bg-slate-800 border border-slate-700 rounded-lg px-2.5 py-1 text-slate-200 font-mono text-xs w-28 focus:outline-none focus:border-sky-500"
          />
        </div>
      </div>

      <!-- Default Subtitle Font Size -->
      <div>
        <div class="flex items-center justify-between mb-1.5">
          <label class="text-slate-300 font-semibold">Default Subtitle Size</label>
          <span class="text-sky-400 font-mono font-semibold">{localSettings.subtitle_font_size} pt</span>
        </div>
        <input
          type="range"
          min="16"
          max="38"
          step="2"
          bind:value={localSettings.subtitle_font_size}
          class="w-full accent-sky-500 bg-slate-800 h-1.5 rounded-lg appearance-none cursor-pointer"
        />
        <div class="flex justify-between text-[10px] text-slate-500 mt-1">
          <span>Small (16)</span>
          <span>Medium (24)</span>
          <span>Large (32)</span>
        </div>
      </div>

      <!-- Hardware Acceleration -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1.5 flex items-center gap-1.5">
          <Cpu class="w-3.5 h-3.5 text-amber-400" />
          <span>Hardware Video Encoder</span>
        </label>
        <select
          bind:value={localSettings.hardware_encoder}
          class="w-full bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200 focus:outline-none focus:border-amber-500"
        >
          <option value="Auto">Auto (Fastest Hardware Acceleration)</option>
          {#if hwCaps?.has_nvidia_nvenc}
            <option value="NvidiaNvenc">NVIDIA NVENC (GeForce RTX / GTX)</option>
          {/if}
          {#if hwCaps?.has_amd_amf}
            <option value="AmdAmf">AMD AMF (Radeon Graphics)</option>
          {/if}
          {#if hwCaps?.has_intel_qsv}
            <option value="IntelQsv">Intel QuickSync (QSV)</option>
          {/if}
          <option value="CpuX264">Software CPU (libx264 - Maximum Density)</option>
        </select>
      </div>
    </div>

    <!-- Footer -->
    <div class="px-6 py-4 border-t border-slate-800 bg-slate-900/90 flex items-center justify-end gap-2 shrink-0">
      <button
        on:click={onClose}
        class="px-3.5 py-1.5 text-xs font-medium text-slate-400 hover:text-slate-200 transition cursor-pointer"
      >
        Cancel
      </button>
      <button
        on:click={handleSave}
        class="px-4 py-1.5 text-xs font-semibold bg-sky-600 hover:bg-sky-500 text-white rounded-lg shadow-md shadow-sky-600/20 transition active:scale-95 cursor-pointer"
      >
        Save Preferences
      </button>
    </div>
  </div>
</div>

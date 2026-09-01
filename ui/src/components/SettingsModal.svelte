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

<div class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm flex items-center justify-center p-6 animate-in fade-in duration-150">
  <div class="bg-[#16181d] border border-[#2b2e38] rounded-xl w-full max-w-lg max-h-[90vh] flex flex-col shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-5 py-3.5 border-b border-[#23262f] bg-[#121418] flex items-center justify-between shrink-0">
      <div class="flex items-center gap-2.5">
        <div class="w-7 h-7 rounded-lg bg-[#1e222b] border border-[#2c303c] flex items-center justify-center text-slate-300">
          <Settings class="w-3.5 h-3.5 text-[#3897e0]" />
        </div>
        <div>
          <h3 class="font-semibold text-xs uppercase tracking-wide text-slate-100">Preferences & Defaults</h3>
          <p class="text-[11px] text-slate-400 mt-0.5">Configure global auto-suggestion and size caps</p>
        </div>
      </div>
      <button
        type="button"
        on:click={onClose}
        class="p-1.5 text-slate-400 hover:text-slate-200 hover:bg-[#20232b] rounded-md transition cursor-pointer"
      >
        <X class="w-4 h-4" />
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-y-auto p-5 space-y-4 text-xs">
      <!-- Preferred Audio Language -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1 flex items-center gap-1.5">
          <Volume2 class="w-3.5 h-3.5 text-[#3897e0]" />
          <span>Preferred Audio Language</span>
        </label>
        <p class="text-[11px] text-slate-400 mb-2">
          Automatically select this dialogue audio track when adding episodes.
        </p>
        <select
          bind:value={localSettings.preferred_audio_lang}
          class="w-full bg-[#1b1e26] border border-[#2c303c] rounded-md px-3 py-2 text-slate-200 focus:outline-none focus:border-[#2481cc]"
        >
          {#each POPULAR_LANGUAGES as lang}
            <option value={lang.code}>{lang.name} ({lang.nativeName})</option>
          {/each}
        </select>
      </div>

      <!-- Preferred Subtitle Language -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1 flex items-center gap-1.5">
          <Subtitles class="w-3.5 h-3.5 text-[#3897e0]" />
          <span>Preferred Subtitle Language</span>
        </label>
        <p class="text-[11px] text-slate-400 mb-2">
          Automatically select and burn this subtitle dialogue track.
        </p>
        <select
          bind:value={localSettings.preferred_subtitle_lang}
          class="w-full bg-[#1b1e26] border border-[#2c303c] rounded-md px-3 py-2 text-slate-200 focus:outline-none focus:border-[#2481cc]"
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
            class={`p-2.5 rounded-lg border text-left transition cursor-pointer ${
              customSizeInput === 1980
                ? "bg-emerald-500/10 border-emerald-500/30 text-emerald-300"
                : "bg-[#1b1e26] hover:bg-[#222630] border-[#2c303c] text-slate-300"
            }`}
          >
            <div class="font-semibold text-xs">Telegram Free</div>
            <div class="text-[10px] text-slate-400 mt-0.5">1,980 MB (Guaranteed safe)</div>
          </button>

          <button
            type="button"
            on:click={() => (customSizeInput = 3980)}
            class={`p-2.5 rounded-lg border text-left transition cursor-pointer ${
              customSizeInput === 3980
                ? "bg-purple-500/10 border-purple-500/30 text-purple-300"
                : "bg-[#1b1e26] hover:bg-[#222630] border-[#2c303c] text-slate-300"
            }`}
          >
            <div class="font-semibold text-xs">Telegram Premium</div>
            <div class="text-[10px] text-slate-400 mt-0.5">3,980 MB (4 GB limit)</div>
          </button>
        </div>

        <div class="flex items-center gap-2">
          <span class="text-slate-400 text-[11px]">Custom Cap (MB):</span>
          <input
            type="number"
            min="100"
            max="4000"
            bind:value={customSizeInput}
            class="bg-[#1b1e26] border border-[#2c303c] rounded-md px-2.5 py-1 text-slate-200 font-mono text-xs w-28 focus:outline-none focus:border-[#2481cc]"
          />
        </div>
      </div>

      <!-- Default Subtitle Font Size -->
      <div>
        <div class="flex items-center justify-between mb-1.5">
          <label class="text-slate-300 font-semibold" for="settings-subtitle-size">Default Subtitle Size</label>
          <span class="text-[#3897e0] font-mono font-semibold">{localSettings.subtitle_font_size} pt</span>
        </div>
        <input
          id="settings-subtitle-size"
          type="range"
          min="8"
          max="36"
          step="1"
          bind:value={localSettings.subtitle_font_size}
          class="w-full accent-[#2481cc] bg-[#222530] h-1.5 rounded-lg appearance-none cursor-pointer"
        />
        <div class="flex justify-between text-[10px] text-slate-500 mt-1 font-mono">
          <span>8pt (Mini)</span>
          <span>12pt</span>
          <span>16pt</span>
          <span>22pt</span>
          <span>36pt (Max)</span>
        </div>
      </div>

      <!-- Hardware Acceleration -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1.5 flex items-center gap-1.5">
          <Cpu class="w-3.5 h-3.5 text-amber-400/90" />
          <span>Hardware Video Encoder</span>
        </label>
        <select
          bind:value={localSettings.hardware_encoder}
          class="w-full bg-[#1b1e26] border border-[#2c303c] rounded-md px-3 py-2 text-slate-200 focus:outline-none focus:border-[#2481cc]"
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
          <option value="CpuX264">Software CPU (libx264 - Slower)</option>
        </select>
      </div>
    </div>

    <!-- Footer -->
    <div class="px-5 py-3 border-t border-[#23262f] bg-[#121418] flex items-center justify-end gap-2 shrink-0">
      <button
        type="button"
        on:click={onClose}
        class="px-3.5 py-1.5 text-xs font-medium text-slate-400 hover:text-slate-200 transition cursor-pointer"
      >
        Cancel
      </button>
      <button
        type="button"
        on:click={handleSave}
        class="px-4 py-1.5 text-xs font-medium bg-[#2481cc] hover:bg-[#2075b8] text-white rounded-md transition active:scale-95 cursor-pointer shadow-sm"
      >
        Save Preferences
      </button>
    </div>
  </div>
</div>

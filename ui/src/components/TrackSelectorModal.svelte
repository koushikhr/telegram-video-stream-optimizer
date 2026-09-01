<script lang="ts">
  import type { QueueItem } from "../types";
  import { X, Volume2, Subtitles, Check, AlertCircle } from "lucide-svelte";
  import { getLanguageName } from "../lib/languages";

  export let item: QueueItem;
  export let onClose: () => void;
  export let onSave: (audioTrackIndex: number, subTrackIndex: number | undefined) => void;

  let selectedAudio = item.selected_audio_track_index ?? 0;
  let selectedSub = item.selected_subtitle_track_index;

  function handleSave() {
    onSave(selectedAudio, selectedSub);
    onClose();
  }
</script>

<div class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-md flex items-center justify-center p-6 animate-in fade-in duration-200">
  <div class="bg-slate-900 border border-slate-800 rounded-2xl w-full max-w-xl max-h-[85vh] flex flex-col shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-6 py-4 border-b border-slate-800 flex items-center justify-between shrink-0">
      <div>
        <h3 class="font-semibold text-sm text-slate-100">Select Audio & Subtitle Tracks</h3>
        <p class="text-xs text-slate-400 truncate max-w-md">{item.file_name}</p>
      </div>
      <button
        on:click={onClose}
        class="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition"
      >
        <X class="w-4 h-4" />
      </button>
    </div>

    <!-- Body -->
    <div class="flex-1 overflow-y-auto p-6 space-y-6">
      <!-- Audio Track Section -->
      <div>
        <div class="flex items-center gap-2 mb-3">
          <Volume2 class="w-4 h-4 text-sky-400" />
          <h4 class="text-xs font-semibold uppercase tracking-wider text-slate-300">
            Audio Dialogue Track
          </h4>
        </div>

        {#if !item.probe?.audio_tracks.length}
          <p class="text-xs text-slate-500 italic">No audio tracks detected</p>
        {:else}
          <div class="space-y-2">
            {#each item.probe.audio_tracks as track}
              <button
                type="button"
                on:click={() => (selectedAudio = track.track_index)}
                class={`w-full text-left p-3 rounded-xl border text-xs transition flex items-center justify-between cursor-pointer ${
                  selectedAudio === track.track_index
                    ? "bg-sky-500/10 border-sky-500/30 text-slate-100"
                    : "bg-slate-800/50 hover:bg-slate-800 border-slate-700/60 text-slate-300"
                }`}
              >
                <div>
                  <div class="flex items-center gap-2 font-medium">
                    <span>{getLanguageName(track.language)}</span>
                    <span class="text-slate-500 font-mono">({track.codec_name.toUpperCase()})</span>
                    {#if track.is_default}
                      <span class="px-1.5 py-0.5 rounded text-[10px] bg-slate-700 text-slate-300">Default</span>
                    {/if}
                    {#if track.is_commentary}
                      <span class="px-1.5 py-0.5 rounded text-[10px] bg-amber-500/20 text-amber-300">Commentary</span>
                    {/if}
                  </div>
                  <div class="text-[11px] text-slate-400 mt-0.5">
                    {track.channels} Channels ({track.channel_layout}) {track.title ? `• ${track.title}` : ""}
                  </div>
                </div>

                {#if selectedAudio === track.track_index}
                  <div class="w-5 h-5 rounded-full bg-sky-500 flex items-center justify-center text-white shrink-0">
                    <Check class="w-3 h-3" />
                  </div>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Subtitle Track Section -->
      <div>
        <div class="flex items-center gap-2 mb-3">
          <Subtitles class="w-4 h-4 text-indigo-400" />
          <h4 class="text-xs font-semibold uppercase tracking-wider text-slate-300">
            Subtitle Track (Burn-in)
          </h4>
        </div>

        <div class="space-y-2">
          <!-- Option: No Subtitles -->
          <button
            type="button"
            on:click={() => (selectedSub = undefined)}
            class={`w-full text-left p-3 rounded-xl border text-xs transition flex items-center justify-between cursor-pointer ${
              selectedSub === undefined
                ? "bg-indigo-500/10 border-indigo-500/30 text-slate-100"
                : "bg-slate-800/50 hover:bg-slate-800 border-slate-700/60 text-slate-400"
            }`}
          >
            <div>
              <div class="font-medium text-slate-200">No Subtitles (Pure Video)</div>
              <div class="text-[11px] text-slate-500">Do not burn any subtitles into the stream</div>
            </div>
            {#if selectedSub === undefined}
              <div class="w-5 h-5 rounded-full bg-indigo-500 flex items-center justify-center text-white shrink-0">
                <Check class="w-3 h-3" />
              </div>
            {/if}
          </button>

          <!-- List detected subtitle tracks -->
          {#if item.probe?.subtitle_tracks}
            {#each item.probe.subtitle_tracks as track}
              <button
                type="button"
                on:click={() => (selectedSub = track.track_index)}
                class={`w-full text-left p-3 rounded-xl border text-xs transition flex items-center justify-between cursor-pointer ${
                  selectedSub === track.track_index
                    ? "bg-indigo-500/10 border-indigo-500/30 text-slate-100"
                    : "bg-slate-800/50 hover:bg-slate-800 border-slate-700/60 text-slate-300"
                }`}
              >
                <div>
                  <div class="flex items-center gap-2 font-medium">
                    <span>{getLanguageName(track.language)}</span>
                    <span class="text-slate-500 font-mono">({track.codec_name.toUpperCase()})</span>
                    {#if track.is_external}
                      <span class="px-1.5 py-0.5 rounded text-[10px] bg-emerald-500/20 text-emerald-300">External File</span>
                    {/if}
                    {#if track.is_hearing_impaired}
                      <span class="px-1.5 py-0.5 rounded text-[10px] bg-slate-700 text-slate-300">SDH</span>
                    {/if}
                    {#if track.is_forced}
                      <span class="px-1.5 py-0.5 rounded text-[10px] bg-purple-500/20 text-purple-300">Forced Signs</span>
                    {/if}
                  </div>
                  {#if track.title}
                    <div class="text-[11px] text-slate-400 mt-0.5 truncate max-w-sm">
                      {track.title}
                    </div>
                  {/if}
                </div>

                {#if selectedSub === track.track_index}
                  <div class="w-5 h-5 rounded-full bg-indigo-500 flex items-center justify-center text-white shrink-0">
                    <Check class="w-3 h-3" />
                  </div>
                {/if}
              </button>
            {/each}
          {/if}
        </div>
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
        Save Changes
      </button>
    </div>
  </div>
</div>

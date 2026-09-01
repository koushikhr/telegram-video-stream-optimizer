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

<div class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm flex items-center justify-center p-6 animate-in fade-in duration-150">
  <div class="bg-[#16181d] border border-[#2b2e38] rounded-xl w-full max-w-xl max-h-[85vh] flex flex-col shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-5 py-3.5 border-b border-[#23262f] bg-[#121418] flex items-center justify-between shrink-0">
      <div>
        <h3 class="font-semibold text-xs uppercase tracking-wide text-slate-100">Select Audio & Subtitle Tracks</h3>
        <p class="text-[11px] text-slate-400 truncate max-w-md mt-0.5">{item.file_name}</p>
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
    <div class="flex-1 overflow-y-auto p-5 space-y-5">
      <!-- Audio Track Section -->
      <div>
        <div class="flex items-center gap-2 mb-2.5">
          <Volume2 class="w-3.5 h-3.5 text-[#3897e0]" />
          <h4 class="text-xs font-semibold uppercase tracking-wider text-slate-300">
            Audio Dialogue Track
          </h4>
        </div>

        {#if !item.probe?.audio_tracks.length}
          <p class="text-xs text-slate-500 italic">No audio tracks detected</p>
        {:else}
          <div class="space-y-1.5">
            {#each item.probe.audio_tracks as track}
              <button
                type="button"
                on:click={() => (selectedAudio = track.track_index)}
                class={`w-full text-left p-2.5 rounded-lg border text-xs transition flex items-center justify-between cursor-pointer ${
                  selectedAudio === track.track_index
                    ? "bg-[#2481cc]/15 border-[#2481cc]/40 text-slate-100"
                    : "bg-[#1b1e26] hover:bg-[#222630] border-[#2c303c] text-slate-300"
                }`}
              >
                <div>
                  <div class="flex items-center gap-2 font-medium">
                    <span>{getLanguageName(track.language)}</span>
                    <span class="text-slate-500 font-mono text-[11px]">({track.codec_name.toUpperCase()})</span>
                    {#if track.is_default}
                      <span class="px-1.5 py-0.2 rounded text-[10px] bg-[#272b36] text-slate-300">Default</span>
                    {/if}
                    {#if track.is_commentary}
                      <span class="px-1.5 py-0.2 rounded text-[10px] bg-amber-500/20 text-amber-300">Commentary</span>
                    {/if}
                  </div>
                  <div class="text-[11px] text-slate-400 mt-0.5 font-mono">
                    {track.channels} Channels ({track.channel_layout}) {track.title ? `• ${track.title}` : ""}
                  </div>
                </div>

                {#if selectedAudio === track.track_index}
                  <div class="w-4 h-4 rounded-full bg-[#2481cc] flex items-center justify-center text-white shrink-0">
                    <Check class="w-2.5 h-2.5" />
                  </div>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Subtitle Track Section -->
      <div>
        <div class="flex items-center gap-2 mb-2.5">
          <Subtitles class="w-3.5 h-3.5 text-[#3897e0]" />
          <h4 class="text-xs font-semibold uppercase tracking-wider text-slate-300">
            Subtitle Track (Burn-in)
          </h4>
        </div>

        <div class="space-y-1.5">
          <!-- Option: No Subtitles -->
          <button
            type="button"
            on:click={() => (selectedSub = undefined)}
            class={`w-full text-left p-2.5 rounded-lg border text-xs transition flex items-center justify-between cursor-pointer ${
              selectedSub === undefined
                ? "bg-[#2481cc]/15 border-[#2481cc]/40 text-slate-100"
                : "bg-[#1b1e26] hover:bg-[#222630] border-[#2c303c] text-slate-400"
            }`}
          >
            <div>
              <div class="font-medium text-slate-200">No Subtitles (Pure Video)</div>
              <div class="text-[11px] text-slate-500">Do not burn any subtitles into the stream</div>
            </div>
            {#if selectedSub === undefined}
              <div class="w-4 h-4 rounded-full bg-[#2481cc] flex items-center justify-center text-white shrink-0">
                <Check class="w-2.5 h-2.5" />
              </div>
            {/if}
          </button>

          <!-- List detected subtitle tracks -->
          {#if item.probe?.subtitle_tracks}
            {#each item.probe.subtitle_tracks as track}
              <button
                type="button"
                on:click={() => (selectedSub = track.track_index)}
                class={`w-full text-left p-2.5 rounded-lg border text-xs transition flex items-center justify-between cursor-pointer ${
                  selectedSub === track.track_index
                    ? "bg-[#2481cc]/15 border-[#2481cc]/40 text-slate-100"
                    : "bg-[#1b1e26] hover:bg-[#222630] border-[#2c303c] text-slate-300"
                }`}
              >
                <div>
                  <div class="flex items-center gap-2 font-medium">
                    <span>{getLanguageName(track.language)}</span>
                    <span class="text-slate-500 font-mono text-[11px]">({track.codec_name.toUpperCase()})</span>
                    {#if track.is_external}
                      <span class="px-1.5 py-0.2 rounded text-[10px] bg-emerald-500/20 text-emerald-300">External File</span>
                    {/if}
                    {#if track.is_hearing_impaired}
                      <span class="px-1.5 py-0.2 rounded text-[10px] bg-[#272b36] text-slate-300">SDH</span>
                    {/if}
                    {#if track.is_forced}
                      <span class="px-1.5 py-0.2 rounded text-[10px] bg-purple-500/20 text-purple-300">Forced Signs</span>
                    {/if}
                  </div>
                  {#if track.title}
                    <div class="text-[11px] text-slate-400 mt-0.5 truncate max-w-sm">
                      {track.title}
                    </div>
                  {/if}
                </div>

                {#if selectedSub === track.track_index}
                  <div class="w-4 h-4 rounded-full bg-[#2481cc] flex items-center justify-center text-white shrink-0">
                    <Check class="w-2.5 h-2.5" />
                  </div>
                {/if}
              </button>
            {/each}
          {/if}
        </div>
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
        Save Changes
      </button>
    </div>
  </div>
</div>

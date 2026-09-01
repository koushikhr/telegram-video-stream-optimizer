<script lang="ts">
  import { Play, Pause, Square, CheckCircle, Clock } from "lucide-svelte";

  export let totalCount: number = 0;
  export let completedCount: number = 0;
  export let isRunning: boolean = false;
  export let onStart: () => void;
  export let onPause: () => void;
  export let onCancel: () => void;
</script>

<footer class="h-14 border-t border-[#23262f] bg-[#121418] px-5 flex items-center justify-between shrink-0 select-none">
  <div class="flex items-center gap-3 text-xs text-slate-400">
    <div class="flex items-center gap-1.5 font-mono text-[11px]">
      <span class="font-semibold text-slate-200">{totalCount}</span>
      <span>{totalCount === 1 ? "video" : "videos"} queued</span>
    </div>
    {#if completedCount > 0}
      <span class="text-slate-600">•</span>
      <div class="flex items-center gap-1.5 text-emerald-400 font-medium font-mono text-[11px]">
        <CheckCircle class="w-3.5 h-3.5" />
        <span>{completedCount} completed</span>
      </div>
    {/if}
  </div>

  <div class="flex items-center gap-2.5">
    {#if isRunning}
      <button
        on:click={onCancel}
        class="px-3 py-1.5 text-xs font-medium bg-rose-500/10 hover:bg-rose-500/20 text-rose-300 border border-rose-500/30 rounded-md flex items-center gap-1.5 transition active:scale-95 cursor-pointer"
        title="Stop all encoding and clean temp files"
      >
        <Square class="w-3.5 h-3.5 fill-current" />
        <span>Stop</span>
      </button>

      <button
        on:click={onPause}
        class="px-3.5 py-1.5 text-xs font-medium bg-amber-600 hover:bg-amber-500 text-white rounded-md flex items-center gap-1.5 transition active:scale-95 cursor-pointer"
      >
        <Pause class="w-3.5 h-3.5 fill-current" />
        <span>Pause</span>
      </button>
    {:else}
      <button
        on:click={onStart}
        disabled={totalCount === 0 || completedCount === totalCount}
        class="px-4 py-1.5 text-xs font-medium bg-[#2481cc] hover:bg-[#2075b8] text-white rounded-md flex items-center gap-2 transition active:scale-95 disabled:opacity-40 disabled:pointer-events-none cursor-pointer shadow-sm"
      >
        <Play class="w-3.5 h-3.5 fill-current" />
        <span>Start Queue</span>
      </button>
    {/if}
  </div>
</footer>

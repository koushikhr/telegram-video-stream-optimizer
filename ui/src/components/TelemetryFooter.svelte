<script lang="ts">
  import { Play, Pause, Square, CheckCircle, Clock } from "lucide-svelte";

  export let totalCount: number = 0;
  export let completedCount: number = 0;
  export let isRunning: boolean = false;
  export let onStart: () => void;
  export let onPause: () => void;
  export let onCancel: () => void;
</script>

<footer class="h-16 border-t border-slate-800 bg-slate-900/90 backdrop-blur px-6 flex items-center justify-between shrink-0 select-none">
  <div class="flex items-center gap-4 text-xs text-slate-400">
    <div class="flex items-center gap-1.5">
      <span class="font-medium text-slate-200">{totalCount}</span>
      <span>{totalCount === 1 ? "video" : "videos"} in queue</span>
    </div>
    {#if completedCount > 0}
      <span class="text-slate-600">•</span>
      <div class="flex items-center gap-1.5 text-emerald-400 font-medium">
        <CheckCircle class="w-3.5 h-3.5" />
        <span>{completedCount} completed</span>
      </div>
    {/if}
  </div>

  <div class="flex items-center gap-3">
    {#if isRunning}
      <button
        on:click={onCancel}
        class="px-3.5 py-1.5 text-xs font-semibold bg-rose-500/10 hover:bg-rose-500/20 text-rose-300 border border-rose-500/30 rounded-xl flex items-center gap-1.5 transition active:scale-95 cursor-pointer"
        title="Stop all encoding and clean temp files"
      >
        <Square class="w-3.5 h-3.5 fill-current" />
        <span>Stop Queue</span>
      </button>

      <button
        on:click={onPause}
        class="px-4 py-2 text-xs font-semibold bg-amber-600 hover:bg-amber-500 text-white rounded-xl shadow-lg shadow-amber-600/25 flex items-center gap-1.5 transition active:scale-95 cursor-pointer"
      >
        <Pause class="w-3.5 h-3.5 fill-current" />
        <span>Pause</span>
      </button>
    {:else}
      <button
        on:click={onStart}
        disabled={totalCount === 0 || completedCount === totalCount}
        class="px-5 py-2 text-xs font-semibold bg-gradient-to-r from-sky-500 to-blue-600 hover:from-sky-400 hover:to-blue-500 text-white rounded-xl shadow-lg shadow-sky-500/25 flex items-center gap-2 transition active:scale-95 disabled:opacity-40 disabled:pointer-events-none cursor-pointer"
      >
        <Play class="w-4 h-4 fill-current" />
        <span>Start Optimization Queue</span>
      </button>
    {/if}
  </div>
</footer>

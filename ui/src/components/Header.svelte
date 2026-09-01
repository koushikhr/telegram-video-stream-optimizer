<script lang="ts">
  import { Settings, Moon, FolderOpen, Plus, Film, Cpu } from "lucide-svelte";

  export let onAddFiles: () => void;
  export let onAddFolder: () => void;
  export let onOpenSettings: () => void;
  export let onOpenPower: () => void;
  export let gpuName: string = "Detecting GPU...";
  export let sleepOnFinish: boolean = false;
  export let batchLimit: number = 0;
</script>

<header class="h-14 border-b border-[#23262f] bg-[#121418] px-5 flex items-center justify-between shrink-0 select-none">
  <div class="flex items-center gap-3">
    <div class="w-8 h-8 rounded-lg bg-[#2481cc]/15 border border-[#2481cc]/30 flex items-center justify-center text-[#3897e0] shadow-sm">
      <Film class="w-4 h-4" />
    </div>
    <div>
      <div class="flex items-center gap-2">
        <h1 class="font-semibold text-xs tracking-wide uppercase text-slate-100">Telegram Video Stream Optimizer</h1>
        <span class="px-1.5 py-0.5 text-[10px] font-mono font-medium bg-[#1e222b] text-slate-300 border border-[#2c303c] rounded">
          FastStart MP4
        </span>
      </div>
      <p class="text-[11px] text-slate-400 mt-0.5">
        Universal Video Transcoder & In-Stream Streamer
      </p>
    </div>
  </div>

  <div class="flex items-center gap-2">
    <!-- Top Right GPU Status Badge -->
    <div class="px-2.5 py-1 text-[11px] font-mono font-medium bg-[#1b1e26] border border-[#2c303c] rounded-md text-slate-300 flex items-center gap-2 shadow-sm mr-1">
      <span class="inline-block w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
      <span class="text-slate-200">{gpuName}</span>
    </div>

    <button
      on:click={onAddFiles}
      class="px-3 py-1.5 text-xs font-medium bg-[#1b1e26] hover:bg-[#232732] text-slate-200 border border-[#2c303c] rounded-md flex items-center gap-1.5 transition active:scale-95 cursor-pointer"
      title="Add video files"
    >
      <Plus class="w-3.5 h-3.5 text-[#3897e0]" />
      <span>Add Videos</span>
    </button>

    <button
      on:click={onAddFolder}
      class="px-3 py-1.5 text-xs font-medium bg-[#1b1e26] hover:bg-[#232732] text-slate-200 border border-[#2c303c] rounded-md flex items-center gap-1.5 transition active:scale-95 cursor-pointer"
      title="Scan entire folder recursively"
    >
      <FolderOpen class="w-3.5 h-3.5 text-amber-400/90" />
      <span>Add Folder</span>
    </button>

    <div class="h-4 w-px bg-[#262933] mx-1"></div>

    <button
      on:click={onOpenPower}
      class={`px-3 py-1.5 text-xs font-medium border rounded-md flex items-center gap-1.5 transition active:scale-95 cursor-pointer ${
        sleepOnFinish || batchLimit > 0
          ? "bg-amber-500/10 border-amber-500/30 text-amber-400"
          : "bg-[#1b1e26] hover:bg-[#232732] text-slate-300 border-[#2c303c]"
      }`}
      title="Power & batch limiting"
    >
      <Moon class="w-3.5 h-3.5" />
      <span>
        {#if sleepOnFinish}
          Sleep Active
        {:else if batchLimit > 0}
          Limit: {batchLimit}
        {:else}
          Power
        {/if}
      </span>
    </button>

    <button
      on:click={onOpenSettings}
      class="p-1.5 text-slate-400 hover:text-slate-100 hover:bg-[#232732] rounded-md transition border border-transparent hover:border-[#2c303c] active:scale-95 cursor-pointer"
      title="Preferences (Languages, Target Size, Hardware)"
    >
      <Settings class="w-4 h-4" />
    </button>
  </div>
</header>

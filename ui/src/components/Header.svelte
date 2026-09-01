<script lang="ts">
  import { Settings, Moon, Sliders, Sparkles, FolderOpen, Plus } from "lucide-svelte";

  export let onAddFiles: () => void;
  export let onAddFolder: () => void;
  export let onOpenSettings: () => void;
  export let onOpenPower: () => void;
  export let gpuName: string = "Detecting GPU...";
  export let sleepOnFinish: boolean = false;
  export let batchLimit: number = 0;
</script>

<header class="h-16 border-b border-slate-800 bg-slate-900/80 backdrop-blur px-6 flex items-center justify-between shrink-0 select-none">
  <div class="flex items-center gap-3">
    <div class="w-9 h-9 rounded-xl bg-gradient-to-tr from-sky-500 to-blue-600 flex items-center justify-center shadow-lg shadow-sky-500/20">
      <Sparkles class="w-5 h-5 text-white" />
    </div>
    <div>
      <div class="flex items-center gap-2">
        <h1 class="font-bold text-sm tracking-tight text-white">Telegram Stream Optimizer</h1>
        <span class="px-1.5 py-0.5 text-[10px] font-semibold bg-sky-500/10 text-sky-400 border border-sky-500/20 rounded-md">
          FastStart MP4
        </span>
      </div>
      <p class="text-xs text-slate-400 flex items-center gap-1.5">
        <span class="inline-block w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
        <span>{gpuName}</span>
      </p>
    </div>
  </div>

  <div class="flex items-center gap-2.5">
    <button
      on:click={onAddFiles}
      class="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-lg flex items-center gap-1.5 transition active:scale-95 shadow-sm cursor-pointer"
      title="Add video files"
    >
      <Plus class="w-3.5 h-3.5 text-sky-400" />
      <span>Add Videos</span>
    </button>

    <button
      on:click={onAddFolder}
      class="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-lg flex items-center gap-1.5 transition active:scale-95 shadow-sm cursor-pointer"
      title="Scan entire folder recursively"
    >
      <FolderOpen class="w-3.5 h-3.5 text-amber-400" />
      <span>Add Folder</span>
    </button>

    <div class="h-4 w-px bg-slate-800 mx-1"></div>

    <button
      on:click={onOpenPower}
      class={`px-3 py-1.5 text-xs font-medium border rounded-lg flex items-center gap-1.5 transition active:scale-95 shadow-sm cursor-pointer ${
        sleepOnFinish || batchLimit > 0
          ? "bg-amber-500/10 border-amber-500/30 text-amber-400"
          : "bg-slate-800 hover:bg-slate-700 text-slate-300 border-slate-700"
      }`}
      title="Laptop battery and sleep settings"
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
      class="p-2 text-slate-400 hover:text-slate-100 hover:bg-slate-800 rounded-lg transition border border-transparent hover:border-slate-700 active:scale-95 cursor-pointer"
      title="Preferences (Languages, Target Size, Hardware)"
    >
      <Settings class="w-4 h-4" />
    </button>
  </div>
</header>

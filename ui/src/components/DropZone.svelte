<script lang="ts">
  import { Film, UploadCloud, FolderPlus } from "lucide-svelte";

  export let onAddFiles: () => void;
  export let onAddFolder: () => void;
</script>

<div class="flex-1 flex flex-col items-center justify-center p-8 text-center border-2 border-dashed border-slate-800 hover:border-sky-500/50 rounded-2xl m-6 bg-slate-900/30 hover:bg-slate-900/50 transition-all duration-300 group select-none">
  <div class="w-16 h-16 rounded-2xl bg-sky-500/10 border border-sky-500/20 flex items-center justify-center mb-5 group-hover:scale-110 transition-transform duration-300">
    <Film class="w-8 h-8 text-sky-400" />
  </div>

  <h2 class="text-lg font-semibold text-white mb-2">Drop Videos or Folders Here</h2>
  <p class="text-xs text-slate-400 max-w-md mb-6 leading-relaxed">
    Add MKV, MP4, WebM or any episode files. They will be repackaged for instant YouTube-like streaming on Telegram, strictly capped under 1,980 MB, with dialogue subtitles cleanly burned in.
  </p>

  <div class="flex items-center gap-3">
    <button
      on:click={onAddFiles}
      class="px-4 py-2 text-xs font-semibold bg-sky-600 hover:bg-sky-500 text-white rounded-xl shadow-lg shadow-sky-600/25 flex items-center gap-2 transition active:scale-95 cursor-pointer"
    >
      <UploadCloud class="w-4 h-4" />
      <span>Select Video Files</span>
    </button>
    <button
      on:click={onAddFolder}
      class="px-4 py-2 text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-xl flex items-center gap-2 transition active:scale-95 cursor-pointer"
    >
      <FolderPlus class="w-4 h-4 text-amber-400" />
      <span>Select Folder</span>
    </button>
  </div>
</div>

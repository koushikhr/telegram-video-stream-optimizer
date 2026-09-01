<script lang="ts">
  import { Film, UploadCloud, FolderPlus } from "lucide-svelte";

  export let onAddFiles: () => void;
  export let onAddFolder: () => void;
</script>

<div class="flex-1 flex flex-col items-center justify-center p-8 text-center border-2 border-dashed border-[#262a34] hover:border-[#3897e0]/50 rounded-xl m-6 bg-[#13151a]/40 hover:bg-[#13151a]/70 transition-all duration-200 group select-none">
  <div class="w-12 h-12 rounded-xl bg-[#1e222b] border border-[#2c303c] flex items-center justify-center mb-4 text-slate-400 group-hover:text-[#3897e0] transition-colors">
    <Film class="w-6 h-6" />
  </div>

  <h2 class="text-sm font-semibold text-slate-100 uppercase tracking-wide mb-1.5">Drop Videos or Folders Here</h2>
  <p class="text-xs text-slate-400 max-w-md mb-5 leading-relaxed">
    Add MKV, MP4, WebM or episode files to repackage for instant YouTube-like streaming on Telegram, capped under 1,980 MB, with dialogue subtitles burned in.
  </p>

  <div class="flex items-center gap-2.5">
    <button
      type="button"
      on:click={onAddFiles}
      class="px-3.5 py-2 text-xs font-medium bg-[#2481cc] hover:bg-[#2075b8] text-white rounded-md flex items-center gap-2 transition active:scale-95 cursor-pointer shadow-sm"
    >
      <UploadCloud class="w-3.5 h-3.5" />
      <span>Select Video Files</span>
    </button>
    <button
      type="button"
      on:click={onAddFolder}
      class="px-3.5 py-2 text-xs font-medium bg-[#1e222b] hover:bg-[#262b37] text-slate-200 border border-[#2c303c] rounded-md flex items-center gap-2 transition active:scale-95 cursor-pointer"
    >
      <FolderPlus class="w-3.5 h-3.5 text-amber-400/90" />
      <span>Select Folder</span>
    </button>
  </div>
</div>

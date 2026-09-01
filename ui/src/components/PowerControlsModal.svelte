<script lang="ts">
  import { X, Moon, BatteryCharging, Power, CheckCircle, ShieldAlert } from "lucide-svelte";

  export let sleepOnFinish: boolean;
  export let batchLimit: number;
  export let queueLength: number;
  export let onClose: () => void;
  export let onSave: (sleep: boolean, limit: number) => void;

  let localSleep = sleepOnFinish;
  let localLimit = batchLimit;

  function handleSave() {
    onSave(localSleep, localLimit);
    onClose();
  }
</script>

<div class="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-md flex items-center justify-center p-6 animate-in fade-in duration-200">
  <div class="bg-slate-900 border border-slate-800 rounded-2xl w-full max-w-md shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-6 py-4 border-b border-slate-800 flex items-center justify-between shrink-0">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-400">
          <Moon class="w-4 h-4" />
        </div>
        <div>
          <h3 class="font-semibold text-sm text-slate-100">Laptop & Battery Controls</h3>
          <p class="text-xs text-slate-400">Manage queue limits and automated sleep</p>
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
    <div class="p-6 space-y-5 text-xs">
      <!-- Queue Batch Limit -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1.5 flex items-center gap-1.5">
          <BatteryCharging class="w-4 h-4 text-sky-400" />
          <span>Batch Conversion Limit</span>
        </label>
        <p class="text-[11px] text-slate-400 mb-2.5">
          Convert only the next N episodes/videos then safely stop (ideal for battery or before leaving).
        </p>
        <div class="flex items-center gap-3">
          <input
            type="number"
            min="0"
            max={queueLength || 50}
            bind:value={localLimit}
            class="bg-slate-800 border border-slate-700 rounded-xl px-3 py-2 text-slate-200 font-mono text-xs w-24 focus:outline-none focus:border-sky-500"
          />
          <span class="text-slate-400">
            {localLimit > 0 ? `Process next ${localLimit} items only` : "Process all items in queue (no limit)"}
          </span>
        </div>
      </div>

      <!-- Sleep PC on Completion -->
      <div class="pt-3 border-t border-slate-800">
        <label class="flex items-start gap-3 cursor-pointer select-none">
          <input
            type="checkbox"
            bind:checked={localSleep}
            class="mt-1 w-4 h-4 rounded border-slate-700 bg-slate-800 text-amber-500 focus:ring-0 focus:ring-offset-0 cursor-pointer"
          />
          <div>
            <div class="text-slate-200 font-medium">Put PC to Sleep when finished</div>
            <div class="text-[11px] text-slate-400 mt-0.5">
              Automatically suspends your laptop or desktop once all queued or limited conversions finish.
            </div>
          </div>
        </label>
      </div>

      <!-- Safe Abort Note -->
      <div class="p-3 bg-slate-800/60 border border-slate-700/60 rounded-xl flex items-start gap-2.5 text-[11px] text-slate-300">
        <ShieldAlert class="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
        <span>
          <strong>Safe Cleanup Protection:</strong> If you cancel mid-encode or quit, the app terminates background encoding and purges temporary files immediately, leaving zero orphaned corrupt files on disk.
        </span>
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
        class="px-4 py-1.5 text-xs font-semibold bg-amber-600 hover:bg-amber-500 text-white rounded-lg shadow-md shadow-amber-600/20 transition active:scale-95 cursor-pointer"
      >
        Apply Settings
      </button>
    </div>
  </div>
</div>

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

<div class="fixed inset-0 z-50 bg-black/75 backdrop-blur-sm flex items-center justify-center p-6 animate-in fade-in duration-150">
  <div class="bg-[#16181d] border border-[#2b2e38] rounded-xl w-full max-w-md shadow-2xl overflow-hidden">
    <!-- Header -->
    <div class="px-5 py-3.5 border-b border-[#23262f] bg-[#121418] flex items-center justify-between shrink-0">
      <div class="flex items-center gap-2.5">
        <div class="w-7 h-7 rounded-lg bg-amber-500/15 border border-amber-500/30 flex items-center justify-center text-amber-400">
          <Moon class="w-3.5 h-3.5" />
        </div>
        <div>
          <h3 class="font-semibold text-xs uppercase tracking-wide text-slate-100">Laptop & Battery Controls</h3>
          <p class="text-[11px] text-slate-400 mt-0.5">Manage queue limits and automated sleep</p>
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
    <div class="p-5 space-y-4 text-xs">
      <!-- Queue Batch Limit -->
      <div>
        <label class="block text-slate-300 font-semibold mb-1 flex items-center gap-1.5">
          <BatteryCharging class="w-3.5 h-3.5 text-[#3897e0]" />
          <span>Batch Conversion Limit</span>
        </label>
        <p class="text-[11px] text-slate-400 mb-2">
          Convert only the next N episodes/videos then safely stop (ideal for battery or before leaving).
        </p>
        <div class="flex items-center gap-3">
          <input
            type="number"
            min="0"
            max={queueLength || 50}
            bind:value={localLimit}
            class="bg-[#1b1e26] border border-[#2c303c] rounded-md px-3 py-1.5 text-slate-200 font-mono text-xs w-24 focus:outline-none focus:border-[#2481cc]"
          />
          <span class="text-slate-400 text-[11px]">
            {localLimit > 0 ? `Process next ${localLimit} items only` : "Process all items in queue (no limit)"}
          </span>
        </div>
      </div>

      <!-- Sleep PC on Completion -->
      <div class="pt-3 border-t border-[#23262f]">
        <label class="flex items-start gap-3 cursor-pointer select-none">
          <input
            type="checkbox"
            bind:checked={localSleep}
            class="mt-0.5 w-4 h-4 rounded border-[#2c303c] bg-[#1b1e26] text-amber-500 focus:ring-0 cursor-pointer"
          />
          <div>
            <div class="text-slate-200 font-medium">Put PC to Sleep when finished</div>
            <div class="text-[11px] text-slate-400 mt-0.5">
              Automatically suspends your system once all queued or limited conversions finish.
            </div>
          </div>
        </label>
      </div>

      <!-- Safe Abort Note -->
      <div class="p-3 bg-[#13151a] border border-[#23262f] rounded-lg flex items-start gap-2.5 text-[11px] text-slate-400">
        <ShieldAlert class="w-3.5 h-3.5 text-emerald-400 shrink-0 mt-0.5" />
        <span>
          <strong class="text-slate-200">Safe Cleanup Protection:</strong> If cancelled or closed mid-encode, background processes terminate cleanly and purge temp files immediately.
        </span>
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
        class="px-4 py-1.5 text-xs font-medium bg-amber-600 hover:bg-amber-500 text-white rounded-md transition active:scale-95 cursor-pointer shadow-sm"
      >
        Apply Settings
      </button>
    </div>
  </div>
</div>

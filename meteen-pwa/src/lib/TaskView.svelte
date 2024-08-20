<script lang="ts">
  import { Checkbox } from "flowbite-svelte";
  import type { Task } from "../meteen-storage-wasm/meteen_storage_wasm";
  import { getContext } from "svelte";
  import type VaultStorage from "./storage";

  const storage: VaultStorage = getContext("storage");

  export let task: Task;

  let done = task.done;

  function updateDone(): void {
    storage.updateTaskDone(task.task_id, done);
  }
</script>

<div class="flex flex-row items-center gap-4 m-2">
  <div>
    <Checkbox class="w-6 h-6" on:change={updateDone} bind:checked={done} />
  </div>
  <div class="flex flex-col gap-1">
    <span class="text-lg">{task.summary}</span>
    <span class="text-sm text-gray-500">{task.task_id}</span>
  </div>
</div>

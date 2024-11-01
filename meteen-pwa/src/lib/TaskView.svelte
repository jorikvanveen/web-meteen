<script lang="ts">
  import { run } from 'svelte/legacy';

  import { Checkbox } from "flowbite-svelte";
  import type { Task } from "../meteen-storage-wasm/meteen_storage_wasm";
  import { getContext } from "svelte";
  import type VaultStorage from "./storage";
  import { formatDate } from "./time";

  const storage: VaultStorage = getContext("storage");

  interface Props {
    task: Task;
  }

  let { task }: Props = $props();


  let done = $state(task.done);

  function fromUtcMillis(millis: bigint): Date {
    console.log("Creating date from", millis);
    const date = new Date(Number(millis));
    return date;
  }

  function updateDone(): void {
    storage.updateTaskDone(task.task_id, done);
  }
  let deadline = $derived(task.deadline ? fromUtcMillis(task.deadline.utc_millis) : null);
  run(() => {
    console.log("viewing with deadline", task.deadline);
  });
</script>

<div class="flex flex-row items-center gap-4 m-2">
  <div>
    <Checkbox class="w-6 h-6" on:change={updateDone} bind:checked={done} />
  </div>
  <div class="flex flex-col gap-1">
    <span class="text-lg">{task.summary}</span>

    {#if deadline}
      <span class="text-sm text-gray-500">{formatDate(deadline)}</span>
    {/if}
  </div>
</div>

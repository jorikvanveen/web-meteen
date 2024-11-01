<script lang="ts">
  import { run } from 'svelte/legacy';

  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import Storage from "../lib/storage";
  import TaskView from "../lib/TaskView.svelte";
  import {
    GridPlusSolid,
    PlusOutline,
    UserAddSolid,
  } from "flowbite-svelte-icons";
  import { Button } from "flowbite-svelte";
  import CreateTask from "../lib/CreateTask.svelte";

  const storage: Storage = getContext("storage");
  const projects = storage.projects;

  let adding = $state(false);

  let tasks = $derived($projects[0].tasks);
  run(() => {
    console.log(tasks);
  });
</script>

<h1 class="text-4xl font-bold pb-2">Today</h1>

{#each tasks as task, i (task.task_id)}
  <TaskView {task} />
  {#if i < tasks.length - 1}
    <hr />
  {/if}
{/each}

<Button
  on:click={() => (adding = true)}
  pill={true}
  class="p-3 fixed right-4 bottom-20"
  size="xl"
>
  <PlusOutline />
</Button>

{#if adding}
  <CreateTask on:close={() => (adding = false)} />
{/if}

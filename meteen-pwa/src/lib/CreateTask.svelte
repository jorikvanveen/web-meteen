<script lang="ts">
  import {
    Button,
    ButtonGroup,
    FloatingLabelInput,
    Input,
    InputAddon,
    Label,
    Select,
  } from "flowbite-svelte";
  import {
    AngleLeftOutline,
    ArchiveSolid,
    ChevronLeftOutline,
    ClipboardSolid,
    FloppyDiskSolid,
    HourglassOutline,
    UserCircleSolid,
  } from "flowbite-svelte-icons";
  import { createEventDispatcher, getContext, onMount } from "svelte";
  import { fly } from "svelte/transition";
  import type VaultStorage from "./storage";
  import DatePicker from "./DatePicker.svelte";
  import { formatDate, formatTime } from "./time";

  const storage: VaultStorage = getContext("storage");
  const dispatch = createEventDispatcher();

  function save() {
    storage.create_task(taskName);
    history.back();
  }

  function onBack(e: PopStateEvent) {
    if (pickingDeadline || pickingScheduled) return;
    e.preventDefault();
    dispatch("close");
  }

  onMount(() => {
    history.pushState({ page: "adding" }, "", "#adding");
    window.addEventListener("popstate", onBack);

    return () => {
      window.removeEventListener("popstate", onBack);
    };
  });

  let taskName = "";

  let projectList = [
    { name: "Inbox", value: "inbox" },
    { name: "Work", value: "work" },
    { name: "Home", value: "home" },
  ];
  let selectedProject = "inbox";

  let scheduleDate: Date | null = null;
  let scheduleDateHasTime = false;
  $: scheduleDateFormatted =
    (scheduleDate ? formatDate(scheduleDate) : "") +
    " " +
    (scheduleDate && scheduleDateHasTime ? formatTime(scheduleDate) : "");

  let deadlineDate: Date | null = null;
  let deadlineDateHasTime = false;
  $: deadlineDateFormatted =
    (deadlineDate ? formatDate(deadlineDate) : "") +
    " " +
    (deadlineDate && deadlineDateHasTime ? formatTime(deadlineDate) : "");

  let pickingScheduled = false;
  let pickingDeadline = false;
</script>

<div
  transition:fly={{ y: 1000, opacity: 1, duration: 200 }}
  class="w-full h-full fixed left-0 top-0 z-20 bg-white"
>
  <div
    class="h-16 w-full bg-primary-600 flex flex-row justify-between items-center box-border p-4"
  >
    <Button
      size="xl"
      pill={true}
      class="p-2 bg-transparent"
      outline
      color="light"
      on:click={() => history.back()}
    >
      <AngleLeftOutline color="white" />
    </Button>
    <span class="font-bold text-xl text-white">New task</span>
    <Button
      size="xl"
      pill={true}
      class="p-2 bg-transparent"
      on:click={save}
      color="light"
    >
      <FloppyDiskSolid color="white" />
    </Button>
  </div>

  <div class="box-border p-4 flex flex-col gap-4">
    <!-- .name-input styling is done in app.css (increases input text size) -->
    <div class="name-input">
      <FloatingLabelInput
        class="name-input"
        autofocus
        style="standard"
        bind:value={taskName}
      >
        Task name
      </FloatingLabelInput>
    </div>
    <div>
      <Label class="block">Project</Label>
      <ButtonGroup class="w-full">
        <InputAddon class="border-r-0">
          <ArchiveSolid color="black" class="w-4 h-4" />
        </InputAddon>
        <Select
          class="rounded-s-none"
          bind:value={selectedProject}
          items={projectList}
        ></Select>
      </ButtonGroup>
    </div>
    <div class="flex flex-col sm:flex-row mt-4 sm:mt-0 justify-between gap-4">
      <div class="w-full">
        <Label>Deadline</Label>
        <ButtonGroup class="w-full">
          <InputAddon>
            <HourglassOutline />
          </InputAddon>
          <Input
            class="w-full"
            type="reset"
            value={deadlineDateFormatted}
            on:click={() => (pickingDeadline = true)}
          />
        </ButtonGroup>
      </div>
      <div class="w-full">
        <Label>Scheduled</Label>
        <ButtonGroup class="w-full">
          <InputAddon>
            <ClipboardSolid />
          </InputAddon>
          <Input
            class="w-full"
            type="reset"
            value={scheduleDateFormatted}
            on:click={() => (pickingScheduled = true)}
          />
        </ButtonGroup>
      </div>
    </div>
  </div>
</div>
{#if pickingDeadline}
  <DatePicker
    bind:datetime={deadlineDate}
    bind:withTime={deadlineDateHasTime}
    on:close={(e) => {
      e.stopPropagation();
      pickingDeadline = false;
    }}
  />
{/if}
{#if pickingScheduled}
  <DatePicker
    bind:datetime={scheduleDate}
    bind:withTime={scheduleDateHasTime}
    on:close={(e) => {
      e.stopImmediatePropagation();
      e.stopPropagation();
      pickingScheduled = false;
    }}
  />
{/if}

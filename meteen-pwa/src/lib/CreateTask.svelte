<script lang="ts">
  import {
    Button,
    ButtonGroup,
    FloatingLabelInput,
    Input,
    InputAddon,
    Label,
    Radio,
    Select,
  } from "flowbite-svelte";
  import {
    AngleLeftOutline,
    ArchiveSolid,
    ChevronLeftOutline,
    ClipboardSolid,
    FlagSolid,
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
    storage.create_task(
      taskName,
      deadlineDate,
      deadlineDateHasTime,
      scheduleDate,
      scheduleDateHasTime,
      priority,
    );
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

  let priority = 1;
  $: console.log(priority);

  let pickingScheduled = false;
  let pickingDeadline = false;
</script>

<div
  transition:fly={{ y: 1000, opacity: 1, duration: 200 }}
  class="w-full h-full fixed left-0 top-0 z-20 bg-white"
>
  <div class="h-16 w-full bg-primary-600">
    <div
      class="flex flex-row justify-between items-center max-w-2xl m-auto box-border p-4"
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
  </div>

  <div class="box-border p-4 flex flex-col gap-4 max-w-2xl m-auto">
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
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        aria-haspopup="dialog"
        role="button"
        tabindex="0"
        on:click={() => (pickingDeadline = true)}
        class="w-full cursor-pointer"
      >
        <Label>Deadline</Label>
        <ButtonGroup class="w-full">
          <InputAddon>
            <HourglassOutline />
          </InputAddon>
          <Input
            class="w-full cursor-pointer"
            type="reset"
            value={deadlineDateFormatted}
          />
        </ButtonGroup>
      </div>
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div
        on:click={() => (pickingScheduled = true)}
        class="w-full cursor-pointer"
      >
        <Label>Scheduled</Label>
        <ButtonGroup class="w-full">
          <InputAddon>
            <ClipboardSolid />
          </InputAddon>
          <Input
            class="w-full cursor-pointer"
            type="reset"
            value={scheduleDateFormatted}
          />
        </ButtonGroup>
      </div>
    </div>
    <div>
      <Label>Priority</Label>
      <ButtonGroup class="w-full">
        <InputAddon>
          <FlagSolid />
        </InputAddon>
        <ul
          class="items-center w-full rounded-r-md border-l-0 border border-gray-200 sm:flex dark:bg-gray-800 dark:border-gray-600 divide-x rtl:divide-x-reverse divide-gray-200 dark:divide-gray-600"
        >
          <li class="w-full">
            <Radio
              name="hor-list"
              color="blue"
              bind:group={priority}
              value={0}
              class="p-3">Low</Radio
            >
          </li>
          <li class="w-full">
            <Radio
              name="hor-list"
              color="green"
              bind:group={priority}
              value={1}
              class="p-3">Standard</Radio
            >
          </li>
          <li class="w-full">
            <Radio
              color="red"
              name="hor-list"
              bind:group={priority}
              value={2}
              class="p-3">High</Radio
            >
          </li>
          <li class="w-full">
            <Radio
              name="hor-list"
              color="purple"
              bind:group={priority}
              value={3}
              class="p-3">Urgent</Radio
            >
          </li>
        </ul>
      </ButtonGroup>
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

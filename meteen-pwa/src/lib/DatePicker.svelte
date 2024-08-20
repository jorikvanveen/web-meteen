<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { fade, fly } from "svelte/transition";
  import Today from "../page/Today.svelte";
  import { Button, Radio } from "flowbite-svelte";
  import {
    AngleRightOutline,
    BanOutline,
    CalendarWeekSolid,
    EditSolid,
    SunSolid,
  } from "flowbite-svelte-icons";

  export let title: string;

  const dispatch = createEventDispatcher();

  function onBack(e: PopStateEvent) {
    e.preventDefault();
    e.stopImmediatePropagation();
    dispatch("close");
  }

  onMount(() => {
    history.pushState({ page: "picking" }, "", "#picking");

    window.addEventListener("popstate", onBack);

    return () => {
      window.removeEventListener("popstate", onBack);
    };
  });

  type DateType = "today" | "tomorrow" | "next-week" | "custom" | "none";
  let dateType: DateType = "today";

  $: console.log(dateType);

  let customDateValue: Date = new Date();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  transition:fade={{ duration: 200 }}
  class="fixed top-0 left-0 w-full h-full opacity-50 bg-black z-[29]"
  on:click={(e) => {
    e.preventDefault();
    history.back();
  }}
></div>
<div
  transition:fly={{ duration: 200, y: 1000 }}
  class="fixed bottom-0 left-0 w-full max-h-[3/4] bg-white z-30 flex flex-col box-border px-4"
>
  <h1 class="text-xl text-center m-2">{title}</h1>
  <div class="flex flex-row gap-4 justify-between">
    <div class="flex flex-col gap-1 w-full">
      <span class="font-bold mb-2">Date</span>
      <Radio name="custom" custom bind:group={dateType} value="today">
        <Button
          outline
          on:click={() => (dateType = "today")}
          color={dateType == "today" ? "primary" : "alternative"}
          class="w-full justify-start"
        >
          <CalendarWeekSolid class="mr-2" /> Today
        </Button>
      </Radio>
      <Radio name="custom" custom bind:group={dateType} value="tomorrow">
        <Button
          outline
          on:click={() => (dateType = "tomorrow")}
          color={dateType == "tomorrow" ? "primary" : "alternative"}
          class="w-full justify-start"
          ><SunSolid class="mr-2" /> Tomorrow</Button
        >
      </Radio>
      <Radio name="custom" custom bind:group={dateType} value="tomorrow">
        <Button
          outline
          on:click={() => (dateType = "next-week")}
          color={dateType == "next-week" ? "primary" : "alternative"}
          class="w-full justify-start"
          ><AngleRightOutline class="mr-2" /> Next week</Button
        >
      </Radio>
      <Radio name="custom" custom bind:group={dateType} value="tomorrow">
        <Button
          outline
          on:click={() => (dateType = "custom")}
          color={dateType == "custom" ? "primary" : "alternative"}
          class="w-full justify-start"><EditSolid class="mr-2" />Custom</Button
        >
      </Radio>
      <Radio name="custom" custom bind:group={dateType} value="none">
        <Button
          outline
          on:click={() => (dateType = "none")}
          color={dateType == "none" ? "primary" : "alternative"}
          class="w-full justify-start"><BanOutline class="mr-2" />None</Button
        >
      </Radio>
    </div>
    <div class="flex flex-col w-full">
      <span class="font-bold mb-2">Time</span>
    </div>
  </div>
</div>

<script lang="ts">
  import { run } from 'svelte/legacy';

  import { createEventDispatcher, onMount, tick } from "svelte";
  import { fade, fly } from "svelte/transition";
  import Today from "../page/Today.svelte";
  import { Button, Input, Radio } from "flowbite-svelte";
  import {
    AngleRightOutline,
    BanOutline,
    CalendarWeekSolid,
    EditSolid,
    EyeSolid,
    MoonSolid,
    SunOutline,
    SunSolid,
  } from "flowbite-svelte-icons";
  import { writable, type Writable } from "svelte/store";
  import RadioButton from "./RadioButton.svelte";
  import { formatDate, formatTime } from "./time";
  import {
    overwriteDate,
    overwriteTime,
    serializeToHtmlDate,
    serializeToHtmlTime,
  } from "./date_util";

  interface Props {
    datetime?: Date | null;
    withTime?: boolean;
  }

  let { datetime = $bindable(null), withTime = $bindable(false) }: Props = $props();

  const dispatch = createEventDispatcher();

  const isFirefox = !!navigator.userAgent.search("Firefox"); // Required because firefox decided not to implement any kind of picker for date="time"

  function onBack(e: PopStateEvent) {
    e.preventDefault();
    e.stopImmediatePropagation();
    dispatch("close");
  }

  function openTimePicker() {
    (
      document.getElementById("#timeInput") as unknown as HTMLInputElement
    ).showPicker();
  }

  function openDatePicker() {
    console.log("Opening date picker");
    (
      document.getElementById("#dateInput") as unknown as HTMLInputElement
    ).showPicker();
  }

  // When loading the component, we parse the `datetime` and assign values to dateType and timeType.
  // These changes should be ignored until the component is mounted, or we will get an infinite reactive feedback loop
  let ignoreInputChanges = true;

  onMount(() => {
    history.pushState({ page: "picking" }, "", "#picking");

    window.addEventListener("popstate", onBack);

    ignoreInputChanges = false;

    return () => {
      window.removeEventListener("popstate", onBack);
    };
  });

  let dateInputValue: string = $state("");

  let timeInputValue: string = $state("");

  function onDateInputChange(_: string) {
    if (dateInputValue == "") return;
    datetime = datetime || new Date();
    overwriteDate(datetime, new Date(dateInputValue));
    datetime = datetime;
  }

  function onTimeInputChange(_: string) {
    if (timeInputValue == "") return;
    datetime = datetime || new Date();
    overwriteTime(datetime, parseTime(timeInputValue));
    datetime = datetime;
  }

  function parseTime(timeStr: string): Date {
    const split = timeStr.split(":");
    const hour = parseInt(split[0]);
    const minute = parseInt(split[1]);
    return new Date(Date.UTC(80, 0, 0, hour - 1, minute));
  }
  type DateType = "today" | "tomorrow" | "next-week" | "custom" | "none";
  let dateType: DateType = $state("today");

  const today = new Date();
  const tomorrow = new Date(today.valueOf() + 24 * 60 * 60 * 1000);
  const nextWeek = new Date(today.valueOf() + 7 * 24 * 60 * 60 * 1000);

  if (datetime) {
    if (datetime.getDate() == today.getDate()) {
      dateType = "today";
    } else if (datetime.getDate() == tomorrow.getDate()) {
      dateType = "tomorrow";
    } else if (datetime.getDate() == nextWeek.getDate()) {
      dateType = "next-week";
    } else {
      dateType = "custom";
    }
  } else {
    dateType = "none";
  }

  function handleDateTypeChange(_: DateType) {
    switch (dateType) {
      case "today":
        datetime = datetime || new Date();
        overwriteDate(datetime, today);
        dateInputValue = serializeToHtmlDate(datetime);
        break;
      case "tomorrow":
        datetime = datetime || new Date();
        overwriteDate(datetime, tomorrow);
        dateInputValue = serializeToHtmlDate(datetime);
        break;
      case "next-week":
        datetime = datetime || new Date();
        overwriteDate(datetime, nextWeek);
        dateInputValue = serializeToHtmlDate(datetime);
        break;
      case "none":
        datetime = null;
        break;
      case "custom":
        break;
    }

    datetime = datetime;
  }

  const morning = new Date(Date.UTC(80, 1, 1, 8, 0, 0));
  const afternoon = new Date(Date.UTC(80, 1, 1, 12, 0, 0));
  const evening = new Date(Date.UTC(80, 1, 1, 16, 0, 0));
  const night = new Date(Date.UTC(80, 1, 1, 20, 0, 0));

  type TimeType =
    | "morning"
    | "afternoon"
    | "evening"
    | "night"
    | "custom"
    | "none";
  let timeType: TimeType = $state("none");

  if (withTime && datetime) {
    let hour = datetime.getHours();
    let minute = datetime.getMinutes();

    if (hour == 9 && minute == 0) {
      timeType = "morning";
    } else if (hour == 13 && minute == 0) {
      timeType = "afternoon";
    } else if (hour == 17 && minute == 0) {
      timeType = "evening";
    } else if (hour == 21 && minute == 0) {
      timeType = "custom";
    }
  } else {
    timeType = "none";
  }

  function handleTimeTypeChange(_: TimeType) {
    switch (timeType) {
      case "morning":
        datetime = datetime || new Date();
        overwriteTime(datetime, morning);
        break;
      case "afternoon":
        datetime = datetime || new Date();
        overwriteTime(datetime, afternoon);
        break;
      case "evening":
        datetime = datetime || new Date();
        overwriteTime(datetime, evening);
        break;
      case "night":
        datetime = datetime || new Date();
        overwriteTime(datetime, night);
        break;
      case "custom":
        break; // Will be updated later
      case "none":
        withTime = false;
        break;
    }

    datetime = datetime;

    if (datetime) {
      timeInputValue = serializeToHtmlTime(datetime);
    }
  }

  run(() => {
    onDateInputChange(dateInputValue);
  });
  run(() => {
    onTimeInputChange(timeInputValue);
  });
  run(() => {
    handleDateTypeChange(dateType);
  });
  run(() => {
    handleTimeTypeChange(timeType);
  });
  run(() => {
    withTime = timeType != "none";
  });
  run(() => {
    console.log(datetime);
  });
  let timeDisabled = $derived(datetime == null);
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  transition:fade={{ duration: 200 }}
  class="fixed top-0 left-0 w-full h-full opacity-50 bg-black z-[29]"
  onclick={(e) => {
    e.preventDefault();
    history.back();
  }}
></div>
<div
  transition:fly={{ duration: 200, y: 1000 }}
  class="fixed bottom-0 left-0 w-full max-h-[3/4] bg-white z-30 flex flex-col items-center box-border p-4"
>
  <div class="flex flex-row gap-4 justify-between w-full max-w-xl">
    <div class="flex flex-col gap-1 w-full">
      <span class="mb-2 text-right text-xl"
        >{datetime ? formatDate(datetime) : "(No date)"}</span
      >
      <RadioButton bind:selectedValue={dateType} value="today">
        <CalendarWeekSolid class="mr-2" /> Today
      </RadioButton>
      <RadioButton bind:selectedValue={dateType} value="tomorrow">
        <SunSolid class="mr-2" /> Tomorrow
      </RadioButton>
      <RadioButton bind:selectedValue={dateType} value="next-week">
        <AngleRightOutline class="mr-2" /> Next week
      </RadioButton>
      <RadioButton
        bind:selectedValue={dateType}
        on:click={openDatePicker}
        value="custom"
      >
        <EditSolid class="mr-2" /> Custom
      </RadioButton>
      <RadioButton bind:selectedValue={dateType} value="none">
        <BanOutline class="mr-2" /> None
      </RadioButton>
      <input
        bind:value={dateInputValue}
        id="#dateInput"
        type="date"
        class="w-0 h-0 p-0 m-0 border-0"
      />
    </div>

    <div class="flex flex-col gap-1 w-full">
      <span class="text-xl mb-2"
        >{datetime && withTime ? formatTime(datetime) : "(No time)"}</span
      >
      <RadioButton
        disabled={timeDisabled}
        bind:selectedValue={timeType}
        value="morning"
      >
        <EyeSolid class="mr-2" />
        {formatTime(morning)}
      </RadioButton>
      <RadioButton
        disabled={timeDisabled}
        bind:selectedValue={timeType}
        value="afternoon"
      >
        <SunSolid class="mr-2" />
        {formatTime(afternoon)}
      </RadioButton>
      <RadioButton
        disabled={timeDisabled}
        bind:selectedValue={timeType}
        value="evening"
      >
        <SunOutline class="mr-2" />
        {formatTime(evening)}
      </RadioButton>
      <RadioButton
        disabled={timeDisabled}
        bind:selectedValue={timeType}
        value="night"
      >
        <MoonSolid class="mr-2" />
        {formatTime(night)}
      </RadioButton>
      <RadioButton
        disabled={timeDisabled}
        bind:selectedValue={timeType}
        on:click={openTimePicker}
        value="custom"
      >
        <EditSolid class="mr-2" /> Custom
      </RadioButton>
      <RadioButton
        disabled={timeDisabled}
        bind:selectedValue={timeType}
        value="none"
      >
        <BanOutline class="mr-2" /> None
      </RadioButton>
      <input
        class="w-0 h-0 p-0 m-0 border-0"
        bind:value={timeInputValue}
        id="#timeInput"
        type="time"
      />
    </div>
  </div>
</div>

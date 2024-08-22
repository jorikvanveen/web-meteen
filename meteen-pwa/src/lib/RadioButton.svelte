<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { CalendarWeekSolid } from "flowbite-svelte-icons";
  import { createEventDispatcher } from "svelte";
  import type { Writable } from "svelte/store";

  export let selectedValue: string;
  export let value: string;
  export let disabled: boolean = false;

  $: selected = selectedValue == value;
</script>

<Button
  {disabled}
  outline={!selected}
  on:click
  on:click={() => {
    selectedValue = value;
  }}
  color={selected ? "primary" : "alternative"}
  class="w-full justify-start"
>
  <slot />
</Button>

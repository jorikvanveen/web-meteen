<script lang="ts">
  import { Button } from "flowbite-svelte";
  import { CalendarWeekSolid } from "flowbite-svelte-icons";
  import { createEventDispatcher } from "svelte";
  import type { Writable } from "svelte/store";

  interface Props {
    selectedValue: string;
    value: string;
    disabled?: boolean;
    children?: import('svelte').Snippet;
  }

  let {
    selectedValue = $bindable(),
    value,
    disabled = false,
    children
  }: Props = $props();

  let selected = $derived(selectedValue == value);
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
  {@render children?.()}
</Button>

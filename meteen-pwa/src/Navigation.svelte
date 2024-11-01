<script lang="ts">
  // @ts-nocheck
  // There's a weird bug where BottomNav's type doesn't
  // include classInner but defining it does change the behavior
  import { BottomNav, BottomNavItem } from "flowbite-svelte";
  import {
    CalendarWeekSolid,
    DatabaseSolid,
    InboxSolid,
  } from "flowbite-svelte-icons";
  import { Page } from "./lib/page";

  interface Props {
    activePage: Page;
  }

  let { activePage }: Props = $props();

  let onToday = $derived(activePage == Page.Today);
  let onInbox = $derived(activePage == Page.Inbox);
  let onBrowse = $derived(activePage == Page.Browse);
</script>

<BottomNav
  position="fixed"
  outerClass="fixed w-full z-10 border-gray-200 dark:bg-gray-700 dark:border-gray-600 bottom-0 start-0 h-16 bg-white border-t"
  classInner="grid-cols-3 m-auto"
>
  <BottomNavItem on:click={() => (activePage = Page.Today)} btnName="Today">
    <CalendarWeekSolid class={onToday ? "fill-primary-500" : ""} size="lg" />
  </BottomNavItem>
  <BottomNavItem on:click={() => (activePage = Page.Inbox)} btnName="Inbox">
    <InboxSolid class={onInbox ? "fill-primary-500" : ""} size="lg" />
  </BottomNavItem>
  <BottomNavItem on:click={() => (activePage = Page.Browse)} btnName="Browse">
    <DatabaseSolid class={onBrowse ? "fill-primary-500" : ""} size="lg" />
  </BottomNavItem>
</BottomNav>

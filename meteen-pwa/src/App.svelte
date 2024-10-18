<script lang="ts">
  export let storage: Storage;

  import PWABadge from "./lib/PWABadge.svelte";
  import type Storage from "./lib/storage";
  import { setContext } from "svelte";
  import { BottomNav, BottomNavItem } from "flowbite-svelte";
  import {
    CalendarWeekSolid,
    InboxSolid,
    BarsOutline,
    DatabaseSolid,
  } from "flowbite-svelte-icons";
  import Today from "./page/Today.svelte";
  import Inbox from "./page/Inbox.svelte";
  import Browse from "./page/Browse.svelte";

  let loading = storage.load();
  setContext("storage", storage);

  enum Page {
    Today,
    Inbox,
    Browse,
  }

  let activePage: Page = Page.Today;

  $: onToday = activePage == Page.Today;
  $: onInbox = activePage == Page.Inbox;
  $: onBrowse = activePage == Page.Browse;
</script>

{#await loading}
  <span>...</span>
{:then _}
  <div class="app-container p-4 pt-6 box-border max-w-4xl m-auto">
    {#if activePage == Page.Today}
      <Today />
    {:else if activePage == Page.Inbox}
      <Inbox />
    {:else}
      <Browse />
    {/if}
  </div>
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

  <PWABadge />

  <style>
    .app-container {
      width: 100%;
      height: calc(100vh - 4rem);
      overflow-y: scroll;
    }
  </style>
{:catch error}
  {console.error(error)}
  <p>Something went wrong</p>
{/await}

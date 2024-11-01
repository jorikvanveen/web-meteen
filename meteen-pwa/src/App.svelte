<script lang="ts">
  import PWABadge from "./lib/PWABadge.svelte";
  import type Storage from "./lib/storage";
  import { setContext } from "svelte";
  import Today from "./page/Today.svelte";
  import Inbox from "./page/Inbox.svelte";
  import Browse from "./page/Browse.svelte";
  import Navigation from "./Navigation.svelte";
  import { Page } from "./lib/page";
  interface Props {
    storage: Storage;
  }

  let { storage }: Props = $props();

  let loading = storage.load();
  setContext("storage", storage);

  let activePage: Page = $state(Page.Today) as Page;
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
  <Navigation {activePage} />

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

<!--
 Copyright (c) 2026 Simon-Weij

 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at https://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
  import CloseIcon from "../icons/CloseIcon.svelte";
  import MaximizeIcon from "../icons/MaximizeIcon.svelte";
  import RestoreIcon from "../icons/RestoreIcon.svelte";
  import "../app.css";
  import { onMount } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import MinusIcon from "../icons/MinusIcon.svelte";

  let appWindow: ReturnType<typeof getCurrentWindow>;
  let isMaximized = false;

  onMount(() => {
    appWindow = getCurrentWindow();
  });

  function minimize() {
    appWindow?.minimize();
  }

  async function toggleMaximize() {
    await appWindow?.toggleMaximize();
    isMaximized = !isMaximized;
  }

  function close() {
    appWindow?.close();
  }
</script>

<div class="h-screen flex flex-col">
  <!-- Titlebar -->
  <div
    class="h-8 bg-[#329ea3] select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0 z-50"
  >
    <div data-tauri-drag-region class="flex-1"></div>
    <div class="flex">
      <button
        onclick={minimize}
        title="minimize"
        class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-8 bg-transparent hover:bg-[#5bbec3] transition-colors"
      >
        <MinusIcon />
      </button>
      <button
        onclick={toggleMaximize}
        title="maximize"
        class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-8 bg-transparent hover:bg-[#5bbec3] transition-colors"
      >
        {#if isMaximized}
          <RestoreIcon />
        {:else}
          <MaximizeIcon />
        {/if}
      </button>
      <button
        onclick={close}
        title="close"
        class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-8 bg-transparent hover:bg-red-500 transition-colors"
      >
        <CloseIcon />
      </button>
    </div>
  </div>

  <div class="pt-8 flex-1 overflow-auto">
    <slot />
  </div>
</div>

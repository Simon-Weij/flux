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
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import MinusIcon from "../icons/MinusIcon.svelte";

  let appWindow: ReturnType<typeof getCurrentWindow>;
  let isMaximized = false;
  let unlisten: (() => void) | null = null;
  let isDragging = false;
  let dragTimeout: number;

  onMount(async () => {
    appWindow = getCurrentWindow();

    isMaximized = await appWindow.isMaximized();

    unlisten = await appWindow.onResized(async () => {
      isMaximized = await appWindow.isMaximized();
    });
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
    clearTimeout(dragTimeout);
  });

  function minimize() {
    appWindow?.minimize();
  }

  async function toggleMaximize() {
    await appWindow?.toggleMaximize();
  }

  function close() {
    appWindow?.close();
  }

  async function handleTitlebarDoubleClick(event: MouseEvent) {
    if (!isDragging) {
      event.preventDefault();
      event.stopPropagation();
      await toggleMaximize();
    }
  }

  function handleMouseDown() {
    isDragging = false;
    dragTimeout = window.setTimeout(() => {
      isDragging = true;
    }, 150);
  }

  function handleMouseUp() {
    clearTimeout(dragTimeout);
  }
</script>

<div class="h-screen flex flex-col bg-[#151515] text-white">
  <!-- Titlebar -->
  <div
    class="h-12 select-none grid grid-cols-[auto_max-content] fixed top-0 left-0 right-0 z-50 bg-[#2e2e2e] shadow-md"
  >
    <div
      data-tauri-drag-region
      class="flex-1"
      ondblclick={handleTitlebarDoubleClick}
      onmousedown={handleMouseDown}
      onmouseup={handleMouseUp}
      role="button"
      tabindex="-1"
    ></div>
    <div class="flex">
      <button
        onclick={minimize}
        title="minimize"
        class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-8 bg-transparent hover:bg-blue-600 transition-colors"
      >
        <MinusIcon />
      </button>
      <button
        onclick={toggleMaximize}
        title="maximize"
        class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-8 bg-transparent hover:bg-blue-600 transition-colors"
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
        class="appearance-none p-0 m-0 border-none inline-flex justify-center items-center w-8 bg-transparent hover:bg-red-600 transition-colors"
      >
        <CloseIcon />
      </button>
    </div>
  </div>

  <div class="pt-12 flex-1 overflow-auto">
    <slot />
  </div>
</div>

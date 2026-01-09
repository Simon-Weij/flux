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
  import { goto } from "$app/navigation";
  import MinusIcon from "../icons/MinusIcon.svelte";
  import UserIcon from "../icons/UserIcon.svelte";
  import UserDropdown from "../components/UserDropdown.svelte";

  let appWindow: ReturnType<typeof getCurrentWindow>;
  let isMaximized = false;
  let unlisten: (() => void) | null = null;
  let showUserDropdown = false;
  let clickOutsideUnlisten: (() => void) | null = null;
  let showFirstLaunchModal = false;

  onMount(async () => {
    appWindow = getCurrentWindow();

    isMaximized = await appWindow.isMaximized();

    unlisten = await appWindow.onResized(async () => {
      isMaximized = await appWindow.isMaximized();
    });

    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as HTMLElement;
      if (
        !target.closest(".user-button") &&
        !target.closest(".user-dropdown")
      ) {
        showUserDropdown = false;
      }
    };

    document.addEventListener("click", handleClickOutside);
    clickOutsideUnlisten = () =>
      document.removeEventListener("click", handleClickOutside);

    if (!localStorage.getItem("firstLaunch")) {
      showFirstLaunchModal = true;
      localStorage.setItem("firstLaunch", "true");
    }
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
    if (clickOutsideUnlisten) {
      clickOutsideUnlisten();
    }
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
</script>

<div class="h-screen flex flex-col bg-[#151515] text-white">
  <!-- Titlebar -->
  <div
    class="h-12 select-none grid grid-cols-[max-content_auto_max-content] fixed top-0 left-0 right-0 z-50 bg-[#2e2e2e] shadow-md"
  >
    <div class="relative flex user-button">
      <button
        onclick={() => (showUserDropdown = !showUserDropdown)}
        title="user"
        class="inline-flex items-center justify-center w-8 p-0 m-0 transition-colors bg-transparent border-none appearance-none cursor-pointer hover:bg-gray-500"
      >
        <UserIcon />
      </button>
      <UserDropdown
        show={showUserDropdown}
        onSettings={() => {
          showUserDropdown = false;
          goto("/settings");
        }}
        onHome={() => {
          showUserDropdown = false;
          goto("/");
        }}
      />
    </div>
    <div
      id="titlebar"
      class="flex-1"
      role="button"
      tabindex="0"
      onmousedown={(e) => {
        if (e.buttons === 1) {
          if (e.detail === 2) {
            toggleMaximize();
          } else {
            appWindow?.startDragging();
          }
        }
      }}
      onkeydown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          toggleMaximize();
        }
      }}
    ></div>
    <div class="flex">
      <button
        onclick={minimize}
        title="minimize"
        class="inline-flex items-center justify-center w-8 p-0 m-0 transition-colors bg-transparent border-none appearance-none cursor-pointer hover:bg-gray-500"
      >
        <MinusIcon />
      </button>
      <button
        onclick={toggleMaximize}
        title="maximize"
        class="inline-flex items-center justify-center w-8 p-0 m-0 transition-colors bg-transparent border-none appearance-none cursor-pointer hover:bg-gray-500"
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
        class="inline-flex items-center justify-center w-8 p-0 m-0 transition-colors bg-transparent border-none appearance-none cursor-pointer hover:bg-red-600"
      >
        <CloseIcon />
      </button>
    </div>
  </div>

  <div class="flex-1 pt-12 overflow-auto">
    <slot />
  </div>

  {#if showFirstLaunchModal}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-md"
      onclick={() => (showFirstLaunchModal = false)}
      onkeydown={(e) => {
        if (e.key === "Escape") showFirstLaunchModal = false;
      }}
      role="button"
      tabindex="0"
    >
      <div
        class="w-full max-w-md p-6 mx-4 bg-gray-800 rounded-lg shadow-lg"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => e.stopPropagation()}
        role="dialog"
        tabindex="-1"
      >
        <h2 class="mb-4 text-xl font-bold text-white">Hello World!</h2>
        <p class="mb-4 text-gray-300">Welcome to your first launch.</p>
        <button
          class="px-4 py-2 text-white bg-blue-600 rounded hover:bg-blue-700"
          onclick={() => (showFirstLaunchModal = false)}>Close</button
        >
      </div>
    </div>
  {/if}
</div>

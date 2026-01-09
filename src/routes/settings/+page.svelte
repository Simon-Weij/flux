<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Select from "../../components/Select.svelte";

  let backend = "obs";
  let clipLength = 30;
  let clipHotkey: string[] = ["KEY_LEFTALT", "KEY_Z"];
  let currentHotkey: string[] = [...clipHotkey];
  let pressedModifiers = new Set<string>();
  let mainKey = "";
  let isCapturing = false;
  let showToast = false;

  const backendOptions = [{ value: "obs", label: "OBS" }];

  function keyToLinuxKey(key: string, location?: number): string {
    const keyMap: { [key: string]: string } = {
      "Control": location === 1 ? "KEY_LEFTCTRL" : "KEY_RIGHTCTRL",
      "Alt": location === 1 ? "KEY_LEFTALT" : "KEY_RIGHTALT", 
      "Shift": location === 1 ? "KEY_LEFTSHIFT" : "KEY_RIGHTSHIFT",
      " ": "KEY_SPACE",
      "Enter": "KEY_ENTER",
      "Escape": "KEY_ESC",
      "Backspace": "KEY_BACKSPACE",
      "Tab": "KEY_TAB",
      "ArrowUp": "KEY_UP",
      "ArrowDown": "KEY_DOWN",
      "ArrowLeft": "KEY_LEFT",
      "ArrowRight": "KEY_RIGHT",
    };

    if (keyMap[key]) return keyMap[key];
    if (key.length === 1) {
      const code = key.toUpperCase().charCodeAt(0);
      if (code >= 65 && code <= 90) return `KEY_${key.toUpperCase()}`;
      if (code >= 48 && code <= 57) return `KEY_${key}`;
    }
    return `KEY_${key.toUpperCase()}`;
  }

  function startCapture() {
    isCapturing = true;
    currentHotkey = [];
    pressedModifiers.clear();
    mainKey = "";
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!isCapturing) return;
    event.preventDefault();
    if (event.key === "Control") pressedModifiers.add(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Alt") pressedModifiers.add(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Shift") pressedModifiers.add(keyToLinuxKey(event.key, event.location));
    else mainKey = keyToLinuxKey(event.key);
    updateCurrentHotkey();
  }

  function handleKeyup(event: KeyboardEvent) {
    if (!isCapturing) return;
    if (event.key === "Control") pressedModifiers.delete(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Alt") pressedModifiers.delete(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Shift") pressedModifiers.delete(keyToLinuxKey(event.key, event.location));
    else mainKey = "";
    if (pressedModifiers.size === 0 && mainKey === "") {
      clipHotkey = [...currentHotkey];
      isCapturing = false;
    }
  }

  function updateCurrentHotkey() {
    if (!isCapturing) return;
    currentHotkey = Array.from(pressedModifiers);
    if (mainKey) currentHotkey.push(mainKey);
  }

  $: displayHotkey = isCapturing ? currentHotkey.join(" + ") : clipHotkey.join(" + ");

  async function saveSettings() {
    try {
      await invoke("save_settings", {
        backend,
        clipLength,
        clipHotkey,
      });
      showToast = true;
      setTimeout(() => (showToast = false), 3000);
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  }

  onMount(() => {
    (async () => {
      try {
        const settings = (await invoke("load_settings")) as {
          backend: string;
          clip_length: number;
          clip_hotkey: string[];
        };
        backend = settings.backend;
        clipLength = settings.clip_length;
        clipHotkey = settings.clip_hotkey;
        currentHotkey = [...clipHotkey];
      } catch (error) {
        console.error("Failed to load settings:", error);
        backend = "obs";
        clipLength = 30;
        clipHotkey = ["KEY_LEFTALT", "KEY_Z"];
        currentHotkey = [...clipHotkey];
      }
    })();

    const keydownListener = (event: KeyboardEvent) => handleKeydown(event);
    const keyupListener = (event: KeyboardEvent) => handleKeyup(event);

    document.addEventListener("keydown", keydownListener);
    document.addEventListener("keyup", keyupListener);

    return () => {
      document.removeEventListener("keydown", keydownListener);
      document.removeEventListener("keyup", keyupListener);
    };
  });
</script>

<div class="px-[20%] py-[3%]">
  <h1 class="mb-8 text-2xl font-bold text-center text-white">Settings</h1>

  <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
    <div>
      <label for="backend" class="block mb-2 font-medium text-gray-300"
        >Backend:</label
      >
      <Select id="backend" bind:value={backend} options={backendOptions} />
    </div>

    <div>
      <label for="clipLength" class="block mb-2 font-medium text-gray-300"
        >Clip Length:</label
      >
      <input
        type="number"
        id="clipLength"
        bind:value={clipLength}
        class="w-full p-3 text-white bg-gray-800 border border-gray-600 rounded focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
      />
    </div>

    <div>
      <label for="clipHotkey" class="block mb-2 font-medium text-gray-300"
        >Clip Hotkey:</label
      >
      <button
        id="clipHotkey"
        onclick={startCapture}
        class="w-full p-3 text-left text-white bg-gray-700 border border-gray-600 rounded cursor-pointer focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
      >
        {displayHotkey}
      </button>
    </div>
  </div>

  <div class="mt-8 text-center">
    <button
      onclick={saveSettings}
      class="px-6 py-3 font-medium text-white bg-gray-600 rounded-lg cursor-pointer hover:bg-gray-400"
    >
      Save Settings
    </button>
  </div>
</div>

{#if showToast}
  <div
    class="fixed z-50 px-4 py-2 text-white bg-green-600 rounded-lg shadow-lg bottom-4 right-4"
  >
    Successfully saved settings!
  </div>
{/if}

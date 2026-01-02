<script lang="ts">
  import { onMount } from "svelte";
  import Select from "../../components/Select.svelte";

  let backend = "obs";
  let clipLength = 30;
  let clipHotkey = "Alt+Z";
  let currentHotkey = clipHotkey;
  let pressedModifiers = new Set<string>();
  let mainKey = "";
  let isCapturing = false;
  let showToast = false;

  const backendOptions = [{ value: "obs", label: "OBS" }];

  function startCapture() {
    isCapturing = true;
    currentHotkey = "Press keys";
    pressedModifiers.clear();
    mainKey = "";
  }

  function handleKeydown(event: KeyboardEvent) {
    if (!isCapturing) return;
    event.preventDefault();
    if (event.key === "Control") pressedModifiers.add("Ctrl");
    else if (event.key === "Alt") pressedModifiers.add("Alt");
    else if (event.key === "Shift") pressedModifiers.add("Shift");
    else mainKey = event.key.toUpperCase();
    updateCurrentHotkey();
  }

  function handleKeyup(event: KeyboardEvent) {
    if (!isCapturing) return;
    if (event.key === "Control") pressedModifiers.delete("Ctrl");
    else if (event.key === "Alt") pressedModifiers.delete("Alt");
    else if (event.key === "Shift") pressedModifiers.delete("Shift");
    else mainKey = "";
    if (pressedModifiers.size === 0 && mainKey === "") {
      clipHotkey = currentHotkey;
      isCapturing = false;
    }
  }

  function updateCurrentHotkey() {
    if (!isCapturing) return;
    let keys = Array.from(pressedModifiers);
    if (mainKey) keys.push(mainKey);
    currentHotkey = keys.join("+");
  }

  $: displayHotkey = isCapturing ? currentHotkey : clipHotkey;

  function saveSettings() {
    localStorage.setItem("backend", backend);
    localStorage.setItem("clipLength", clipLength.toString());
    localStorage.setItem("clipHotkey", clipHotkey);
    showToast = true;
    setTimeout(() => (showToast = false), 3000);
  }

  onMount(() => {
    // Load settings from localStorage
    backend = localStorage.getItem("backend") || "obs";
    clipLength = parseInt(localStorage.getItem("clipLength") || "30");
    clipHotkey = localStorage.getItem("clipHotkey") || "Alt+Z";
    currentHotkey = clipHotkey;

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

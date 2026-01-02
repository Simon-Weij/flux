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

  onMount(() => {
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

<h1 class="text-white text-center mb-8 text-2xl font-bold">Settings</h1>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
  <div>
    <label for="backend" class="block text-gray-300 mb-2 font-medium"
      >Backend:</label
    >
    <Select id="backend" bind:value={backend} options={backendOptions} />
  </div>

  <div>
    <label for="clipLength" class="block text-gray-300 mb-2 font-medium"
      >Clip Length:</label
    >
    <input
      type="number"
      id="clipLength"
      bind:value={clipLength}
      class="w-full p-3 border border-gray-600 rounded bg-gray-800 text-white focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
    />
  </div>

  <div>
    <label for="clipHotkey" class="block text-gray-300 mb-2 font-medium"
      >Clip Hotkey:</label
    >
    <button
      id="clipHotkey"
      onclick={startCapture}
      class="w-full p-3 border border-gray-600 rounded bg-gray-700 text-white cursor-pointer focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25 text-left"
    >
      {displayHotkey}
    </button>
  </div>
</div>

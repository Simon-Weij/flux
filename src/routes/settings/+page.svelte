<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Select from "../../components/Select.svelte";

  let backend = "gpu-screen-recorder";
  let clipLength = 30;
  let clipHotkey: string[] = ["KEY_LEFTALT", "KEY_Z"];
  let framerate = 60;
  let replay_time = 30;
  let container = "mp4";
  let output = "~/Videos/clip";
  let codec = "h264";
  let quality = 20;
  let framerate_mode = "vfr";
  let bitrate_mode = "cqp";
  let currentHotkey: string[] = [...clipHotkey];
  let pressedModifiers = new Set<string>();
  let mainKey = "";
  let isCapturing = false;
  let showToast = false;

  let captureOptions: string[] = [];
  let audioDevices: string[] = [];

  const backendOptions = [
    { value: "gpu-screen-recorder", label: "GPU Screen Recorder" },
  ];

  const containerOptions = [
    { value: "mp4", label: "MP4" },
    { value: "mkv", label: "MKV" },
    { value: "webm", label: "WebM" },
  ];

  const codecOptions = [
    { value: "h264", label: "H.264" },
    { value: "hevc", label: "HEVC" },
    { value: "av1", label: "AV1" },
  ];

  const framerateModeOptions = [
    { value: "vfr", label: "Variable Framerate" },
    { value: "cfr", label: "Constant Framerate" },
  ];

  const bitrateModeOptions = [
    { value: "cqp", label: "Constant Quality" },
    { value: "cbr", label: "Constant Bitrate" },
  ];

  function keyToLinuxKey(key: string, location?: number): string {
    const keyMap: { [key: string]: string } = {
      Control: location === 1 ? "KEY_LEFTCTRL" : "KEY_RIGHTCTRL",
      Alt: location === 1 ? "KEY_LEFTALT" : "KEY_RIGHTALT",
      Shift: location === 1 ? "KEY_LEFTSHIFT" : "KEY_RIGHTSHIFT",
      " ": "KEY_SPACE",
      Enter: "KEY_ENTER",
      Escape: "KEY_ESC",
      Backspace: "KEY_BACKSPACE",
      Tab: "KEY_TAB",
      ArrowUp: "KEY_UP",
      ArrowDown: "KEY_DOWN",
      ArrowLeft: "KEY_LEFT",
      ArrowRight: "KEY_RIGHT",
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
    if (event.key === "Control")
      pressedModifiers.add(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Alt")
      pressedModifiers.add(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Shift")
      pressedModifiers.add(keyToLinuxKey(event.key, event.location));
    else mainKey = keyToLinuxKey(event.key);
    updateCurrentHotkey();
  }

  function handleKeyup(event: KeyboardEvent) {
    if (!isCapturing) return;
    if (event.key === "Control")
      pressedModifiers.delete(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Alt")
      pressedModifiers.delete(keyToLinuxKey(event.key, event.location));
    else if (event.key === "Shift")
      pressedModifiers.delete(keyToLinuxKey(event.key, event.location));
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

  $: displayHotkey = isCapturing
    ? currentHotkey.join(" + ")
    : clipHotkey.join(" + ");

  async function saveSettings() {
    try {
      await invoke("save_settings", {
        backend,
        clipLength,
        clipHotkey,
        framerate,
        replay_time,
        container,
        output,
        codec,
        quality,
        framerate_mode,
        bitrate_mode,
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
          framerate: number;
          replay_time: number;
          container: string;
          output: string;
          codec: string;
          quality: number;
          framerate_mode: string;
          bitrate_mode: string;
        };
        backend = settings.backend;
        clipLength = settings.clip_length;
        clipHotkey = settings.clip_hotkey;
        framerate = settings.framerate;
        replay_time = settings.replay_time;
        container = settings.container;
        output = settings.output;
        codec = settings.codec;
        quality = settings.quality;
        framerate_mode = settings.framerate_mode;
        bitrate_mode = settings.bitrate_mode;
        currentHotkey = [...clipHotkey];

        try {
          captureOptions = await invoke("get_capture_options");
        } catch (e) {
          console.error("Failed to load options:", e);
        }
      } catch (error) {
        console.error("Failed to load settings:", error);
        backend = "gpu-screen-recorder";
        clipLength = 30;
        clipHotkey = ["KEY_LEFTALT", "KEY_Z"];
        framerate = 60;
        replay_time = 30;
        container = "mp4";
        output = "~/Videos/clip";
        codec = "h264";
        quality = 20;
        framerate_mode = "vfr";
        bitrate_mode = "cqp";
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

    {#if backend === "gpu-screen-recorder"}
      <div>
        <label for="framerate" class="block mb-2 font-medium text-gray-300"
          >Framerate:</label
        >
        <input
          type="number"
          id="framerate"
          bind:value={framerate}
          class="w-full p-3 text-white bg-gray-800 border border-gray-600 rounded focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
        />
      </div>

      <div>
        <label for="container" class="block mb-2 font-medium text-gray-300"
          >Container:</label
        >
        <Select
          id="container"
          bind:value={container}
          options={containerOptions}
        />
      </div>

      <div>
        <label for="output" class="block mb-2 font-medium text-gray-300"
          >Output:</label
        >
        <div class="flex">
          <input
            type="text"
            id="output"
            bind:value={output}
            class="flex-1 p-3 text-white bg-gray-800 border border-gray-600 rounded-l focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
          />
          <button
            onclick={async () => {
              try {
                const folder = (await invoke("select_folder")) as string;
                if (folder) output = folder;
              } catch (e) {
                console.error("Failed to select folder:", e);
              }
            }}
            class="px-3 py-3 text-white bg-gray-700 border border-l-0 border-gray-600 rounded-r hover:bg-gray-600"
          >
            Browse
          </button>
        </div>
      </div>

      <div>
        <label for="codec" class="block mb-2 font-medium text-gray-300"
          >Codec:</label
        >
        <Select id="codec" bind:value={codec} options={codecOptions} />
      </div>

      <div>
        <label for="quality" class="block mb-2 font-medium text-gray-300"
          >Quality (lower = better):</label
        >
        <input
          type="number"
          id="quality"
          bind:value={quality}
          class="w-full p-3 text-white bg-gray-800 border border-gray-600 rounded focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
        />
      </div>

      <div>
        <label for="framerate_mode" class="block mb-2 font-medium text-gray-300"
          >Framerate Mode:</label
        >
        <Select
          id="framerate_mode"
          bind:value={framerate_mode}
          options={framerateModeOptions}
        />
      </div>

      <div>
        <label for="bitrate_mode" class="block mb-2 font-medium text-gray-300"
          >Bitrate Mode:</label
        >
        <Select
          id="bitrate_mode"
          bind:value={bitrate_mode}
          options={bitrateModeOptions}
        />
      </div>

      <div>
        <label for="replay_time" class="block mb-2 font-medium text-gray-300"
          >Replay Time:</label
        >
        <input
          type="number"
          id="replay_time"
          bind:value={replay_time}
          class="w-full p-3 text-white bg-gray-800 border border-gray-600 rounded focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25"
        />
      </div>
    {/if}
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

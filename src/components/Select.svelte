<script lang="ts">
  import { onMount } from "svelte";

  export let value: string;
  export let options: { value: string; label: string }[] = [];
  export let placeholder: string = "Select an option";
  export let id: string | undefined = undefined;
  let showDropdown = false;
  let selectedLabel = placeholder;
  let container: HTMLDivElement;

  $: {
    const selectedOption = options.find((opt) => opt.value === value);
    selectedLabel = selectedOption ? selectedOption.label : placeholder;
  }

  function toggleDropdown() {
    showDropdown = !showDropdown;
  }

  function selectOption(option: { value: string; label: string }) {
    value = option.value;
    showDropdown = false;
  }

  onMount(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (container && !container.contains(event.target as Node)) {
        showDropdown = false;
      }
    };
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });
</script>

<div class="relative" bind:this={container}>
  <button
    type="button"
    {id}
    onclick={toggleDropdown}
    class="w-full p-3 border border-gray-600 rounded bg-gray-800 text-white focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25 text-left flex justify-between items-center"
  >
    <span>{selectedLabel}</span>
    <svg
      class="w-5 h-5 transition-transform {showDropdown ? 'rotate-180' : ''}"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M19 9l-7 7-7-7"
      ></path>
    </svg>
  </button>

  {#if showDropdown}
    <div
      class="absolute top-full left-0 right-0 mt-1 bg-gray-800 border border-gray-600 rounded shadow-lg z-50 max-h-60 overflow-y-auto"
    >
      {#each options as option}
        <button
          type="button"
          onclick={() => selectOption(option)}
          class="block w-full px-4 py-2 text-left text-white hover:bg-gray-700 transition-colors"
        >
          {option.label}
        </button>
      {/each}
    </div>
  {/if}
</div>

<script lang="ts">
  export let show: boolean = false;
  export let onSettings: () => void;
</script>

{#if show}
  <div
    class="user-dropdown absolute top-12 left-0 bg-[#2e2e2e] border border-gray-600 rounded-md shadow-lg z-50"
  >
    <button
      onclick={onSettings}
      class="block w-full px-4 py-2 text-left text-white transition-colors cursor-pointer hover:bg-gray-700"
    >
      Settings
    </button>
    <button
      class="block w-full px-4 py-2 text-left text-white transition-colors cursor-pointer hover:bg-gray-700"
    >
      Login
    </button>
  </div>
{/if}

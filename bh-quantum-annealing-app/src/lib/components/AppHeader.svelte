<script lang="ts">
  import type { Tab, TabId } from "$lib/app-tabs";

  let {
    tabs,
    activeTab,
    headerStyle,
    onSelectTab
  }: {
    tabs: Tab[];
    activeTab: TabId;
    headerStyle: string;
    onSelectTab: (tab: TabId) => void;
  } = $props();
</script>

<header class="top-chrome" style={headerStyle}>
  <div class="brand-strip">
    <span class="brand-name">Quantum Annealing</span>
  </div>
  <nav class="tab-bar" aria-label="Application pages">
    {#each tabs as tab}
      <button
        class:home-tab={tab.id === "home"}
        class:active={activeTab === tab.id}
        class="tab-button"
        type="button"
        onclick={() => onSelectTab(tab.id)}
        aria-label={tab.label}
      >
        {#if tab.id === "home"}
          <img src="/icons/pixel-house.png" alt="" class="home-icon" />
        {:else}
          <span class="tab-label">{tab.label}</span>
        {/if}
      </button>
    {/each}
  </nav>
</header>

<style>
  .top-chrome {
    position: sticky;
    top: 1.35rem;
    z-index: 40;
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 2rem;
    width: min(1100px, calc(100vw - 1.6rem));
    margin: 0 auto;
    padding: 0.45rem 0.2rem 0.65rem;
    background: transparent;
    border-bottom: 1px solid rgba(255, 255, 255, 0.12);
    transition:
      opacity 0.28s ease,
      transform 0.32s ease;
  }

  .brand-strip {
    display: flex;
    align-items: flex-end;
    color: #fff6a8;
    font-family: "Dotemp", "RetroByte", monospace;
    font-size: 0.98rem;
    letter-spacing: 0.08em;
    line-height: 1;
  }

  .brand-name {
    text-transform: uppercase;
  }

  .tab-bar {
    display: flex;
    gap: 0.55rem;
    flex-wrap: wrap;
    justify-content: flex-end;
    align-items: flex-end;
  }

  .tab-button {
    display: inline-flex;
    align-items: flex-end;
    min-width: auto;
    border: none;
    background: transparent;
    color: rgba(255, 232, 255, 0.72);
    border-radius: 999px;
    padding: 0.35rem 0.55rem;
    text-align: center;
    cursor: pointer;
    box-shadow: none;
    transition:
      color 0.2s ease,
      transform 0.2s ease,
      background-color 0.2s ease;
  }

  .tab-button.active {
    color: #fff6a8;
    background: rgba(255, 255, 255, 0.05);
  }

  .tab-label {
    display: block;
    font-size: 1rem;
    text-transform: uppercase;
    letter-spacing: 0.13em;
    line-height: 1;
    position: relative;
    top: 4px;
  }

  .tab-button:hover {
    color: #ffffff;
    transform: translateY(-1px);
  }

  .home-tab {
    padding: 0.05rem 0.3rem 0;
  }

  .home-icon {
    display: block;
    width: 28px;
    height: 28px;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
  }

  @media (max-width: 980px) {
    .top-chrome {
      flex-direction: column;
      align-items: stretch;
      top: 0.75rem;
      width: calc(100vw - 1rem);
    }

    .tab-bar {
      justify-content: flex-start;
    }
  }
</style>

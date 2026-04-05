<script lang="ts">
  import "katex/dist/katex.min.css";
  import { onMount } from "svelte";
  import AppHeader from "$lib/components/AppHeader.svelte";
  import HomePage from "$lib/components/pages/HomePage.svelte";
  import QuantumPage from "$lib/components/pages/QuantumPage.svelte";
  import LearnedPage from "$lib/components/pages/LearnedPage.svelte";
  import LivePage from "$lib/components/pages/LivePage.svelte";
  import { tabs, type TabId } from "$lib/app-tabs";

  let activeTab = $state<TabId>("home");
  let scrollY = $state(0);

  onMount(() => {
    const update = () => {
      scrollY = window.scrollY;
    };

    update();
    window.addEventListener("scroll", update, { passive: true });
    return () => window.removeEventListener("scroll", update);
  });

  function heroProgress(): number {
    return Math.min(scrollY / 420, 1);
  }

  function activeTitleScale(): number {
    return 1 - heroProgress() * 0.38;
  }

  function activeTitleOffset(): number {
    return heroProgress() * -180;
  }

  function headerRevealProgress(): number {
    if (activeTab !== "home") {
      return 1;
    }

    return Math.max(0, Math.min((heroProgress() - 0.32) / 0.38, 1));
  }

  function headerStyle(): string {
    const progress = headerRevealProgress();
    return `opacity: ${progress}; transform: translateY(${(1 - progress) * -18}px); pointer-events: ${progress < 0.05 ? "none" : "auto"};`;
  }
</script>

<svelte:head>
  <title>Quantum Annealing App</title>
</svelte:head>

<main class="app-shell">
  <AppHeader
    {tabs}
    {activeTab}
    headerStyle={headerStyle()}
    onSelectTab={(tab) => (activeTab = tab)}
  />

  {#if activeTab === "home"}
    <HomePage titleOffset={activeTitleOffset()} titleScale={activeTitleScale()} />
  {:else if activeTab === "quantum"}
    <QuantumPage />
  {:else if activeTab === "learned"}
    <LearnedPage />
  {:else}
    <LivePage />
  {/if}
</main>

<style>
  @font-face {
    font-family: "Quagey";
    src: url("/fonts/QuageyPixel-Regular.otf") format("opentype");
    font-display: swap;
  }

  @font-face {
    font-family: "Glastone";
    src: url("/fonts/GlastonePixel-Regular.otf") format("opentype");
    font-display: swap;
  }

  @font-face {
    font-family: "Dotemp";
    src: url("/fonts/Dotemp-8bit.ttf") format("truetype");
    font-display: swap;
  }

  @font-face {
    font-family: "RetroByte";
    src: url("/fonts/RetroByte.ttf") format("truetype");
    font-display: swap;
  }

  @font-face {
    font-family: "Kalesi";
    src: url("/fonts/KalesiPixel-Regular.otf") format("opentype");
    font-display: swap;
  }

  :global(html) {
    scroll-behavior: smooth;
  }

  :global(body) {
    margin: 0;
    color: #f4ddff;
    background:
      radial-gradient(circle at 20% 15%, rgba(255, 55, 140, 0.16), transparent 24%),
      radial-gradient(circle at 80% 22%, rgba(44, 155, 255, 0.18), transparent 28%),
      linear-gradient(180deg, #090910 0%, #0e0814 56%, #07070c 100%);
    font-family: "Dotemp", "RetroByte", monospace;
  }

  :global(p),
  :global(li) {
    font-family: "Kalesi", "Dotemp", "RetroByte", monospace;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  .app-shell {
    min-height: 100vh;
  }
</style>

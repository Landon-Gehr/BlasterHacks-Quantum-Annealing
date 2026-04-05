<script lang="ts">
  import { onMount } from "svelte";

  const FIXED_SYSTEM_SIZE = 32;
  const FIXED_DOMAIN_DISPLAY = "4π";
  const FIXED_QUBO_BITS = 3;
  const MIN_COEFF = -10;
  const MAX_COEFF = 10;

  type TabId = "home" | "quantum" | "learned" | "live";

  type Tab = {
    id: TabId;
    label: string;
    caption: string;
  };

  const tabs: Tab[] = [
    { id: "home", label: "Home", caption: "Landing" },
    { id: "quantum", label: "Quantum", caption: "Background" },
    { id: "learned", label: "Learned Solver", caption: "Model notes" },
    { id: "live", label: "Live Solver", caption: "Allen-Cahn demo" }
  ];

  let activeTab = $state<TabId>("home");
  let scrollY = $state(0);

  let coeffSinX = $state(1);
  let coeffSinY = $state(1);
  let coeffCosX = $state(1);
  let coeffCosY = $state(1);
  let coeffSinXY = $state(1);
  let coeffCosXY = $state(1);
  let boundaryFunction = $state("0");
  let status = $state("Retro preview ready.");

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

  function updateCoefficient(
    setter: (value: number) => void,
    event: Event
  ) {
    const target = event.currentTarget as HTMLInputElement;
    const parsed = Number(target.value);
    if (Number.isNaN(parsed)) {
      return;
    }

    setter(Math.max(MIN_COEFF, Math.min(MAX_COEFF, parsed)));
  }

  function resetInputs() {
    coeffSinX = 1;
    coeffSinY = 1;
    coeffCosX = 1;
    coeffCosY = 1;
    coeffSinXY = 1;
    coeffCosXY = 1;
    boundaryFunction = "0";
    status = "Poisson inputs reset to defaults.";
  }

  function loadDemo() {
    resetInputs();
    boundaryFunction = "sin(x) + cos(y)";
    status = "Loaded the placeholder Poisson demo.";
  }

  function generatePreview() {
    status = "Preview generated for the current Poisson setup.";
  }

  function coefficientString(value: number): string {
    return Number.isInteger(value) ? value.toString() : value.toFixed(2).replace(/\.?0+$/, "");
  }

  function term(value: number, basis: string): string {
    return `${coefficientString(value)}${basis}`;
  }

  function forcingExpression(): string {
    return [
      term(coeffSinX, " sin(x)"),
      term(coeffSinY, " sin(y)"),
      term(coeffCosX, " cos(x)"),
      term(coeffCosY, " cos(y)"),
      term(coeffSinXY, " sin(x)sin(y)"),
      term(coeffCosXY, " cos(x)cos(y)")
    ].join(" + ");
  }

  function quboDimension(): number {
    return FIXED_SYSTEM_SIZE * FIXED_QUBO_BITS;
  }
</script>

<svelte:head>
  <title>Quantum Annealing App</title>
</svelte:head>

<main class="app-shell">
  <header
    class:compact={heroProgress() > 0.12}
    class="top-chrome"
    style={`opacity: ${headerRevealProgress()}; transform: translateY(${(1 - headerRevealProgress()) * -18}px); pointer-events: ${headerRevealProgress() < 0.05 ? "none" : "auto"};`}
  >
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
          onclick={() => (activeTab = tab.id)}
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

  {#if activeTab === "home"}
    <section class="landing-shell">
      <section class="hero-stage">
        <div class="hero-grid"></div>
        <div class="hero-orb hero-orb-a"></div>
        <div class="hero-orb hero-orb-b"></div>

        <div
          class="hero-title-wrap"
          style={`transform: translateY(${activeTitleOffset()}px) scale(${activeTitleScale()});`}
        >
          <p class="hero-kicker">Simulating PDEs with</p>
          <h1>QUANTUM<br />ANNEALING</h1>
          <p class="hero-subtitle">
            (or not)
          </p>
        </div>
      </section>

      <section class="story-frame">
        <article class="about-card">
          <p class="window-tag">ABOUT US</p>
          <h2>Who are we?</h2>
          <p>
            Byron Selvage: BS in AMS. MS in AMS in progress...
          </p>
          <p>
            Landon Gehr: BS in CS. MS in CS in progress...
          </p>
          <p>
            Kyle Sperber: BS in AMS and Physics. MS in AMS and Physics done?
          </p>
          <p>
            Melody Goldanloo: BS in CS and MS in QE Software in progress...
          </p>
          <a
            class="github-link"
            href="https://github.com/Landon-Gehr/BlasterHacks-Quantum-Annealing"
            target="_blank"
            rel="noreferrer"
            aria-label="Open GitHub repository"
          >
            <svg viewBox="0 0 24 24" aria-hidden="true">
              <path
                d="M12 2C6.48 2 2 6.59 2 12.25c0 4.53 2.87 8.37 6.84 9.73.5.1.68-.22.68-.49 0-.24-.01-1.04-.01-1.88-2.78.62-3.37-1.21-3.37-1.21-.46-1.2-1.11-1.52-1.11-1.52-.91-.64.07-.63.07-.63 1 .07 1.53 1.06 1.53 1.06.9 1.57 2.35 1.12 2.92.86.09-.67.35-1.12.63-1.38-2.22-.26-4.55-1.14-4.55-5.09 0-1.13.39-2.05 1.03-2.77-.1-.26-.45-1.3.1-2.71 0 0 .85-.28 2.8 1.06A9.5 9.5 0 0 1 12 7.1c.85 0 1.7.12 2.5.36 1.95-1.34 2.8-1.06 2.8-1.06.56 1.41.21 2.45.1 2.71.64.72 1.03 1.64 1.03 2.77 0 3.96-2.33 4.82-4.56 5.08.36.32.68.94.68 1.9 0 1.37-.01 2.47-.01 2.81 0 .27.18.6.69.49A10.26 10.26 0 0 0 22 12.25C22 6.59 17.52 2 12 2Z"
              />
            </svg>
            <span class="github-tooltip">GitHub repo</span>
          </a>
        </article>
      </section>
    </section>
  {:else if activeTab === "quantum"}
    <section class="page-shell">
      <article class="retro-window wide">
        <div class="window-bar">
          <span>Quantum / Background</span>
          <span class="window-controls">_ □ ×</span>
        </div>
        <div class="window-body">
          <div class="window-copy">
            <p class="section-kicker">Theory track</p>
            <h3>QUBO → Ising → quantum annealing</h3>
            <p>
              This page will explain how the PDE-inspired optimization problem is encoded into a
              QUBO, translated into Ising form, and then handed to a quantum annealing style
              solver.
            </p>
            <p>
              It should also carry the short proof-of-concept story: the theory works, but
              simulating the relevant quantum processes on classical hardware is still
              computationally impractical to scale.
            </p>
          </div>
          <div class="pixel-panel">
            <span class="pixel-label">PLACEHOLDER</span>
            <div class="pixel-display">
              <span>5×5</span>
              <span>SYSTEM</span>
            </div>
          </div>
        </div>
      </article>

      <article class="retro-window duo">
        <div class="window-bar">
          <span>Results snapshot</span>
          <span class="window-controls">_ □ ×</span>
        </div>
        <div class="window-body split">
          <div>
            <p class="section-kicker">What to show here later</p>
            <ul>
              <li>5×5 system and solution</li>
              <li>Improved Poisson result</li>
              <li>Short claim about limited scaling on classical hardware</li>
            </ul>
          </div>
          <div class="pixel-card">
            <div class="pixel-badge">RETRO PAGE</div>
            <p>Drop in screenshots or figures later.</p>
          </div>
        </div>
      </article>
    </section>
  {:else if activeTab === "learned"}
    <section class="page-shell">
      <article class="retro-window full">
        <div class="window-bar pink">
          <span>Learned Solver</span>
          <span class="window-controls">_ □ ×</span>
        </div>
        <div class="window-body stacked">
          <p class="section-kicker">Placeholder</p>
          <h2>Landon’s section goes here</h2>
          <p>
            This page is reserved for the learned-solver narrative and whatever “improves halfway
            solved solution?” turns into once you have the final details.
          </p>
          <div class="pixel-card alt">
            <p>Planned content:</p>
            <ul>
              <li>What the learned solver changes</li>
              <li>Where it helps relative to the baseline</li>
              <li>Illustrations or solver-state comparisons</li>
            </ul>
          </div>
        </div>
      </article>
    </section>
  {:else}
    <section class="page-shell">
      <article class="retro-window full">
        <div class="window-bar blue">
          <span>Live Solver / Allen-Cahn</span>
          <span class="window-controls">_ □ ×</span>
        </div>
        <div class="window-body stacked">
          <div class="window-copy">
            <p class="section-kicker">Interactive demo</p>
            <h2>Draw an initial condition, then watch it evolve</h2>
            <p>
              The long-term goal here is a live Allen-Cahn demo driven by a classical time-stepping
              solver, with discussion of what the linear solves enable and what it would cost to
              push the same workflow onto a QPU.
            </p>
          </div>
        </div>
      </article>

      <article class="retro-window full">
        <div class="window-bar orange">
          <span>Current Poisson setup placeholder</span>
          <span class="window-controls">_ □ ×</span>
        </div>
        <div class="window-body stacked">
          <div class="compact-intro">
            <p class="section-kicker">Existing workflow</p>
            <h3>Retro-window version of the current form</h3>
            <p>
              This is where the current Poisson setup lives for now, inside a full-width retro page
              rather than a utility dashboard split.
            </p>
          </div>

          <section class="control-block">
            <h4>Forcing function</h4>
            <div class="formula-editor">
              <span>f(x, y) =</span>
              <input
                type="number"
                min={MIN_COEFF}
                max={MAX_COEFF}
                step="any"
                bind:value={coeffSinX}
                onchange={(event) => updateCoefficient((value) => (coeffSinX = value), event)}
              />
              <span>sin(x) +</span>
              <input
                type="number"
                min={MIN_COEFF}
                max={MAX_COEFF}
                step="any"
                bind:value={coeffSinY}
                onchange={(event) => updateCoefficient((value) => (coeffSinY = value), event)}
              />
              <span>sin(y) +</span>
              <input
                type="number"
                min={MIN_COEFF}
                max={MAX_COEFF}
                step="any"
                bind:value={coeffCosX}
                onchange={(event) => updateCoefficient((value) => (coeffCosX = value), event)}
              />
              <span>cos(x) +</span>
              <input
                type="number"
                min={MIN_COEFF}
                max={MAX_COEFF}
                step="any"
                bind:value={coeffCosY}
                onchange={(event) => updateCoefficient((value) => (coeffCosY = value), event)}
              />
              <span>cos(y) +</span>
              <input
                type="number"
                min={MIN_COEFF}
                max={MAX_COEFF}
                step="any"
                bind:value={coeffSinXY}
                onchange={(event) => updateCoefficient((value) => (coeffSinXY = value), event)}
              />
              <span>sin(x)sin(y) +</span>
              <input
                type="number"
                min={MIN_COEFF}
                max={MAX_COEFF}
                step="any"
                bind:value={coeffCosXY}
                onchange={(event) => updateCoefficient((value) => (coeffCosXY = value), event)}
              />
              <span>cos(x)cos(y)</span>
            </div>
          </section>

          <section class="control-grid">
            <div class="control-block">
              <h4>Boundary function</h4>
              <label class="field">
                <span>g on ∂Ω</span>
                <input type="text" bind:value={boundaryFunction} />
              </label>
            </div>

            <div class="control-block preview-block">
              <h4>Preview</h4>
              <p>Ω = (0, {FIXED_DOMAIN_DISPLAY}) × (0, {FIXED_DOMAIN_DISPLAY})</p>
              <p>−Δu(x, y) = {forcingExpression()} in Ω</p>
              <p>u(x, y) = {boundaryFunction} on ∂Ω</p>
              <p>Internal QUBO dimension: {quboDimension()}</p>
              <p class="status-line">{status}</p>
            </div>
          </section>

          <div class="action-row">
            <button type="button" class="primary" onclick={generatePreview}>Generate Preview</button>
            <button type="button" onclick={loadDemo}>Load Demo</button>
            <button type="button" onclick={resetInputs}>Reset</button>
          </div>
        </div>
      </article>
    </section>
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

  :global(button),
  :global(input) {
    font: inherit;
  }

  .app-shell {
    min-height: 100vh;
  }

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

  .landing-shell,
  .page-shell {
    padding: 1.25rem;
  }

  .hero-stage {
    position: relative;
    min-height: 110vh;
    display: grid;
    place-items: center;
    overflow: hidden;
  }

  .hero-grid {
    position: absolute;
    inset: 0;
    background-image:
      linear-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255, 255, 255, 0.05) 1px, transparent 1px);
    background-size: 56px 56px;
    opacity: 0.28;
  }

  .hero-orb {
    position: absolute;
    border-radius: 999px;
    filter: blur(20px);
    opacity: 0.7;
  }

  .hero-orb-a {
    width: 14rem;
    height: 14rem;
    background: rgba(255, 84, 162, 0.34);
    top: 15%;
    left: 8%;
  }

  .hero-orb-b {
    width: 16rem;
    height: 16rem;
    background: rgba(49, 143, 255, 0.3);
    right: 10%;
    bottom: 14%;
  }

  .hero-title-wrap {
    position: sticky;
    top: 7rem;
    z-index: 2;
    width: min(72rem, calc(100vw - 3rem));
    padding: 1.8rem;
    transform-origin: top center;
    transition: transform 0.14s linear;
  }

  .hero-kicker,
  .section-kicker,
  .window-tag {
    margin: 0 0 0.6rem;
    color: #ff8fcb;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 1.5rem;
  }

  .hero-title-wrap h1 {
    margin: 0;
    font-family: "Quagey", "Dotemp", monospace;
    font-size: clamp(4.8rem, 82vw, 11.5rem);
    line-height: 0.88;
    letter-spacing: 0.06em;
    color: #fff6a8;
    text-shadow:
      0 0 18px rgba(255, 153, 0, 0.18),
      4px 4px 0 #05050a;
  }

  .hero-subtitle {
    width: min(30rem, 100%);
    margin: 1.2rem 0 0;
    color: #f7dfff;
    font-size: 1.1rem;
    line-height: 1.5;
  }

  .github-link {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    margin-top: 1.2rem;
    padding: 0;
    border: none;
    background: transparent;
    color: #fff3ff;
    text-decoration: none;
    transition:
      transform 0.2s ease,
      color 0.2s ease;
  }

  .github-link:hover {
    transform: translateY(-1px);
    color: #fff6a8;
  }

  .github-link svg {
    width: 64px;
    height: 64px;
    fill: currentColor;
  }

  .github-tooltip {
    opacity: 0;
    transform: translateX(-4px);
    transition:
      opacity 0.18s ease,
      transform 0.18s ease;
    color: currentColor;
    font-size: 0.78rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    pointer-events: none;
  }

  .github-link:hover .github-tooltip,
  .github-link:focus-visible .github-tooltip {
    opacity: 1;
    transform: translateX(0);
  }

  .story-frame,
  .page-shell {
    display: grid;
    gap: 1.15rem;
  }

  .about-card,
  .retro-window {
    border: 2px solid #2b2340;
    background:
      linear-gradient(180deg, rgba(20, 16, 28, 0.96), rgba(10, 10, 16, 0.96));
    box-shadow:
      0 0 0 2px rgba(255, 255, 255, 0.02) inset,
      8px 8px 0 #06050a;
  }

  .about-card {
    padding: 1.3rem;
  }

  .about-card h2,
  .window-copy h3,
  .retro-window h2,
  .retro-window h3 {
    margin: 0.25rem 0 0.75rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.05;
  }

  .retro-window.full,
  .retro-window.wide,
  .retro-window.duo {
    width: 100%;
  }

  .window-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.55rem 0.8rem;
    border-bottom: 2px solid #2b2340;
    background: linear-gradient(90deg, #2f2750, #19162d);
    color: #fff3ff;
    text-transform: uppercase;
    font-size: 0.8rem;
    letter-spacing: 0.08em;
  }

  .window-bar.pink {
    background: linear-gradient(90deg, #ef70b7, #9c3f72);
    color: #240914;
  }

  .window-bar.blue {
    background: linear-gradient(90deg, #6ab6ff, #2f5cff);
    color: #081327;
  }

  .window-bar.orange {
    background: linear-gradient(90deg, #ffb347, #ff6e42);
    color: #241105;
  }

  .window-controls {
    letter-spacing: 0.2em;
  }

  .window-body {
    padding: 1rem 1.1rem 1.15rem;
  }

  .window-body.stacked {
    display: grid;
    gap: 1rem;
  }

  .window-body.split {
    display: grid;
    gap: 1rem;
    grid-template-columns: minmax(0, 1.2fr) minmax(14rem, 0.8fr);
  }

  .pixel-panel,
  .pixel-card,
  .control-block {
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.035);
    padding: 0.9rem;
  }

  .pixel-panel {
    display: grid;
    align-content: center;
    min-height: 12rem;
    background:
      linear-gradient(180deg, rgba(255, 211, 97, 0.14), rgba(255, 72, 130, 0.08)),
      rgba(255, 255, 255, 0.03);
  }

  .pixel-label {
    display: inline-block;
    margin-bottom: 0.8rem;
    color: #84c3ff;
    font-size: 0.72rem;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }

  .pixel-display {
    font-family: "Glastone", "Dotemp", monospace;
    font-size: clamp(2rem, 5vw, 3.8rem);
    line-height: 0.9;
    color: #ffe972;
  }

  .pixel-card.alt {
    background: rgba(92, 190, 255, 0.08);
  }

  .compact-intro p,
  .window-copy p,
  .about-card p,
  .retro-window p,
  .retro-window li {
    color: #efdaf9;
    line-height: 1.65;
  }

  .page-shell {
    padding: 1.25rem;
  }

  .control-grid {
    display: grid;
    gap: 1rem;
    grid-template-columns: minmax(0, 1fr) minmax(18rem, 0.9fr);
  }

  .formula-editor {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.55rem;
    margin-top: 0.75rem;
  }

  .formula-editor input,
  .field input {
    width: 4.8rem;
    border: 2px solid #332b4b;
    background: #0c0b14;
    color: #ffeefc;
    padding: 0.55rem 0.6rem;
    outline: none;
  }

  .field {
    display: grid;
    gap: 0.6rem;
  }

  .field input {
    width: 100%;
  }

  .action-row {
    display: flex;
    gap: 0.8rem;
    flex-wrap: wrap;
  }

  .action-row button {
    border: 2px solid #38294f;
    background: linear-gradient(180deg, #181420, #100d16);
    color: #fbeeff;
    padding: 0.7rem 1rem;
    cursor: pointer;
    box-shadow: 4px 4px 0 #06050a;
  }

  .action-row button.primary {
    background: linear-gradient(180deg, #ff4aa2, #b8307d);
    color: #250812;
  }

  .preview-block p,
  .status-line {
    margin: 0.35rem 0;
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

    .window-body.split,
    .control-grid {
      grid-template-columns: minmax(0, 1fr);
    }

    .hero-title-wrap {
      width: calc(100vw - 2rem);
      top: 6.5rem;
      padding: 1rem;
    }

    .landing-shell,
    .page-shell {
      padding: 0.8rem;
    }
  }
</style>

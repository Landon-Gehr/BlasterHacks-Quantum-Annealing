<script lang="ts">
  const FIXED_SYSTEM_SIZE = 32;
  const FIXED_DOMAIN_DISPLAY = "4π";
  const FIXED_QUBO_BITS = 3;
  const MIN_COEFF = -10;
  const MAX_COEFF = 10;

  type TabId = "background" | "comparison" | "allen-cahn" | "poisson" | "about";

  type Tab = {
    id: TabId;
    label: string;
    eyebrow: string;
  };

  const tabs: Tab[] = [
    { id: "background", label: "Background", eyebrow: "Context" },
    { id: "comparison", label: "Classic vs. Quantum", eyebrow: "Comparison" },
    { id: "allen-cahn", label: "Allen-Cahn", eyebrow: "Interactive" },
    { id: "poisson", label: "Poisson Setup", eyebrow: "Current workflow" },
    { id: "about", label: "About", eyebrow: "More Info" }
  ];

  let activeTab = $state<TabId>("poisson");

  let coeffSinX = $state(1);
  let coeffSinY = $state(1);
  let coeffCosX = $state(1);
  let coeffCosY = $state(1);
  let coeffSinXY = $state(1);
  let coeffCosXY = $state(1);
  let boundaryFunction = $state("0");
  let status = $state("Preview ready.");

  function clampCoefficient(value: number): number {
    return Math.max(MIN_COEFF, Math.min(MAX_COEFF, value));
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

    setter(clampCoefficient(parsed));
  }

  function resetPoissonInputs() {
    coeffSinX = 1;
    coeffSinY = 1;
    coeffCosX = 1;
    coeffCosY = 1;
    coeffSinXY = 1;
    coeffCosXY = 1;
    boundaryFunction = "0";
    status = "Inputs reset to the default Poisson example.";
  }

  function loadDemo() {
    resetPoissonInputs();
    boundaryFunction = "sin(x) + cos(y)";
    status = "Loaded the demo forcing coefficients and boundary function.";
  }

  function generatePreview() {
    status = "Preview generated from the current Poisson setup.";
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
  <title>BlasterHacks Quantum Annealing</title>
</svelte:head>

<main class="app-shell">
  <section class="hero">
    <p class="eyebrow">BlasterHacks Quantum Annealing</p>
    <h1>Quantum Annealing + ML 4 PDEs B) </h1>
    <p class="hero-copy">
      Separate the mathematical background from the interactive setup. Keep the current
      Poisson workflow intact while reserving a dedicated space for the Allen-Cahn view.
    </p>
  </section>

  <nav class="tab-bar" aria-label="Application sections">
    {#each tabs as tab}
      <button
        class:active={activeTab === tab.id}
        class="tab-button"
        type="button"
        onclick={() => (activeTab = tab.id)}
      >
        <span class="tab-eyebrow">{tab.eyebrow}</span>
        <span class="tab-label">{tab.label}</span>
      </button>
    {/each}
  </nav>

  {#if activeTab === "background"}
    <section class="page-grid single">
      <article class="panel prose">
        <p class="section-label">Background</p>
        <h2>What this app is organizing</h2>
        <p>
          The current interface fixes a Poisson problem on the domain
          <strong> Ω = (0, 4π) × (0, 4π)</strong> with a single boundary function
          <strong> g on ∂Ω</strong>. The user only adjusts the forcing-function coefficients and
          the boundary expression.
        </p>

        <div class="callout">
          <p class="callout-title">Model problem</p>
          <p>−Δu(x, y) = f(x, y) in Ω</p>
          <p>u(x, y) = g on ∂Ω</p>
        </div>

        <h3>Why the forcing function is restricted</h3>
        <p>
          Instead of accepting an arbitrary symbolic expression, the app uses a fixed trig basis.
          That keeps the interaction simple and makes downstream discretization and QUBO generation
          more controlled.
        </p>

        <div class="formula-block">
          <p>f(x, y) = a sin(x) + b sin(y) + c cos(x) + d cos(y)</p>
          <p>+ e sin(x)sin(y) + f cos(x)cos(y)</p>
        </div>

        <h3>What stays fixed</h3>
        <p>
          The domain and discretization size stay fixed in this version of the app. That lets the
          UI focus on problem specification rather than exposing lower-level numerical parameters.
        </p>
      </article>
    </section>
  {:else if activeTab === "allen-cahn"}
    <section class="page-grid single">
      <article class="panel prose">
        <p class="section-label">Allen-Cahn</p>
        <h2>Placeholder for the interactive Allen-Cahn workflow</h2>
        <p>
          This tab is ready to hold the separate Allen-Cahn interface once the model details are
          finalized.
        </p>

        <div class="callout muted">
          <p class="callout-title">Reserved scope</p>
          <p>Parameter controls for the Allen-Cahn equation</p>
          <p>Visualization of the evolving field or steady-state profile</p>
          <p>Notes connecting the Allen-Cahn model to the annealing pipeline</p>
        </div>
      </article>
    </section>
  {:else}
    <section class="page-grid">
      <article class="panel">
        <p class="section-label">Poisson Setup</p>
        <h2>Current interactive configuration</h2>
        <p class="meta-line">
          Fixed domain Ω = (0, {FIXED_DOMAIN_DISPLAY}) × (0, {FIXED_DOMAIN_DISPLAY}), fixed n =
          {FIXED_SYSTEM_SIZE}
        </p>

        <section class="block">
          <div class="block-header">
            <h3>Forcing Function</h3>
            <p>Edit coefficients directly in the formula. Each coefficient is clamped to [-10, 10].</p>
          </div>

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

        <section class="block">
          <div class="block-header">
            <h3>Boundary Function</h3>
            <p>Single Dirichlet boundary function used across the whole boundary ∂Ω.</p>
          </div>

          <label class="field">
            <span>g on ∂Ω</span>
            <input type="text" bind:value={boundaryFunction} />
          </label>
        </section>

        <div class="action-row">
          <button type="button" class="primary" onclick={generatePreview}>Generate Preview</button>
          <button type="button" onclick={loadDemo}>Load Demo</button>
          <button type="button" onclick={resetPoissonInputs}>Reset</button>
        </div>
      </article>

      <aside class="panel preview">
        <p class="section-label">Preview</p>
        <h2>Problem statement</h2>

        <div class="callout">
          <p>Ω = (0, {FIXED_DOMAIN_DISPLAY}) × (0, {FIXED_DOMAIN_DISPLAY})</p>
          <p>−Δu(x, y) = {forcingExpression()} in Ω</p>
          <p>u(x, y) = {boundaryFunction} on ∂Ω</p>
        </div>

        <section class="block compact">
          <div class="block-header">
            <h3>Discretization</h3>
          </div>
          <p>Fixed grid size: n = {FIXED_SYSTEM_SIZE}</p>
          <p>Internal QUBO dimension: {quboDimension()}</p>
        </section>

        <p class="status">{status}</p>
      </aside>
    </section>
  {/if}
</main>

<style>
  :global(body) {
    margin: 0;
    font-family:
      "Iowan Old Style", "Palatino Linotype", "Book Antiqua", Palatino, Georgia, serif;
    color: #f5efe0;
    background:
      radial-gradient(circle at top, rgba(215, 141, 63, 0.18), transparent 34%),
      linear-gradient(180deg, #151313 0%, #0d0f14 100%);
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  .app-shell {
    min-height: 100vh;
    padding: 2rem 2rem 2.5rem;
    box-sizing: border-box;
  }

  .hero {
    max-width: 62rem;
    margin-bottom: 1.5rem;
  }

  .eyebrow,
  .section-label,
  .tab-eyebrow {
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    font-size: 0.72rem;
    color: #d1a05a;
  }

  .hero h1,
  .panel h2,
  .prose h2 {
    margin: 0.25rem 0 0;
    font-size: clamp(2rem, 4vw, 3.4rem);
    line-height: 1.02;
    font-weight: 600;
  }

  .hero-copy,
  .meta-line,
  .block-header p,
  .prose p,
  .status {
    color: #d3d0c7;
  }

  .hero-copy {
    max-width: 46rem;
    margin: 0.9rem 0 0;
    font-size: 1.02rem;
    line-height: 1.6;
  }

  .tab-bar {
    display: flex;
    gap: 0.85rem;
    margin-bottom: 1.5rem;
    flex-wrap: wrap;
  }

  .tab-button {
    border: 1px solid rgba(236, 215, 177, 0.16);
    border-radius: 1.15rem;
    background: rgba(18, 20, 28, 0.72);
    color: #efe8d8;
    padding: 0.9rem 1rem;
    min-width: 12rem;
    text-align: left;
    cursor: pointer;
    transition:
      transform 0.2s ease,
      border-color 0.2s ease,
      background 0.2s ease;
  }

  .tab-button:hover {
    transform: translateY(-1px);
    border-color: rgba(236, 215, 177, 0.36);
  }

  .tab-button.active {
    background: linear-gradient(135deg, rgba(160, 90, 37, 0.95), rgba(74, 39, 28, 0.95));
    border-color: rgba(255, 222, 174, 0.48);
  }

  .tab-label {
    display: block;
    margin-top: 0.2rem;
    font-size: 1.1rem;
    font-weight: 600;
  }

  .page-grid {
    display: grid;
    grid-template-columns: minmax(0, 1.6fr) minmax(20rem, 0.9fr);
    gap: 1.25rem;
    align-items: start;
  }

  .page-grid.single {
    grid-template-columns: minmax(0, 1fr);
  }

  .panel {
    border: 1px solid rgba(236, 215, 177, 0.13);
    border-radius: 1.6rem;
    background:
      linear-gradient(180deg, rgba(27, 30, 38, 0.95), rgba(15, 17, 23, 0.95));
    padding: 1.4rem;
    box-shadow: 0 24px 80px rgba(0, 0, 0, 0.28);
  }

  .prose {
    max-width: 58rem;
  }

  .prose h3 {
    margin-bottom: 0.45rem;
    font-size: 1.15rem;
  }

  .callout,
  .formula-block,
  .block {
    border-radius: 1.1rem;
    background: rgba(250, 245, 232, 0.04);
    border: 1px solid rgba(236, 215, 177, 0.12);
    padding: 1rem 1.05rem;
  }

  .callout {
    margin: 1rem 0;
  }

  .callout-title {
    margin: 0 0 0.5rem;
    font-weight: 700;
    color: #f6d49d;
  }

  .muted {
    opacity: 0.88;
  }

  .formula-block {
    font-family: "STIX Two Text", "Times New Roman", serif;
    font-size: 1.04rem;
  }

  .block + .block {
    margin-top: 1rem;
  }

  .block-header h3 {
    margin: 0;
    font-size: 1.15rem;
  }

  .block-header p {
    margin: 0.3rem 0 0;
    font-size: 0.96rem;
    line-height: 1.5;
  }

  .formula-editor {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.55rem;
    margin-top: 0.8rem;
  }

  .formula-editor input,
  .field input {
    border: 1px solid rgba(236, 215, 177, 0.18);
    border-radius: 0.8rem;
    background: rgba(11, 13, 18, 0.8);
    color: #f8f1e2;
    padding: 0.55rem 0.7rem;
    outline: none;
  }

  .formula-editor input {
    width: 4.9rem;
  }

  .field {
    display: grid;
    gap: 0.55rem;
    margin-top: 0.75rem;
  }

  .field span {
    color: #f0e3c9;
  }

  .action-row {
    display: flex;
    gap: 0.8rem;
    flex-wrap: wrap;
    margin-top: 1.25rem;
  }

  .action-row button {
    border: 1px solid rgba(236, 215, 177, 0.18);
    border-radius: 999px;
    background: rgba(13, 15, 20, 0.82);
    color: #f8f1e2;
    padding: 0.7rem 1rem;
    cursor: pointer;
  }

  .action-row button.primary {
    background: linear-gradient(135deg, #c46d35, #8e4326);
    border-color: rgba(255, 211, 153, 0.26);
  }

  .preview .callout p,
  .formula-block p,
  .callout p {
    margin: 0.2rem 0;
  }

  .compact {
    margin-top: 1rem;
  }

  .status {
    margin: 1rem 0 0;
    font-size: 0.95rem;
  }

  @media (max-width: 920px) {
    .app-shell {
      padding: 1.2rem;
    }

    .page-grid {
      grid-template-columns: minmax(0, 1fr);
    }

    .tab-button {
      min-width: unset;
      flex: 1 1 12rem;
    }
  }
</style>

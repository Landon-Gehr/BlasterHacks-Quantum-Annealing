<script lang="ts">
  import Katex from "svelte-katex";

  const FIXED_SYSTEM_SIZE = 32;
  const FIXED_DOMAIN_DISPLAY = "4π";
  const FIXED_QUBO_BITS = 3;
  const MIN_COEFF = -10;
  const MAX_COEFF = 10;

  let coeffSinX = $state(1);
  let coeffSinY = $state(1);
  let coeffCosX = $state(1);
  let coeffCosY = $state(1);
  let coeffSinXY = $state(1);
  let coeffCosXY = $state(1);
  let boundaryFunction = $state("0");
  let status = $state("Retro preview ready.");

  function updateCoefficient(setter: (value: number) => void, event: Event) {
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

  function latexifyExpression(expression: string): string {
    return expression
      .trim()
      .replace(/\bpi\b/g, "\\pi")
      .replace(/\bsin\b/g, "\\sin")
      .replace(/\bcos\b/g, "\\cos")
      .replace(/\*/g, " ");
  }

  function forcingLatex(): string {
    return [
      term(coeffSinX, " \\sin(x)"),
      term(coeffSinY, " \\sin(y)"),
      term(coeffCosX, " \\cos(x)"),
      term(coeffCosY, " \\cos(y)"),
      term(coeffSinXY, " \\sin(x)\\sin(y)"),
      term(coeffCosXY, " \\cos(x)\\cos(y)")
    ].join(" + ");
  }

  function domainLatex(): string {
    return `\\Omega = (0, 4\\pi) \\times (0, 4\\pi)`;
  }

  function poissonLatex(): string {
    return `-\\Delta u(x,y) = ${forcingLatex()} \\quad \\text{in } \\Omega`;
  }

  function boundaryLatex(): string {
    const rhs = latexifyExpression(boundaryFunction) || "0";
    return `u(x,y) = ${rhs} \\quad \\text{on } \\partial\\Omega`;
  }

  function quboLatex(): string {
    return `\\text{QUBO dimension} = ${FIXED_SYSTEM_SIZE} \\cdot ${FIXED_QUBO_BITS} = ${quboDimension()}`;
  }
</script>

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
          solver, with discussion of what the linear solves enable and what it would cost to push
          the same workflow onto a QPU.
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
          <span class="formula-label"><Katex>f(x,y)=</Katex></span>
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
            <span class="formula-label"><Katex>{"g \\text{ on } \\partial \\Omega"}</Katex></span>
            <input type="text" bind:value={boundaryFunction} />
          </label>
        </div>

        <div class="control-block preview-block">
          <h4>Preview</h4>
          <div class="equation-stack">
            <div class="equation-line"><Katex>{domainLatex()}</Katex></div>
            <div class="equation-line"><Katex>{poissonLatex()}</Katex></div>
            <div class="equation-line"><Katex>{boundaryLatex()}</Katex></div>
            <div class="equation-line"><Katex>{quboLatex()}</Katex></div>
          </div>
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

<style>
  .page-shell {
    display: grid;
    gap: 1.15rem;
    padding: 1.25rem;
  }

  .retro-window {
    width: 100%;
    border: 2px solid #2b2340;
    background:
      linear-gradient(180deg, rgba(20, 16, 28, 0.96), rgba(10, 10, 16, 0.96));
    box-shadow:
      0 0 0 2px rgba(255, 255, 255, 0.02) inset,
      8px 8px 0 #06050a;
  }

  .window-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.55rem 0.8rem;
    border-bottom: 2px solid #2b2340;
    color: #081327;
    text-transform: uppercase;
    font-size: 0.8rem;
    letter-spacing: 0.08em;
  }

  .window-bar.blue {
    background: linear-gradient(90deg, #6ab6ff, #2f5cff);
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

  .section-kicker {
    margin: 0 0 0.6rem;
    color: #ff8fcb;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 1.5rem;
  }

  .window-copy h2,
  .compact-intro h3 {
    margin: 0.25rem 0 0.75rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.05;
  }

  .compact-intro p,
  .window-copy p,
  p {
    color: #efdaf9;
    line-height: 1.65;
  }

  .control-grid {
    display: grid;
    gap: 1rem;
    grid-template-columns: minmax(0, 1fr) minmax(18rem, 0.9fr);
  }

  .control-block {
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.035);
    padding: 0.9rem;
  }

  .formula-editor {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.55rem;
    margin-top: 0.75rem;
  }

  .formula-label {
    display: inline-flex;
    align-items: center;
    color: #fff4c7;
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

  .preview-block p,
  .status-line {
    margin: 0.35rem 0;
  }

  .equation-stack {
    display: grid;
    gap: 0.7rem;
  }

  .equation-line {
    overflow-x: auto;
    padding-bottom: 0.1rem;
    color: #fff4c7;
  }

  :global(.preview-block .katex) {
    font-size: 1.04rem;
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

  @media (max-width: 980px) {
    .page-shell {
      padding: 0.8rem;
    }

    .control-grid {
      grid-template-columns: minmax(0, 1fr);
    }
  }
</style>

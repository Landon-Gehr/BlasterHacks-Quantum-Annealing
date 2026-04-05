<script lang="ts">
  import katex from "katex";

  function renderBlockMath(expression: string): string {
    return katex.renderToString(expression, {
      displayMode: true,
      throwOnError: false,
      strict: "ignore"
    });
  }

  function formatNumber(value: number): string {
    if (Number.isInteger(value)) {
      return value.toString();
    }

    return value.toFixed(6).replace(/\.?0+$/, "");
  }

  function vectorToLatex(values: number[]): string {
    return String.raw`\begin{bmatrix}${values.map(formatNumber).join(String.raw`\\`)}\end{bmatrix}`;
  }

  function matrixToLatex(values: number[][]): string {
    return String.raw`\begin{bmatrix}${values
      .map((row) => row.map(formatNumber).join(" & "))
      .join(String.raw`\\`)}\end{bmatrix}`;
  }

  const pipeline = [
    {
      step: "01",
      title: "PDE",
      label: "Continuous problem",
      math: String.raw`-\Delta u = f \text{ on } \Omega, \qquad u|_{\partial \Omega} = 0`,
      objective: String.raw`\min_{u \in \mathbb{R}^{\Omega}} \; \mathcal{L}(u)`
    },
    {
      step: "02",
      title: "Linear System",
      label: "Discretize the operators",
      math: String.raw`A x = b`,
      objective: String.raw`\min_{x \in \mathbb{R}^n} \; \|A x - b\|_2^2`
    },
    {
      step: "03",
      title: "QUBO",
      label: "Encode continuous values in bits",
      math: String.raw`x \approx x(q), \qquad q \in \{0,1\}^N`,
      objective: String.raw`\min_{q \in \{0,1\}^N} \; q^\top Q q`
    },
    {
      step: "04",
      title: "Ising",
      label: "Transform to spins in {-1, 1}",
      math: String.raw`q_i = \tfrac{1 + s_i}{2}, \qquad s_i \in \{-1,1\}`,
      objective: String.raw`\min_{s \in \{-1,1\}^N} \; \sum_{i<j} J_{ij} s_i s_j + \sum_i h_i s_i + c`
    }
  ];

  const solverStages = [
    String.raw`E(x) = \|A x - b\|_2^2`,
    String.raw`x_i \approx u_{\min} + \sum_{k=0}^{K-1} w_k q_{ik}`,
    String.raw`E(q) = q^\top Q q`,
    String.raw`E(s) = \sum_{i<j} J_{ij} s_i s_j + \sum_i h_i s_i + c`
  ];

  const solvedSystem = {
    a: [
      [1.0, 3.0, 0.0, 2.0, 0.0],
      [0.0, 0.0, 4.0, 6.0, 5.0],
      [3.0, 1.0, 3.0, 0.0, 2.0],
      [1.0, 0.0, 0.0, 0.0, 3.0],
      [2.0, 0.0, 5.0, 1.0, 0.0]
    ],
    b: [15.0, 61.0, 24.0, 16.0, 21.0],
    xRecovered: [1.1697191697191698, 1.8876678876678876, 2.8156288156288154, 4.075702075702075, 4.835164835164836],
    xTrue: [1, 2, 3, 4, 5],
    encoding: String.raw`u_{\min} = 0, \quad u_{\max} = 10, \quad k = 12`
  };

  const successPanels = [
    {
      title: "5x5 dense test",
      math: String.raw`\min_{x \in \mathbb{R}^5} \|A x - b\|_2^2`
    },
    {
      title: "Coarse Poisson solve",
      math: String.raw`-\Delta u = f, \qquad u_{QA} \approx u_{\mathrm{manufactured}}`
    }
  ];

  const limitations = [
    String.raw`x_i \text{ is quantized to } k \text{ bits}`,
    String.raw`N_{\mathrm{binary}} = k \cdot N_{\mathrm{physical}}`,
    String.raw`\text{classical simulation cost} \gg \text{practical interactive cost}`,
    String.raw`\text{no QPU access} \Rightarrow \text{theory validation, not hardware advantage}`
  ];

  const heroEquation = String.raw`\text{PDE} \to A x = b \to q^\top Q q \to (J, h)`;
  const systemEquation = String.raw`A x = b`;
  const solvedMatrixA = matrixToLatex(solvedSystem.a);
  const solvedVectorB = vectorToLatex(solvedSystem.b);
  const solvedRecoveredX = vectorToLatex(solvedSystem.xRecovered);
  const solvedTrueX = vectorToLatex(solvedSystem.xTrue);
</script>

<section class="page-shell">
  <article class="retro-window hero">
    <div class="window-bar gold">
      <span>Quantum Pipeline / Theory</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body hero-layout">
      <div class="hero-copy">
        <p class="section-kicker">From PDE to annealing</p>
        <h2>We turn a discretized physics problem into an optimization problem a spin system can represent.</h2>
        <div class="math-block compact">{@html renderBlockMath(heroEquation)}</div>
      </div>

      <aside class="signal-panel">
        <span class="panel-label">Pipeline</span>
        <div class="signal-flow">
          <span>PDE</span>
          <span>→</span>
          <span>A x = b</span>
          <span>→</span>
          <span>QUBO</span>
          <span>→</span>
          <span>ISING</span>
        </div>
        <p>
          Continuous field → sparse algebra → binary optimization → spin Hamiltonian.
        </p>
      </aside>
    </div>
  </article>

  <article class="retro-window">
    <div class="window-bar violet">
      <span>Theory Stack</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body stacked">
      <div class="section-head">
        <p class="section-kicker">Full theory</p>
        <h3>The four translation layers</h3>
      </div>

      <div class="pipeline-grid">
        {#each pipeline as stage}
          <section class="theory-card">
            <div class="theory-topline">
              <span class="step-chip">{stage.step}</span>
              <span class="step-label">{stage.label}</span>
            </div>
            <h4>{stage.title}</h4>
            <div class="math-block">{@html renderBlockMath(stage.math)}</div>
            <div class="math-block emphasis">{@html renderBlockMath(stage.objective)}</div>
          </section>
        {/each}
      </div>
    </div>
  </article>

  <article class="retro-window">
    <div class="window-bar cyan">
      <span>How The Solver Works</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body solver-layout">
      <div class="solver-copy">
        <p class="section-kicker">Optimization view</p>
        <h3>Solver board</h3>
      </div>

      <div class="solver-rail">
        {#each solverStages as stage}
          <div class="solver-step math-block">{@html renderBlockMath(stage)}</div>
        {/each}
      </div>
    </div>
  </article>

  <div class="lower-grid">
    <article class="retro-window">
      <div class="window-bar orange">
        <span>What Worked</span>
        <span class="window-controls">_ □ ×</span>
      </div>
      <div class="window-body stacked">
        <div class="section-head compact">
          <p class="section-kicker">Proof of concept</p>
          <h3>Observed successes</h3>
        </div>

        <div class="results-list short">
          {#each successPanels as panel}
            <section class="result-card">
              <h4>{panel.title}</h4>
              <div class="math-block emphasis">{@html renderBlockMath(panel.math)}</div>
            </section>
          {/each}
        </div>

        <section class="matrix-board">
          <span class="result-tag">5x5 system solved</span>
          <div class="math-block emphasis">{@html renderBlockMath(systemEquation)}</div>
          <div class="matrix-grid">
            <div>
              <span class="eq-label">A</span>
              <div class="math-block matrix-math">{@html renderBlockMath(String.raw`A = ${solvedMatrixA}`)}</div>
            </div>
            <div>
              <span class="eq-label">b</span>
              <div class="math-block matrix-math">{@html renderBlockMath(String.raw`b = ${solvedVectorB}`)}</div>
            </div>
          </div>
          <div class="math-block">{@html renderBlockMath(solvedSystem.encoding)}</div>
          <div class="matrix-grid">
            <div>
              <span class="eq-label">Recovered x</span>
              <div class="math-block matrix-math">{@html renderBlockMath(String.raw`x_{\mathrm{QA}} = ${solvedRecoveredX}`)}</div>
            </div>
            <div>
              <span class="eq-label">True x</span>
              <div class="math-block matrix-math">{@html renderBlockMath(String.raw`x_{\mathrm{true}} = ${solvedTrueX}`)}</div>
            </div>
          </div>
        </section>
      </div>
    </article>

    <article class="retro-window">
      <div class="window-bar red">
        <span>Limitations</span>
        <span class="window-controls">_ □ ×</span>
      </div>
      <div class="window-body stacked">
        <div class="section-head compact">
          <p class="section-kicker">Reality check</p>
          <h3>Why this does not scale yet</h3>
        </div>

        <div class="limit-panel">
          {#each limitations as limitation}
            <div class="limit-item">
              <span class="limit-bullet">!</span>
              <div class="math-block compact">{@html renderBlockMath(limitation)}</div>
            </div>
          {/each}
        </div>
      </div>
    </article>
  </div>
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
    text-transform: uppercase;
    font-size: 0.8rem;
    letter-spacing: 0.08em;
    color: #120a20;
  }

  .window-bar.gold {
    background: linear-gradient(90deg, #ffe972, #ff8a4d);
  }

  .window-bar.violet {
    background: linear-gradient(90deg, #d191ff, #6d5cff);
    color: #fef6ff;
  }

  .window-bar.cyan {
    background: linear-gradient(90deg, #79e7ff, #2e9fff);
  }

  .window-bar.orange {
    background: linear-gradient(90deg, #ffbc68, #ff7c44);
  }

  .window-bar.red {
    background: linear-gradient(90deg, #ff8f9e, #ff4d6d);
  }

  .window-controls {
    letter-spacing: 0.2em;
  }

  .window-body {
    padding: 1rem 1.1rem 1.15rem;
  }

  .hero-layout,
  .solver-layout,
  .stacked {
    display: grid;
    gap: 1rem;
  }

  .hero-layout {
    grid-template-columns: minmax(0, 1.45fr) minmax(16rem, 0.8fr);
    align-items: start;
  }

  .solver-layout {
    grid-template-columns: minmax(0, 0.9fr) minmax(0, 1.1fr);
  }

  .lower-grid {
    display: grid;
    gap: 1.15rem;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  }

  .section-kicker {
    margin: 0 0 0.6rem;
    color: #ff8fcb;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 1.5rem;
  }

  .hero-copy h2,
  .section-head h3,
  .solver-copy h3 {
    margin: 0.2rem 0 0.8rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.02;
  }

  .hero-copy p,
  .solver-copy p,
  .theory-card p,
  .solver-step p,
  .result-card p,
  .limit-item p,
  .equation-strip p,
  .signal-panel p {
    margin: 0;
    color: #efdaf9;
    line-height: 1.65;
  }

  .math-block {
    margin: 0;
    padding: 0.75rem 0.85rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(9, 12, 22, 0.56);
    color: #fff6c7;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 0.95rem;
    line-height: 1.5;
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .math-block :global(.katex-display) {
    margin: 0;
  }

  .math-block :global(.katex) {
    color: inherit;
    font-size: 1em;
  }

  .math-block.compact {
    padding: 0.55rem 0.7rem;
  }

  .math-block.emphasis {
    background: rgba(255, 233, 114, 0.08);
  }

  .matrix-math {
    overflow-x: auto;
  }

  .signal-panel,
  .theory-card,
  .solver-step,
  .result-card,
  .matrix-board,
  .limit-panel,
  .equation-strip > div {
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.035);
  }

  .signal-panel {
    padding: 0.95rem;
    background:
      linear-gradient(180deg, rgba(255, 233, 114, 0.12), rgba(122, 41, 255, 0.08)),
      rgba(255, 255, 255, 0.035);
  }

  .panel-label,
  .result-tag,
  .eq-label,
  .step-label {
    display: inline-block;
    margin-bottom: 0.65rem;
    color: #84c3ff;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    font-size: 0.72rem;
  }

  .signal-flow {
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem;
    align-items: center;
    margin-bottom: 0.9rem;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: clamp(1.2rem, 2vw, 1.85rem);
    color: #ffe972;
    line-height: 1.1;
  }

  .pipeline-grid {
    display: grid;
    gap: 0.9rem;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .theory-card,
  .solver-step,
  .result-card {
    padding: 0.95rem;
  }

  .theory-topline {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    margin-bottom: 0.65rem;
  }

  .step-chip {
    min-width: 2.2rem;
    padding: 0.25rem 0.45rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.05);
    color: #fff6a8;
    text-align: center;
  }

  .theory-card h4,
  .solver-step h4,
  .result-card h4 {
    margin: 0 0 0.45rem;
    color: #fff3ff;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 1.1rem;
  }

  .theory-summary {
    margin-bottom: 0.55rem;
    color: #fff5c2;
  }

  .solver-rail,
  .results-list {
    display: grid;
    gap: 0.8rem;
  }

  .results-list.short {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .matrix-board {
    display: grid;
    gap: 0.85rem;
    padding: 0.95rem;
  }

  .matrix-grid {
    display: grid;
    gap: 0.8rem;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .limit-panel {
    display: grid;
    gap: 0.7rem;
    padding: 0.9rem;
  }

  .limit-item {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.75rem;
    align-items: start;
  }

  .limit-bullet {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.6rem;
    height: 1.6rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 79, 118, 0.14);
    color: #ffd8df;
    font-family: "Glastone", "Dotemp", monospace;
  }

  .section-head.compact h3,
  .section-head.compact .section-kicker {
    margin-bottom: 0.45rem;
  }

  @media (max-width: 1080px) {
    .hero-layout,
    .solver-layout,
    .lower-grid,
    .pipeline-grid,
    .results-list.short,
    .matrix-grid {
      grid-template-columns: minmax(0, 1fr);
    }
  }

  @media (max-width: 760px) {
    .page-shell {
      padding: 0.8rem;
    }

    .window-body {
      padding: 0.9rem;
    }

    .signal-flow {
      font-size: 1rem;
    }
  }
</style>

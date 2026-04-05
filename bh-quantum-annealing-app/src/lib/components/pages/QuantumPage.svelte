<script lang="ts">
  import Katex from "svelte-katex";

  const poissonEq = String.raw`-\Delta u(x,y)=f(x,y),\qquad u=g\text{ on }\partial\Omega`;
  const finiteDifferenceEq = String.raw`\frac{u_{i-1,j}-2u_{i,j}+u_{i+1,j}}{\Delta x^2}+\frac{u_{i,j-1}-2u_{i,j}+u_{i,j+1}}{\Delta y^2}=f_{i,j}`;
  const linearEq = String.raw`Au=b`;
  const encodingEq = String.raw`u_i \approx u_{\min}+\sum_{k=0}^{K-1} w_k z_{ik},\qquad z_{ik}\in\{0,1\}`;
  const quboEq = String.raw`\min_{z\in\{0,1\}^{nK}} z^\top Qz`;
  const normalEq = String.raw`M=A^\top A,\qquad C=A^\top b`;
  const isingEq = String.raw`E(s)=\sum_i h_i s_i+\sum_{i<j}J_{ij}s_is_j+c,\qquad s_i\in\{-1,1\}`;
  const scaleEq = String.raw`q=nK,\qquad Q\in\mathbb{R}^{q\times q}`;
  const gridScaleEq = String.raw`n=m^2\Rightarrow q=m^2K,\qquad \text{couplings scale like }m^4K^2`;
  const exampleA = String.raw`A=\begin{bmatrix}1&3&0&2&0\\0&0&4&6&5\\3&1&3&0&2\\1&0&0&0&3\\2&0&5&1&0\end{bmatrix}`;
  const exampleB = String.raw`b=\begin{bmatrix}15\\61\\24\\16\\21\end{bmatrix}`;
  const exampleEncoding = String.raw`u_{\min}=0,\quad u_{\max}=10,\quad K=12\Rightarrow q=5\cdot12=60`;
</script>

<section class="page-shell">
  <article class="retro-window hero">
    <div class="window-bar gold">
      <span>Quantum Pipeline / Theory</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body split hero-split">
      <div class="window-copy">
        <p class="section-kicker">Theory track</p>
        <h3>From Poisson to annealing</h3>
        <p>
          The project starts with a continuous PDE, discretizes it into a linear system, encodes
          the unknowns into binary variables, and then solves the resulting optimization problem by
          converting it into an Ising model for annealing.
        </p>
        <p>
          In the current code, the classical side builds a finite-difference Poisson system, while
          the quantum side takes a sparse matrix <em>A</em> and right-hand side <em>b</em>,
          constructs a QUBO, converts that QUBO to Ising form, and sends it to the
          <code>quantrs2_anneal</code> simulator.
        </p>
      </div>
      <div class="pixel-panel equation-panel">
        <span class="pixel-label">Pipeline</span>
        <div class="equation-stack">
          <div class="equation-line"><Katex>{poissonEq}</Katex></div>
          <div class="equation-line"><Katex>{finiteDifferenceEq}</Katex></div>
          <div class="equation-line"><Katex>{linearEq}</Katex></div>
          <div class="equation-line"><Katex>{encodingEq}</Katex></div>
          <div class="equation-line"><Katex>{quboEq}</Katex></div>
          <div class="equation-line"><Katex>{isingEq}</Katex></div>
        </div>
      </div>
    </div>
  </article>

  <article class="retro-window wide">
    <div class="window-bar">
      <span>Worked Example</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body split">
      <div>
        <p class="section-kicker">5×5 system from the Rust crate</p>
        <p>
          The current `quantum-solver` crate includes a concrete 5×5 linear system. It uses a
          dense example matrix, converts it to sparse form, chooses a real-valued encoding range,
          and builds the QUBO from there.
        </p>
        <p>
          That gives a small enough system to explain the mechanics clearly while still exercising
          the full pipeline implemented in the solver code.
        </p>
        <ul>
          <li>`dense_to_csmat` and `dense_to_csvec` convert the example into sparse storage.</li>
          <li>`compute_qubo` forms the binary objective from `A`, `b`, and the encoding.</li>
          <li>`qubo_to_ising` converts the QUBO coefficients into Ising fields and couplings.</li>
          <li>`solve_ising_model` runs the annealing simulator and decodes the bits back to reals.</li>
        </ul>
      </div>
      <div class="pixel-card math-card">
        <div class="pixel-badge">Current Example</div>
        <div class="equation-stack">
          <div class="equation-line"><Katex>{exampleA}</Katex></div>
          <div class="equation-line"><Katex>{exampleB}</Katex></div>
          <div class="equation-line"><Katex>{normalEq}</Katex></div>
          <div class="equation-line"><Katex>{exampleEncoding}</Katex></div>
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

  <article class="retro-window duo">
    <div class="window-bar">
      <span>Why It Gets Hard</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body split">
      <div>
        <p class="section-kicker">Scaling wall</p>
        <p>
          The theory is clean, but the variable count grows quickly. If a 2D grid has
          <em>m</em>×<em>m</em> unknowns, then the linear system size is <em>n = m²</em>. Encoding
          each unknown with <em>K</em> bits creates a QUBO with <em>q = nK</em> binary variables.
        </p>
        <p>
          That means the QUBO matrix itself is <em>q × q</em>, so the interaction structure grows
          like <em>m⁴K²</em>. This is exactly why the project works as a proof of concept on small
          systems but becomes computationally painful when we try to scale realistic PDE grids on
          classical hardware.
        </p>
        <p>
          In short: the mapping from PDE solve to annealing objective works, but simulating the
          quantum process classically is the bottleneck.
        </p>
      </div>
      <div class="pixel-card">
        <div class="pixel-badge">Takeaway</div>
        <div class="equation-stack">
          <div class="equation-line"><Katex>{scaleEq}</Katex></div>
          <div class="equation-line"><Katex>{gridScaleEq}</Katex></div>
        </div>
        <p class="takeaway-copy">
          TL;DR: the workflow is mathematically valid and small examples are informative, but the
          state size and coupling count explode before the approach becomes a practical classical
          replacement for standard PDE solvers.
        </p>
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

  .hero-split {
    align-items: start;
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

  .pixel-label,
  .pixel-badge {
    display: inline-block;
    margin-bottom: 0.8rem;
    color: #84c3ff;
    font-size: 0.72rem;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }

  .equation-panel,
  .math-card {
    align-self: start;
  }

  .equation-stack {
    display: grid;
    gap: 0.75rem;
  }

  .equation-line {
    overflow-x: auto;
    padding-bottom: 0.1rem;
    color: #fff4c7;
  }

  .takeaway-copy {
    margin-top: 1rem;
  }

  :global(.equation-line .katex) {
    font-size: 1.02rem;
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

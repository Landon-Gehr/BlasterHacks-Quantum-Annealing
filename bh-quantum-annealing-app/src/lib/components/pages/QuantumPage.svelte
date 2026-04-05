<script lang="ts">
  import katex from "katex";

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

  const pipelineStages = [
    {
      step: "01",
      label: "PDE",
      title: "Start with the continuous problem",
      body:
        "The classical model is a Poisson problem on a rectangular domain. Boundary data and forcing define the field we want to recover.",
      math: poissonEq
    },
    {
      step: "02",
      label: "Discretize",
      title: "Turn the PDE into algebra",
      body:
        "Finite differences replace derivatives with local stencil updates, producing a sparse linear system whose unknowns are the grid values.",
      math: finiteDifferenceEq
    },
    {
      step: "03",
      label: "Encode",
      title: "Represent real variables with bits",
      body:
        "Each continuous unknown is approximated with a fixed binary expansion over a chosen interval. This is the bridge from numerics to combinatorics.",
      math: encodingEq
    },
    {
      step: "04",
      label: "Optimize",
      title: "Solve QUBO, then map to Ising",
      body:
        "The encoded objective becomes a binary quadratic model. That QUBO is then translated into an Ising Hamiltonian for annealing.",
      math: `${quboEq}\\\\${isingEq}`
    }
  ];

  const successPanels = [
    {
      title: "The mapping exists end to end",
      text:
        "The codebase already includes the full symbolic pipeline from sparse linear system to QUBO to Ising model to annealing simulator."
    },
    {
      title: "Small systems are explainable",
      text:
        "A 5×5 example is small enough to inspect directly, which makes it useful for understanding how the encoding behaves."
    },
    {
      title: "Classical Poisson solves remain the baseline",
      text:
        "The finite-difference side still gives the cleanest path to reliable PDE solutions, which is why it acts as the comparison point."
    }
  ];

  const limitations = [
    "Each real unknown becomes K binary variables.",
    "A 2D grid with m\\times m unknowns gives n=m^2 real variables.",
    "So the QUBO dimension becomes q=nK=m^2K.",
    "The interaction structure scales like m^4K^2, which becomes expensive very quickly."
  ];
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
          The current <code>quantum-solver</code> crate includes a concrete 5×5 linear system. It
          uses a dense example matrix, converts it to sparse form, chooses a real-valued encoding
          range, and builds the QUBO from there.
        </p>
        <p>
          That gives a small enough system to explain the mechanics clearly while still exercising
          the full pipeline implemented in the solver code.
        </p>
        <ul>
          <li><code>dense_to_csmat</code> and <code>dense_to_csvec</code> prepare sparse inputs.</li>
          <li><code>compute_qubo</code> forms the binary objective from <code>A</code>, <code>b</code>, and the encoding.</li>
          <li><code>qubo_to_ising</code> converts QUBO coefficients into Ising fields and couplings.</li>
          <li><code>solve_ising_model</code> runs the annealing simulator and decodes the bits back to reals.</li>
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
        <p class="math-caption">
          Continuous field → sparse algebra → binary optimization → spin Hamiltonian.
        </p>
      </div>
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
        {#each pipelineStages as stage}
          <section class="theory-card">
            <div class="theory-topline">
              <span class="step-chip">{stage.step}</span>
              <span class="step-label">{stage.label}</span>
            </div>
            <h4>{stage.title}</h4>
            <p>{stage.body}</p>
            <div class="equation-line compact"><Katex>{stage.math}</Katex></div>
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
        <div class="limitations">
          {#each limitations as limitation}
            <div class="limit-item">
              <span class="limit-bullet">!</span>
              <p>{limitation}</p>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </article>

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

      <div class="results-list">
        {#each successPanels as panel}
          <section class="result-card">
            <h4>{panel.title}</h4>
            <p>{panel.text}</p>
          </section>
        {/each}
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
    background: linear-gradient(90deg, #2f2750, #19162d);
    color: #fff3ff;
    text-transform: uppercase;
    font-size: 0.8rem;
    letter-spacing: 0.08em;
  }

  .window-bar.gold {
    background: linear-gradient(90deg, #ffca54, #ff8f49);
    color: #2a1300;
  }

  .window-bar.violet {
    background: linear-gradient(90deg, #7f63ff, #d749c5);
    color: #140822;
  }

  .window-bar.orange {
    background: linear-gradient(90deg, #ffb347, #ff6e42);
    color: #241105;
  }

  .window-controls {
    letter-spacing: 0.2em;
  }
  .window-controls {
    letter-spacing: 0.2em;
  }

  .window-body {
    padding: 1rem 1.1rem 1.15rem;
  }
  .window-body {
    padding: 1rem 1.1rem 1.15rem;
  }

  .window-body.split {
    display: grid;
    gap: 1rem;
    grid-template-columns: minmax(0, 1.2fr) minmax(14rem, 0.8fr);
  }

  .window-body.stacked {
    display: grid;
    gap: 1rem;
  }

  .hero-split {
    align-items: start;
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

  .window-copy h3,
  .section-head h3 {
    margin: 0.25rem 0 0.75rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.05;
  }

  .window-copy p,
  .retro-window p,
  .retro-window li {
    color: #efdaf9;
    line-height: 1.65;
  }

  .pixel-panel,
  .pixel-card,
  .theory-card,
  .result-card {
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.035);
    padding: 0.9rem;
  }

  .pixel-panel {
    display: grid;
    align-content: start;
    min-height: 12rem;
    background:
      linear-gradient(180deg, rgba(255, 211, 97, 0.14), rgba(255, 72, 130, 0.08)),
      rgba(255, 255, 255, 0.03);
  }

  .cpoint-mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.6rem;
    height: 1.6rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(95, 255, 180, 0.12);
    color: #5fffb4;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 0.9rem;
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

  .cpoint p { margin: 0; color: #efdaf9; font-size: 0.86rem; line-height: 1.55; }

  .equation-line.compact {
    margin-top: 0.6rem;
  }

  .math-caption {
    margin-top: 1rem;
  }

  .pipeline-grid,
  .results-list {
    display: grid;
    gap: 1rem;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .theory-topline {
    display: flex;
    align-items: center;
    gap: 0.65rem;
    margin-bottom: 0.65rem;
  }

  .step-chip {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 2.2rem;
    padding: 0.18rem 0.45rem;
    background: rgba(255, 246, 168, 0.12);
    color: #fff6a8;
    border: 1px solid rgba(255, 246, 168, 0.28);
  }

  .step-label {
    color: #84c3ff;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    font-size: 0.78rem;
  }

  .theory-card h4,
  .result-card h4 {
    margin: 0 0 0.45rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.08;
  }

  .limitations {
    display: grid;
    gap: 0.7rem;
    margin-top: 1rem;
  }

  .limit-item {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: 0.65rem;
    align-items: start;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.025);
    padding: 0.7rem 0.75rem;
  }

  .limit-bullet {
    color: #ff8fcb;
    font-family: "Glastone", "Dotemp", monospace;
  }

  .limit-item p {
    margin: 0;
  }

  :global(.equation-line .katex) {
    font-size: 1.25rem;
  }

  /* ── Responsive ── */
  @media (max-width: 1080px) {
    .window-body.split,
    .pipeline-grid,
    .results-list {
      grid-template-columns: minmax(0, 1fr);
    }
  }

  @media (max-width: 980px) {
    .page-shell {
      padding: 0.8rem;
    }
  }
</style>

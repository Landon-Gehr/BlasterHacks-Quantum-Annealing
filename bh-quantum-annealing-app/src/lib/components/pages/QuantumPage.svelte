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
  <article class="retro-window wide">
    <div class="window-bar">
      <span>Quantum / Background</span>
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

  .window-controls {
    letter-spacing: 0.2em;
  }

  .window-body {
    padding: 1rem 1.1rem 1.15rem;
  }

  .window-body.split {
    display: grid;
    gap: 1rem;
    grid-template-columns: minmax(0, 1.2fr) minmax(14rem, 0.8fr);
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

  .window-copy h3 {
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
  .pixel-card {
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

  @media (max-width: 980px) {
    .page-shell {
      padding: 0.8rem;
    }

    .window-body.split {
      grid-template-columns: minmax(0, 1fr);
    }
  }
</style>

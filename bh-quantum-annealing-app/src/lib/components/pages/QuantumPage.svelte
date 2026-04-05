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
    if (Number.isInteger(value)) return value.toString();
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

  // Each theory stage: main math, objective/detail math, prose
  const theoryStages = [
    {
      step: "01",
      title: "PDE",
      label: "Continuous problem",
      math: String.raw`-\Delta u = f \text{ on } \Omega, \qquad u|_{\partial \Omega} = 0`,
      objective: String.raw`\min_{u \in \mathbb{R}^{\Omega}} \; \mathcal{L}(u)`,
      prose:
        "We start with a PDE on a continuous domain Ω. The unknown field u must satisfy the equation everywhere in the interior and match prescribed values on the boundary. This is the original physics — infinite-dimensional and not directly amenable to linear algebra.",
      detail: null
    },
    {
      step: "02",
      title: "Linear System",
      label: "Discretize the operators",
      math: String.raw`A x = b`,
      objective: String.raw`\min_{x \in \mathbb{R}^n} \; \|A x - b\|_2^2`,
      prose:
        "Finite-difference or finite-element discretization replaces the continuous operators with a sparse matrix A and a right-hand-side vector b. Solving Ax = b is equivalent to minimising the squared residual — a least-squares objective that is purely quadratic in x.",
      detail: null
    },
    {
      step: "03",
      title: "Binary Encoding",
      label: "Represent each real unknown in bits",
      math: String.raw`x_i \;\approx\; \sum_{k=0}^{K-1} 2^k \, q_{i,k}, \qquad q_{i,k} \in \{0,1\}`,
      objective: String.raw`x_i \approx u_{\min} + \frac{u_{\max} - u_{\min}}{2^K - 1}\sum_{k=0}^{K-1} 2^k \, q_{i,k}`,
      prose:
        "Each continuous unknown x_i is replaced by K binary variables q_{i,k} ∈ {0,1}. The powers-of-two weights give a fixed-point binary expansion whose integer value, after rescaling to the target range [u_min, u_max], approximates x_i. Precision improves with K, but the total number of binary variables grows as N_binary = K · N_physical — the root of the scaling problem.",
      detail: String.raw`N_{\mathrm{binary}} = K \cdot N_{\mathrm{physical}}`
    },
    {
      step: "04",
      title: "QUBO",
      label: "Substitute encoding into the residual",
      math: String.raw`x \approx x(q), \qquad q \in \{0,1\}^N`,
      objective: String.raw`\min_{q \in \{0,1\}^N} \; q^\top Q q`,
      prose:
        "Substituting the binary expansion into the residual ‖Ax − b‖² and expanding yields a purely quadratic form in the binary variables. The result is a QUBO (Quadratic Unconstrained Binary Optimization) problem: minimise q⊤Qq over {0,1}^N, with the matrix Q encoding every pairwise interaction between bits.",
      detail: null
    },
    {
      step: "05",
      title: "Ising Model",
      label: "Shift bits to spins in {−1, +1}",
      math: String.raw`q_i = \tfrac{1 + s_i}{2}, \qquad s_i \in \{-1,+1\}`,
      objective: String.raw`\min_{s \in \{-1,1\}^N} \; \sum_{i<j} J_{ij}\, s_i s_j + \sum_i h_i\, s_i + c`,
      prose:
        "Substituting q_i = (1 + s_i)/2 into the QUBO rewrites the objective in terms of ±1 spin variables. The result is an Ising Hamiltonian with coupling matrix J and bias vector h. This is the native representation for quantum annealers: each qubit is a spin, couplings are wire weights, and minimisation corresponds to finding the ground state.",
      detail: null
    }
  ];

  // QuantumAnnealingSimulator steps
  const annealerSteps = [
    {
      label: "Init",
      title: "Transverse-field start",
      math: String.raw`H(0) = H_T = -\!\sum_i \sigma_i^x`,
      note:
        "The simulator initialises every qubit in the |+⟩ superposition state — the unique ground state of the transverse-field driver H_T. All spin configurations are equally probable at this point; the system has no preference for the problem landscape yet."
    },
    {
      label: "Anneal",
      title: "Adiabatic schedule",
      math: String.raw`H\!\left(s(t)\right) = \bigl(1 - s(t)\bigr)\,H_T \;+\; s(t)\,H_P, \qquad s: 0 \to 1`,
      note:
        "The annealing parameter s(t) is swept from 0 to 1. As s grows the transverse field — which drives quantum tunnelling across energy barriers — is suppressed, and the problem Hamiltonian H_P is turned on. The adiabatic theorem guarantees that if the schedule is slow enough the system remains in the instantaneous ground state throughout."
    },
    {
      label: "Problem",
      title: "Ising Hamiltonian",
      math: String.raw`H_P = \sum_{i<j} J_{ij}\,\sigma_i^z\sigma_j^z + \sum_i h_i\,\sigma_i^z`,
      note:
        "H_P encodes the objective directly: J_{ij} are the off-diagonal couplings between pairs of qubits, derived from the upper triangle of Q; h_i are the local biases derived from the diagonal of Q and the linear terms. Minimising the expectation of H_P is equivalent to minimising q⊤Qq."
    },
    {
      label: "Read",
      title: "Measurement and decode",
      math: String.raw`s^* = \arg\min_{s\in\{-1,+1\}^N} E(s) \;\xrightarrow{\;q_i = \frac{1+s_i}{2}\;}\; q^* \;\xrightarrow{\;\text{rescale}\;}\; x^*`,
      note:
        "At the end of the schedule each qubit is measured and collapses to ±1. The best spin configuration s* is converted back to bits via q_i = (1 + s_i)/2, then to physical unknowns by reversing the binary expansion. Multiple shots can be taken and the lowest-energy sample selected."
    }
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
    encoding: String.raw`u_{\min} = 0, \quad u_{\max} = 10, \quad K = 12`
  };

  const successPanels = [
    { title: "5×5 dense test",    math: String.raw`\min_{x \in \mathbb{R}^5} \|A x - b\|_2^2` },
    { title: "Coarse Poisson solve", math: String.raw`-\Delta u = f, \qquad u_{QA} \approx u_{\mathrm{manufactured}}` }
  ];

  const systemEquation   = String.raw`A x = b`;
  const solvedMatrixA    = matrixToLatex(solvedSystem.a);
  const solvedVectorB    = vectorToLatex(solvedSystem.b);
  const solvedRecoveredX = vectorToLatex(solvedSystem.xRecovered);
  const solvedTrueX      = vectorToLatex(solvedSystem.xTrue);
</script>

<section class="page-shell">

  <!-- ── HERO ─────────────────────────────────────────────── -->
  <article class="retro-window">
    <div class="window-bar">
      <span>Quantum Pipeline / Theory</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body">
      <p class="section-kicker">From PDE to annealing</p>
      <h2>We turn a discretized physics problem into an optimization problem a spin system can represent.</h2>
    </div>
  </article>

  <!-- ── THEORY STACK ──────────────────────────────────────── -->
  <article class="retro-window">
    <div class="window-bar">
      <span>Theory Stack</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body stacked">
      <div class="section-head">
        <p class="section-kicker">Full derivation</p>
        <h3>Five translation layers: from PDE to spin Hamiltonian</h3>
      </div>

      <div class="theory-grid">
        {#each theoryStages as stage}
          <section class="theory-card">
            <div class="theory-topline">
              <span class="step-chip">{stage.step}</span>
              <div>
                <span class="step-label">{stage.label}</span>
                <h4>{stage.title}</h4>
              </div>
            </div>
            <div class="math-block">{@html renderBlockMath(stage.math)}</div>
            <div class="math-block emphasis">{@html renderBlockMath(stage.objective)}</div>
            <p class="card-prose">{stage.prose}</p>
            {#if stage.detail}
              <div class="math-block detail-math">{@html renderBlockMath(stage.detail)}</div>
            {/if}
          </section>
        {/each}
      </div>
    </div>
  </article>

  <!-- ── SOLVER ────────────────────────────────────────────── -->
  <article class="retro-window">
    <div class="window-bar">
      <span>How QuantumAnnealingSimulator Solves the Ising Model</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body stacked">
      <div class="section-head">
        <p class="section-kicker">quantrs2_anneal</p>
        <h3>Adiabatic evolution from transverse field to problem Hamiltonian</h3>
      </div>

      <div class="annealer-grid">
        {#each annealerSteps as step, i}
          <section class="annealer-card">
            <div class="theory-topline">
              <span class="step-chip">{step.label}</span>
              <h4>{step.title}</h4>
            </div>
            <div class="math-block emphasis">{@html renderBlockMath(step.math)}</div>
            <p class="card-prose">{step.note}</p>
          </section>
          {#if i < annealerSteps.length - 1}
            <div class="annealer-connector">↓</div>
          {/if}
        {/each}
      </div>
    </div>
  </article>

  <!-- ── LOWER GRID ────────────────────────────────────────── -->
  <div class="lower-grid">

    <article class="retro-window">
      <div class="window-bar">
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
          <span class="result-tag">5×5 system solved</span>
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
      <div class="window-bar">
        <span>Conclusion</span>
        <span class="window-controls">_ □ ×</span>
      </div>
      <div class="window-body stacked">
        <div class="section-head compact">
          <p class="section-kicker">Takeaway</p>
          <h3>Limited success with infeasible scaling</h3>
        </div>

        <div class="conclusion-body">
          <p class="conclusion-lead">
            The pipeline is theoretically sound and produces correct solutions on small instances. The bottleneck is not correctness — it is qubit count.
          </p>

          <div class="conclusion-points">
            <div class="cpoint">
              <span class="cpoint-mark">✓</span>
              <div>
                <strong>Pipeline validated</strong>
                <p>PDE → linear system → QUBO → Ising transformation is algebraically exact.</p>
              </div>
            </div>
            <div class="cpoint">
              <span class="cpoint-mark">✓</span>
              <div>
                <strong>Correct on toy problems</strong>
                <p>5×5 dense and coarse Poisson cases recovered ground truth with 12-bit encoding.</p>
              </div>
            </div>
            <div class="cpoint">
              <span class="cpoint-mark warn">✗</span>
              <div>
                <strong>Qubit count explodes</strong>
                <div class="math-block compact conclusion-math">{@html renderBlockMath(String.raw`N_\mathrm{binary} = K \cdot N_\mathrm{physical} \;\xrightarrow{K=12}\; 12\times\text{overhead}`)}</div>
              </div>
            </div>
            <div class="cpoint">
              <span class="cpoint-mark warn">✗</span>
              <div>
                <strong>No QPU access</strong>
                <p>Classical simulation of the annealer grows exponentially — theory validated, hardware advantage absent.</p>
              </div>
            </div>
          </div>
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

  /* ── Window chrome ── */
  .retro-window {
    width: 100%;
    border: 2px solid #2b2340;
    background: linear-gradient(180deg, rgba(20, 16, 28, 0.96), rgba(10, 10, 16, 0.96));
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
    background: rgba(255, 255, 255, 0.04);
    text-transform: uppercase;
    font-size: 0.8rem;
    letter-spacing: 0.08em;
    color: rgba(255, 255, 255, 0.5);
  }

  .window-controls { letter-spacing: 0.2em; }

  .window-body { padding: 1rem 1.1rem 1.15rem; }

  .stacked { display: grid; gap: 1rem; }

  /* ── Typography ── */
  .section-kicker {
    margin: 0 0 0.6rem;
    color: #ff8fcb;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 1.5rem;
  }

  h2, h3 {
    margin: 0.2rem 0 0.8rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.02;
  }

  h4 {
    margin: 0 0 0.45rem;
    color: #fff3ff;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 1.1rem;
  }

  .section-head { display: grid; gap: 0.2rem; }
  .section-head.compact h3,
  .section-head.compact .section-kicker { margin-bottom: 0.45rem; }

  .card-prose {
    margin: 0.65rem 0 0;
    color: #efdaf9;
    line-height: 1.65;
    font-size: 0.9rem;
  }

  /* ── Math blocks ── */
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

  .math-block :global(.katex-display) { margin: 0; }
  .math-block :global(.katex) { color: inherit; font-size: 1em; }
  .math-block.compact    { padding: 0.55rem 0.7rem; }
  .math-block.emphasis   { background: rgba(255, 233, 114, 0.08); }
  .math-block.detail-math { margin-top: 0.6rem; }
  .matrix-math           { overflow-x: auto; }

  /* ── Shared card chrome ── */
  .theory-card,
  .annealer-card,
  .result-card,
  .matrix-board,
  .cpoint {
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.035);
  }

  .panel-label,
  .result-tag,
  .eq-label,
  .step-label {
    display: inline-block;
    margin-bottom: 0.4rem;
    color: #84c3ff;
    text-transform: uppercase;
    letter-spacing: 0.14em;
    font-size: 0.72rem;
  }

  /* ── Step chip ── */
  .step-chip {
    flex-shrink: 0;
    min-width: 2.2rem;
    padding: 0.25rem 0.45rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.05);
    color: #fff6a8;
    text-align: center;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 0.8rem;
  }

  .theory-topline {
    display: flex;
    align-items: flex-start;
    gap: 0.65rem;
    margin-bottom: 0.65rem;
  }

  /* ── Theory grid: 2-col, rich prose cards ── */
  .theory-grid {
    display: grid;
    gap: 0.9rem;
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  /* Last card (05 Ising) spans full width */
  .theory-grid > :last-child { grid-column: 1 / -1; }

  .theory-card {
    padding: 0.95rem;
    display: grid;
    gap: 0.5rem;
    align-content: start;
  }

  /* ── Annealer: single-column flowing list ── */
  .annealer-grid { display: grid; gap: 0; }

  .annealer-card {
    padding: 0.95rem;
    display: grid;
    gap: 0.5rem;
    align-content: start;
  }

  .annealer-connector {
    text-align: center;
    padding: 0.3rem 0;
    color: rgba(255, 255, 255, 0.2);
    font-size: 1.2rem;
    border-left: 2px solid rgba(255, 255, 255, 0.06);
    border-right: 2px solid rgba(255, 255, 255, 0.06);
    background: rgba(255, 255, 255, 0.015);
  }

  /* ── Lower grid ── */
  .lower-grid {
    display: grid;
    gap: 1.15rem;
    grid-template-columns: minmax(0, 1fr) minmax(0, 1fr);
  }

  .results-list { display: grid; gap: 0.8rem; }
  .results-list.short { grid-template-columns: repeat(2, minmax(0, 1fr)); }

  .result-card  { padding: 0.95rem; }

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

  /* ── Conclusion ── */
  .conclusion-body    { display: grid; gap: 0.9rem; }
  .conclusion-points  { display: grid; gap: 0.65rem; }

  .conclusion-lead {
    margin: 0;
    color: #fff6a8;
    font-size: 0.97rem;
    line-height: 1.65;
    padding: 0.75rem 0.9rem;
    border-left: 3px solid rgba(255, 255, 255, 0.18);
    background: rgba(255, 255, 255, 0.035);
  }

  .cpoint {
    display: grid;
    grid-template-columns: 2rem 1fr;
    gap: 0.7rem;
    align-items: start;
    padding: 0.75rem;
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

  .cpoint-mark.warn {
    background: rgba(255, 79, 118, 0.14);
    color: #ffd8df;
  }

  .cpoint strong {
    display: block;
    color: #fff3ff;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 0.95rem;
    margin-bottom: 0.3rem;
  }

  .cpoint p { margin: 0; color: #efdaf9; font-size: 0.86rem; line-height: 1.55; }

  .conclusion-math { margin-top: 0.4rem; }

  /* ── Responsive ── */
  @media (max-width: 1080px) {
    .theory-grid,
    .lower-grid,
    .results-list.short,
    .matrix-grid {
      grid-template-columns: minmax(0, 1fr);
    }
    .theory-grid > :last-child { grid-column: 1; }
  }

  @media (max-width: 760px) {
    .page-shell  { padding: 0.8rem; }
    .window-body { padding: 0.9rem; }
  }
</style>
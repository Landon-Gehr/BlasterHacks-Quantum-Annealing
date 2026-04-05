<script lang="ts">
  const FIXED_SYSTEM_SIZE = 32;
  const FIXED_DOMAIN_DISPLAY = "4π";
  const FIXED_QUBO_BITS = 3;
  const MIN_COEFF = -10;
  const MAX_COEFF = 10;

  type TabId =
    | "overview"
    | "math-pipeline"
    | "results"
    | "ddpm"
    | "allen-cahn"
    | "poisson"
    | "conclusion";

  type Tab = {
    id: TabId;
    label: string;
    eyebrow: string;
  };

  const tabs: Tab[] = [
    { id: "overview", label: "Overview", eyebrow: "Introduction" },
    { id: "math-pipeline", label: "Math Pipeline", eyebrow: "Theory" },
    { id: "results", label: "Results", eyebrow: "What We Achieved" },
    { id: "ddpm", label: "DDPM", eyebrow: "ML Approach" },
    { id: "allen-cahn", label: "Allen-Cahn", eyebrow: "Motivation" },
    { id: "poisson", label: "Poisson Setup", eyebrow: "Interactive" },
    { id: "conclusion", label: "Conclusion", eyebrow: "Takeaway" }
  ];

  let activeTab = $state<TabId>("overview");

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
    <p class="eyebrow">BlasterHacks · Colorado School of Mines · April 2026</p>
    <h1>Solving PDEs with Quantum Annealing</h1>
    <p class="hero-copy">
      A full pipeline from partial differential equations to quantum annealing —
      converting PDE solving into a binary optimization problem solved by simulating quantum tunneling.
    </p>
  </section>

  <nav class="tab-bar" aria-label="Presentation sections">
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

  <!-- ── OVERVIEW ─────────────────────────────────────────────────────────── -->
  {#if activeTab === "overview"}
    <section class="page-grid">
      <article class="panel prose">
        <p class="section-label">Overview</p>
        <h2>What we built and why</h2>
        <p>
          Quantum annealers are purpose-built optimization machines. Rather than running
          general-purpose logic gates, they exploit quantum tunneling to find the low-energy
          configuration of an Ising spin system — which is exactly the structure you get when
          you encode a least-squares problem in binary.
        </p>
        <p>
          Our project turns a classical PDE solve into that optimization problem, runs it on a
          simulated quantum annealer, and measures how close we can get with limited hardware.
        </p>

        <div class="callout">
          <p class="callout-title">Core achievement</p>
          <p>
            We implemented the complete pipeline — finite-difference discretization, binary QUBO
            encoding, QUBO-to-Ising conversion, and simulated quantum annealing — and verified it
            on a 5×5 sparse linear system and a Poisson PDE on a discrete grid.
          </p>
        </div>

        <h3>Pipeline at a glance</h3>
        <div class="pipeline">
          <div class="pipeline-step">
            <span class="step-num">1</span>
            <strong>PDE</strong>
            <span class="step-sub">−Δu = f in Ω</span>
          </div>
          <span class="step-arrow">→</span>
          <div class="pipeline-step">
            <span class="step-num">2</span>
            <strong>Linear System</strong>
            <span class="step-sub">Au = b</span>
          </div>
          <span class="step-arrow">→</span>
          <div class="pipeline-step">
            <span class="step-num">3</span>
            <strong>QUBO</strong>
            <span class="step-sub">min xᵀQx</span>
          </div>
          <span class="step-arrow">→</span>
          <div class="pipeline-step">
            <span class="step-num">4</span>
            <strong>Ising Model</strong>
            <span class="step-sub">H(s) = Σ Jᵢⱼ sᵢsⱼ + Σ hᵢsᵢ</span>
          </div>
          <span class="step-arrow">→</span>
          <div class="pipeline-step">
            <span class="step-num">5</span>
            <strong>Anneal</strong>
            <span class="step-sub">quantum tunneling</span>
          </div>
        </div>
      </article>

      <aside class="panel">
        <p class="section-label">At a Glance</p>
        <h2>Key numbers</h2>

        <div class="stat-grid">
          <div class="stat-card">
            <span class="stat-value">5×5</span>
            <span class="stat-label">Linear system solved</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">60</span>
            <span class="stat-label">QUBO bits (5 vars × 12 bits)</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">96</span>
            <span class="stat-label">QUBO bits (Poisson, 3 bits/var)</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">500</span>
            <span class="stat-label">Annealing sweeps per run</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">16</span>
            <span class="stat-label">Independent repetitions</span>
          </div>
          <div class="stat-card">
            <span class="stat-value">8</span>
            <span class="stat-label">Trotter slices (path-integral)</span>
          </div>
        </div>

        <div class="callout muted" style="margin-top:1rem;">
          <p class="callout-title">Team</p>
          <p>Landon Gehr · Melody Goldanloo</p>
          <p>Byron Selvage · Kyle Sperber</p>
          <p style="margin-top:0.4rem; font-size:0.9rem; color:#b09060;">Colorado School of Mines</p>
        </div>
      </aside>
    </section>

  <!-- ── MATH PIPELINE ─────────────────────────────────────────────────────── -->
  {:else if activeTab === "math-pipeline"}
    <section class="page-grid single">
      <article class="panel prose">
        <p class="section-label">Theory</p>
        <h2>From PDE to quantum spin glass — step by step</h2>

        <!-- Step 1 -->
        <div class="math-step">
          <div class="math-step-header">
            <span class="math-step-num">Step 1</span>
            <h3>PDE → Linear System (Finite Differences)</h3>
          </div>
          <p>
            We solve the 2D Poisson equation with Dirichlet boundary conditions on
            Ω = (0, 4π) × (0, 4π):
          </p>
          <div class="formula-block">
            <p>−Δu(x, y) = f(x, y) &nbsp; in Ω</p>
            <p>u(x, y) = g &nbsp; on ∂Ω</p>
          </div>
          <p>
            Discretizing with a 5-point finite-difference stencil on an n-point grid
            (spacing h = 4π/(n+1)):
          </p>
          <div class="formula-block">
            <p>u[i−1,j] + u[i+1,j] + u[i,j−1] + u[i,j+1] − 4u[i,j]</p>
            <p>────────────────────────────────────── = −f[i,j]</p>
            <p style="text-align:right">h²</p>
          </div>
          <p>
            Collecting all interior grid values into a vector <em>u</em> yields the
            sparse linear system <strong>Au = b</strong>, where A is the discretized
            negative Laplacian (a banded, symmetric positive-definite matrix).
          </p>
        </div>

        <!-- Step 2 -->
        <div class="math-step">
          <div class="math-step-header">
            <span class="math-step-num">Step 2</span>
            <h3>Linear System → QUBO (Binary Encoding)</h3>
          </div>
          <p>
            Quantum annealers work with binary variables. Each continuous unknown uᵢ
            is approximated using K bits:
          </p>
          <div class="formula-block">
            <p>uᵢ ≈ u_min + Σₖ bᵢₖ · wₖ</p>
            <p>wₖ = (u_max − u_min) / (2ᴷ − 1) · 2ᵏ</p>
          </div>
          <p>
            Substituting this encoding into the least-squares objective
            ‖Au − b‖² produces a quadratic form purely in binary variables:
          </p>
          <div class="formula-block">
            <p>min ‖Au − b‖²  →  min x<sup>T</sup>Qx</p>
            <p>where Q encodes  A<sup>T</sup>A  and  A<sup>T</sup>b</p>
            <p>Q<sub>ij</sub> = w<sub>i</sub>w<sub>j</sub>(A<sup>T</sup>A)<sub>⌊i/K⌋,⌊j/K⌋</sub></p>
          </div>
          <p>
            Q is called the <strong>QUBO matrix</strong> (Quadratic Unconstrained
            Binary Optimization). It is symmetric and upper-triangular in storage.
          </p>
          <div class="callout">
            <p class="callout-title">Precision tradeoff</p>
            <p>
              K = 3 bits gives 2³ = 8 discrete levels per variable.
              K = 12 bits gives 4096 levels — but also a 4× larger QUBO.
              More bits → better accuracy → exponentially harder to solve.
            </p>
          </div>
        </div>

        <!-- Step 3 -->
        <div class="math-step">
          <div class="math-step-header">
            <span class="math-step-num">Step 3</span>
            <h3>QUBO → Ising Model</h3>
          </div>
          <p>
            Physical quantum annealers use Ising spins sᵢ ∈ &lbrace;−1, +1&rbrace; rather than
            bits bᵢ ∈ &lbrace;0, 1&rbrace;. The substitution bᵢ = (sᵢ + 1)/2 rewrites the QUBO:
          </p>
          <div class="formula-block">
            <p>min x<sup>T</sup>Qx  →  min H(s) = Σ<sub>i&lt;j</sub> J<sub>ij</sub>s<sub>i</sub>s<sub>j</sub> + Σ<sub>i</sub> h<sub>i</sub>s<sub>i</sub> + offset</p>
            <p style="margin-top:0.5rem">J<sub>ij</sub> = Q<sub>ij</sub>/2</p>
            <p>h<sub>i</sub> = Q<sub>ii</sub>/2 + Σ<sub>j≠i</sub> Q<sub>ij</sub>/2</p>
            <p>offset = Σ<sub>i</sub> Q<sub>ii</sub>/2 + Σ<sub>i&lt;j</sub> Q<sub>ij</sub>/2</p>
          </div>
          <p>
            H(s) is the classical Ising Hamiltonian — the energy function the
            annealer will minimize. The couplings Jᵢⱼ encode variable interactions
            and the local fields hᵢ encode linear biases.
          </p>
        </div>

        <!-- Step 4 -->
        <div class="math-step">
          <div class="math-step-header">
            <span class="math-step-num">Step 4</span>
            <h3>Ising → Quantum Annealing (Simulated Tunneling)</h3>
          </div>
          <p>
            Quantum annealing uses a time-dependent Hamiltonian that smoothly
            interpolates from a quantum driver to the problem Hamiltonian:
          </p>
          <div class="formula-block">
            <p>H(t) = A(t) · H₀  +  B(t) · H<sub>P</sub></p>
            <p>H₀ = −Γ Σᵢ σᵢˣ &nbsp;&nbsp; (transverse field — quantum superposition)</p>
            <p>H<sub>P</sub> = H(s) &nbsp;&nbsp;&nbsp;&nbsp;&nbsp; (problem Hamiltonian)</p>
          </div>
          <p>
            At t = 0 the system is in a trivial superposition of all spin states
            (ground state of H₀). As t → T the driver is switched off and the system
            settles into the ground state of Hₚ — our solution.
          </p>
          <p>
            The key quantum advantage: tunneling through energy barriers lets the
            system escape local minima that trap classical simulated annealing.
          </p>
          <div class="formula-block">
            <p>Simulation via path-integral Monte Carlo (Trotter decomposition):</p>
            <p>e<sup>−βH</sup> ≈ (e<sup>−β/P · H₀</sup> · e<sup>−β/P · H<sub>P</sub></sup>)<sup>P</sup></p>
            <p>P = Trotter slices (8 in our runs)</p>
          </div>
          <div class="callout">
            <p class="callout-title">Annealing parameters used</p>
            <p>Initial transverse field Γ₀ = 5.0 &nbsp;|&nbsp; Linear field schedule</p>
            <p>Temperature: T₀ = 10.0 → T_f = 0.01 &nbsp;|&nbsp; Linear schedule</p>
            <p>Sweeps = 500 &nbsp;|&nbsp; Repetitions = 16 &nbsp;|&nbsp; Trotter slices = 8</p>
          </div>
        </div>
      </article>
    </section>

  <!-- ── RESULTS ────────────────────────────────────────────────────────────── -->
  {:else if activeTab === "results"}
    <section class="page-grid">
      <article class="panel prose">
        <p class="section-label">What We Achieved</p>
        <h2>Results and accuracy</h2>

        <h3>Test 1 — 5×5 Sparse Linear System</h3>
        <p>
          The 5×5 example is a hand-constructed sparse system with a known solution.
          We encoded each of the 5 unknowns with 12 bits (4096 levels, range [0, 10])
          producing a 60-bit QUBO.
        </p>
        <div class="formula-block">
          <p>A =</p>
          <p style="font-size:0.88rem; color:#d3d0c7;">
            [1, 3, 0, 2, 0]<br>
            [0, 0, 4, 6, 5]<br>
            [3, 1, 3, 0, 2]<br>
            [1, 0, 0, 0, 3]<br>
            [2, 0, 5, 1, 0]
          </p>
          <p>b = [15, 61, 24, 16, 21]</p>
          <p style="margin-top:0.4rem; color:#d1a05a;">Encoding: u_min=0, u_max=10, k_bits=12 → 60-bit QUBO</p>
        </div>
        <div class="callout">
          <p class="callout-title">Outcome</p>
          <p>
            The solver recovered values with decent accuracy. With 12-bit precision
            (steps of ~0.00244 per unit), the binary resolution is fine enough to
            approach the true solution — the bottleneck is the annealing quality, not
            encoding precision.
          </p>
        </div>

        <h3>Test 2 — Poisson PDE on a Discrete Grid</h3>
        <p>
          Solving −Δu = f on Ω = (0, 4π)² with Dirichlet BCs.
          With n = 32 unknowns and 3 bits/variable, the QUBO has 96 binary variables.
        </p>
        <div class="callout">
          <p class="callout-title">Outcome</p>
          <p>
            We improved over a naive random guess. The solution captures the broad
            shape of the PDE solution, though 3-bit encoding (8 discrete levels per
            variable) limits resolution significantly.
          </p>
        </div>

        <h3>Images — DDPM Training Data &amp; PDE Solutions</h3>
        <div class="img-row">
          <figure class="img-figure">
            <img src="/solution.png" alt="PDE solution visualization" />
            <figcaption>Computed PDE solution field used for DDPM training data</figcaption>
          </figure>
          <figure class="img-figure">
            <img src="/output.png" alt="DDPM output" />
            <figcaption>DDPM model output on test data</figcaption>
          </figure>
        </div>
      </article>

      <aside class="panel">
        <p class="section-label">Limitations</p>
        <h2>Why it's hard</h2>

        <div class="limitation">
          <div class="limitation-icon">⚠</div>
          <div>
            <strong>Bit precision</strong>
            <p>
              K bits encode each variable to 2ᴷ levels. With K = 3 the step size is
              (u_max − u_min)/7. More bits → exponentially larger QUBO → harder to
              anneal classically.
            </p>
          </div>
        </div>

        <div class="limitation">
          <div class="limitation-icon">⚠</div>
          <div>
            <strong>Not guaranteed to converge</strong>
            <p>
              Unlike direct solvers (LU, CG), quantum annealing is a heuristic
              optimizer. It minimizes ‖Au − b‖², but may find a local minimum
              rather than the true solution — especially in simulation.
            </p>
          </div>
        </div>

        <div class="limitation">
          <div class="limitation-icon">⚠</div>
          <div>
            <strong>Limited simulator passes</strong>
            <p>
              Classical simulation of quantum annealing is exponentially expensive.
              With only 500 sweeps and 16 repetitions, the solver does not fully
              explore the 2⁹⁶ state space of the 96-bit Poisson QUBO.
            </p>
          </div>
        </div>

        <div class="limitation">
          <div class="limitation-icon">⚠</div>
          <div>
            <strong>Classical simulation overhead</strong>
            <p>
              We use path-integral Monte Carlo (Trotter slices) to simulate quantum
              tunneling on a CPU. Real QPU hardware would skip this entirely —
              tunneling is native, free, and fast.
            </p>
          </div>
        </div>

        <div class="callout" style="margin-top:1.2rem;">
          <p class="callout-title">Bottom line</p>
          <p>
            The pipeline is <em>correct</em>. The limits are hardware, not math.
            A real QPU anneals hundreds of qubits in microseconds — where our
            classical simulation takes seconds per bit.
          </p>
        </div>
      </aside>
    </section>

  <!-- ── DDPM ────────────────────────────────────────────────────────────────── -->
  {:else if activeTab === "ddpm"}
    <section class="page-grid single">
      <article class="panel prose">
        <p class="section-label">ML Approach</p>
        <h2>DDPM — Denoising Diffusion to boost quantum results</h2>

        <h3>Why we tried this</h3>
        <p>
          Classical simulation of quantum annealing is computationally intractable
          at scale. A 96-bit QUBO has 2⁹⁶ ≈ 10²⁸ possible states. Our simulator
          samples a tiny fraction of this space, so results are noisy.
        </p>
        <p>
          The idea: train a <strong>Denoising Diffusion Probabilistic Model (DDPM)</strong>
          on pairs of (noisy annealing result, true PDE solution) and use it to
          post-process outputs — effectively learning to correct the annealer's errors.
        </p>

        <div class="callout">
          <p class="callout-title">Dual motivation</p>
          <p>
            A DDPM could serve as either a <em>post-processor</em> (clean up annealing
            output) or a <em>surrogate solver</em> (replace expensive annealing
            simulations with fast neural inference), making the pipeline more feasible
            on current hardware.
          </p>
        </div>

        <h3>How DDPM works</h3>
        <div class="math-step" style="margin-top:0">
          <div class="math-step-header">
            <span class="math-step-num">Forward</span>
            <h3>Adding noise (fixed process)</h3>
          </div>
          <p>
            Starting from a clean PDE solution x₀, we progressively add Gaussian
            noise over T steps using a linear schedule &lbrace;βₜ&rbrace;:
          </p>
          <div class="formula-block">
            <p>x_t = √(ᾱ_t) · x₀  +  √(1 − ᾱ_t) · ε,   ε ~ N(0, I)</p>
            <p>ᾱ_t = Π<sub>s=1</sub><sup>t</sup> (1 − β_s)   (cumulative product of kept signal)</p>
            <p>β_t linear from β₁ = 10⁻⁴ to β_T = 2×10⁻²</p>
          </div>
        </div>

        <div class="math-step">
          <div class="math-step-header">
            <span class="math-step-num">Reverse</span>
            <h3>Denoising (learned process)</h3>
          </div>
          <p>
            A neural network ε̂_θ(x_t, t) is trained to predict the noise added at
            each step. During inference, we iteratively remove noise:
          </p>
          <div class="formula-block">
            <p>x_&lbrace;t−1&rbrace; = (1/√α_t) · (x_t − β_t/√(1−ᾱ_t) · ε̂_θ(x_t, t))  +  σ_t · z</p>
            <p>z ~ N(0, I),   σ_t = √β_t   (stochastic term)</p>
          </div>
          <p>
            Training minimizes the MSE between the predicted noise and the true noise:
          </p>
          <div class="formula-block">
            <p>L = E[‖ε − ε̂_θ(√(ᾱ_t) x₀ + √(1−ᾱ_t) ε, t)‖²]</p>
          </div>
        </div>

        <h3>What we implemented</h3>
        <p>
          We implemented the full DDPM forward/reverse pipeline in Python
          (<code>mlp-test/diffusiontemp.py</code>) and generated PDE solution data
          for training. The Rust side (<code>mlp-test/src/main.rs</code>) uses the
          Burn deep learning framework with WGPU GPU backend for model training.
        </p>
        <p>
          Training data was generated from the classical Rust solver
          (<code>bhqa/src/bin/Classical_Solver.rs</code>), which produces ground-truth
          Poisson solutions that DDPM learns to recover from noisy inputs.
        </p>

        <h3>Why it's still challenging</h3>
        <p>
          Even with DDPM, full integration with the quantum annealing pipeline
          remains computationally demanding. The QUBO problem space is discrete and
          high-dimensional, making it hard to define a meaningful "noise" process
          over binary variables. We explored the continuous relaxation, but connecting
          the diffusion output back to valid binary solutions requires additional
          rounding or projection steps that introduce their own errors.
        </p>
        <div class="callout muted">
          <p class="callout-title">Key insight</p>
          <p>
            DDPM is most promising as a learned post-processor: run the quantum
            annealer cheaply (few sweeps), then apply the trained diffusion model to
            refine the output. This hybrid approach decouples hardware access from
            solution quality.
          </p>
        </div>
      </article>
    </section>

  <!-- ── ALLEN-CAHN ─────────────────────────────────────────────────────────── -->
  {:else if activeTab === "allen-cahn"}
    <section class="page-grid">
      <article class="panel prose">
        <p class="section-label">Motivation</p>
        <h2>Allen-Cahn — why fast PDE solvers matter</h2>

        <h3>What is Allen-Cahn?</h3>
        <p>
          The Allen-Cahn equation is a phase-field PDE that models the evolution of
          an order parameter u ∈ [−1, +1] representing two competing phases (e.g.,
          solid/liquid, two grain orientations in a metal):
        </p>
        <div class="formula-block">
          <p>∂u/∂t = ε² Δu  −  u³  +  2u &nbsp; in Ω × (0, T]</p>
          <p>where  F'(u) = u³ − 2u  is the derivative of the double-well potential</p>
          <p>F(u) = ¼(u²−1)²</p>
        </div>
        <p>
          The parameter ε controls interface thickness. As the solution evolves,
          regions of u ≈ +1 and u ≈ −1 form and coarsen, separated by thin
          diffuse interfaces.
        </p>

        <h3>Our interactive Python solver</h3>
        <p>
          We built a matplotlib-based interactive solver in
          <code>Alen-Cahn/drawer.py</code>. The user draws the initial phase
          configuration by clicking on a 128×128 canvas; the solver then:
        </p>
        <ul class="feature-list">
          <li>Assembles the 2D finite-difference Laplacian via Kronecker products</li>
          <li>Solves the linear diffusion sub-step implicitly (Krylov method via scipy)</li>
          <li>Applies the nonlinear reaction step explicitly each time step</li>
          <li>Animates the evolving field over t ∈ [0, 50]</li>
        </ul>
        <div class="callout">
          <p class="callout-title">Parameters</p>
          <p>Grid: 128 × 128 &nbsp;|&nbsp; x, y ∈ [0, 5]</p>
          <p>Time: t ∈ [0, 50], 100 steps &nbsp;|&nbsp; ε = Δx (interface ≈ one grid cell)</p>
          <p>Initial conditions: user-drawn ±1 mask with 0.8 amplitude noise</p>
        </div>

        <h3>Why we care about quantum speedups here</h3>
        <p>
          Allen-Cahn is a parabolic PDE — at each time step we solve a large linear
          system. For real-world materials simulations (grain growth, solidification),
          grids of 1024² to 4096² are common, generating systems with millions of
          unknowns. Classical solvers spend 80–95% of wall-clock time on these
          linear solves.
        </p>
        <p>
          If quantum annealers can solve large-scale optimization problems faster than
          classical Krylov methods, every time step of Allen-Cahn becomes faster —
          enabling higher spatial resolution, longer time horizons, or real-time
          materials design.
        </p>
      </article>

      <aside class="panel">
        <p class="section-label">QPU Feasibility</p>
        <h2>What full-scale Allen-Cahn on a QPU would take</h2>

        <div class="limitation">
          <div class="limitation-icon">📐</div>
          <div>
            <strong>System size</strong>
            <p>
              128×128 grid → 16,384 interior unknowns. With 12-bit encoding that's
              a <em>196,608-bit QUBO</em> per time step. Way beyond current simulators.
            </p>
          </div>
        </div>

        <div class="limitation">
          <div class="limitation-icon">⏱</div>
          <div>
            <strong>Repeated solves</strong>
            <p>
              Allen-Cahn requires solving a linear system at every time step (100+
              steps in our demo). Each must be encoded, annealed, and decoded
              independently.
            </p>
          </div>
        </div>

        <div class="limitation">
          <div class="limitation-icon">🔗</div>
          <div>
            <strong>Connectivity constraints</strong>
            <p>
              Real QPU chips (D-Wave, etc.) have limited qubit connectivity. The
              dense QUBO from a 2D Laplacian requires heavy minor-embedding, often
              multiplying qubit count 5–10×.
            </p>
          </div>
        </div>

        <div class="limitation">
          <div class="limitation-icon">🔌</div>
          <div>
            <strong>Hardware access</strong>
            <p>
              D-Wave 2000Q has ~2000 qubits; Advantage has ~5000. A 196k-qubit
              problem far exceeds any current device. The calculation requires
              next-generation hardware.
            </p>
          </div>
        </div>

        <div class="callout" style="margin-top:1.2rem;">
          <p class="callout-title">Our scope</p>
          <p>
            Our 32-unknown Poisson solve is a proof-of-concept on a very discrete
            grid. It demonstrates the pipeline is <em>correct</em> — scaling to
            Allen-Cahn resolution requires proportionally more qubits, not a
            different algorithm.
          </p>
        </div>
      </aside>
    </section>

  <!-- ── POISSON SETUP (existing interactive) ──────────────────────────────── -->
  {:else if activeTab === "poisson"}
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

  <!-- ── CONCLUSION ─────────────────────────────────────────────────────────── -->
  {:else if activeTab === "conclusion"}
    <section class="page-grid single">
      <article class="panel prose">
        <p class="section-label">Takeaway</p>
        <h2>What we showed — and where it goes next</h2>

        <div class="callout">
          <p class="callout-title">Core result</p>
          <p>
            We implemented a complete, working quantum annealing pipeline for PDE solving.
            The math is sound, the code runs, and the solver finds approximate solutions.
            <strong>With appropriate QPU hardware, this approach would work well.</strong>
          </p>
        </div>

        <h3>What we demonstrated</h3>
        <ul class="feature-list">
          <li>Finite-difference discretization of a 2D Poisson equation → Au = b</li>
          <li>Binary QUBO encoding with variable bit depth (3–12 bits per unknown)</li>
          <li>Exact QUBO-to-Ising conversion preserving solution equivalence</li>
          <li>Path-integral Monte Carlo quantum annealing simulation via quantrs2_anneal</li>
          <li>Recovery of continuous solution values from Ising spin measurements</li>
          <li>Prototype DDPM pipeline for refining noisy quantum outputs</li>
          <li>Interactive Allen-Cahn solver demonstrating the real-world PDE motivation</li>
        </ul>

        <h3>Current limitations</h3>
        <p>
          Every limitation we hit is a hardware limitation, not an algorithmic one.
          Classical simulation of quantum tunneling is exponentially expensive — the
          same computation that takes our CPU seconds would take a real QPU microseconds.
          Our 96-bit Poisson QUBO barely scratches the surface of what a 5000-qubit
          D-Wave Advantage could handle.
        </p>

        <div class="math-step">
          <div class="math-step-header">
            <span class="math-step-num">The gap</span>
            <h3>Simulation vs. real hardware</h3>
          </div>
          <div class="formula-block">
            <p>Classical simulation: O(2ⁿ) state-space exploration — exponential cost</p>
            <p>Real QPU:            O(1) tunneling — the quantum state evolves natively</p>
            <p>Our 96-bit QUBO: feasible on D-Wave Advantage (5000+ physical qubits)</p>
            <p>Allen-Cahn (128²): needs next-generation hardware (&gt;200k logical qubits)</p>
          </div>
        </div>

        <h3>The path forward</h3>
        <ul class="feature-list">
          <li>
            <strong>Near-term:</strong> Run on real QPU hardware (D-Wave cloud access) to
            validate the Poisson and Allen-Cahn pipelines at meaningful grid sizes.
          </li>
          <li>
            <strong>DDPM integration:</strong> Train the diffusion model on larger datasets
            and integrate it as a post-processor to correct limited-sweep QPU outputs.
          </li>
          <li>
            <strong>Bit depth scaling:</strong> Explore adaptive encoding — more bits where
            the solution varies rapidly, fewer where it is smooth — to reduce QUBO size
            without sacrificing accuracy.
          </li>
          <li>
            <strong>Time-dependent PDEs:</strong> Extend the pipeline to Allen-Cahn and
            other parabolic problems by solving one time step per annealing call.
          </li>
        </ul>

        <div class="callout" style="margin-top:1.5rem;">
          <p class="callout-title">Final message</p>
          <p>
            Quantum annealing for PDEs is not a distant dream — it is a pipeline
            we built and ran. The classical simulation bottleneck is real, but it is
            temporary. As QPU hardware matures, every piece of this pipeline scales
            directly to meaningful problem sizes.
          </p>
          <p style="margin-top:0.6rem; color:#d1a05a;">
            Landon Gehr · Melody Goldanloo · Byron Selvage · Kyle Sperber
          </p>
        </div>
      </article>
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
    min-width: 10rem;
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
    margin: 0.75rem 0;
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

  /* ── Pipeline diagram ─────────────────────────────────── */
  .pipeline {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    margin: 1rem 0;
  }

  .pipeline-step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    background: rgba(250, 245, 232, 0.06);
    border: 1px solid rgba(236, 215, 177, 0.18);
    border-radius: 1rem;
    padding: 0.7rem 0.9rem;
    min-width: 7rem;
    text-align: center;
  }

  .step-num {
    font-size: 0.68rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: #d1a05a;
    font-weight: 600;
  }

  .pipeline-step strong {
    font-size: 0.95rem;
    color: #f5efe0;
  }

  .step-sub {
    font-size: 0.8rem;
    color: #a09070;
    font-family: "STIX Two Text", "Times New Roman", serif;
  }

  .step-arrow {
    color: #c47a30;
    font-size: 1.4rem;
    flex-shrink: 0;
  }

  /* ── Stat grid ────────────────────────────────────────── */
  .stat-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.7rem;
    margin-top: 0.75rem;
  }

  .stat-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: rgba(250, 245, 232, 0.05);
    border: 1px solid rgba(236, 215, 177, 0.12);
    border-radius: 0.9rem;
    padding: 0.65rem 0.5rem;
    text-align: center;
  }

  .stat-value {
    font-size: 1.6rem;
    font-weight: 700;
    color: #f6d49d;
    line-height: 1.1;
  }

  .stat-label {
    font-size: 0.76rem;
    color: #a09070;
    margin-top: 0.2rem;
    line-height: 1.3;
  }

  /* ── Math steps ───────────────────────────────────────── */
  .math-step {
    border-left: 3px solid rgba(196, 122, 48, 0.5);
    padding-left: 1.1rem;
    margin: 1.5rem 0;
  }

  .math-step-header {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .math-step-num {
    font-size: 0.7rem;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: #d1a05a;
    font-weight: 700;
    white-space: nowrap;
  }

  .math-step h3 {
    margin: 0;
    font-size: 1.1rem;
    color: #f5efe0;
  }

  /* ── Limitations ──────────────────────────────────────── */
  .limitation {
    display: flex;
    gap: 0.75rem;
    align-items: flex-start;
    padding: 0.75rem 0;
    border-bottom: 1px solid rgba(236, 215, 177, 0.08);
  }

  .limitation:last-of-type {
    border-bottom: none;
  }

  .limitation-icon {
    font-size: 1.2rem;
    flex-shrink: 0;
    width: 1.8rem;
    text-align: center;
  }

  .limitation strong {
    display: block;
    margin-bottom: 0.2rem;
    color: #f0e3c9;
  }

  .limitation p {
    margin: 0;
    font-size: 0.9rem;
    line-height: 1.5;
    color: #b0a898;
  }

  /* ── Feature list ─────────────────────────────────────── */
  .feature-list {
    padding-left: 1.2rem;
    color: #d3d0c7;
    line-height: 1.7;
  }

  .feature-list li {
    margin: 0.3rem 0;
  }

  /* ── Images ───────────────────────────────────────────── */
  .img-row {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    margin: 1rem 0;
  }

  .img-figure {
    flex: 1 1 12rem;
    margin: 0;
  }

  .img-figure img {
    width: 100%;
    border-radius: 0.8rem;
    border: 1px solid rgba(236, 215, 177, 0.14);
  }

  .img-figure figcaption {
    font-size: 0.82rem;
    color: #a09070;
    margin-top: 0.35rem;
    text-align: center;
  }

  /* ── Code inline ──────────────────────────────────────── */
  code {
    font-family: "SF Mono", "Cascadia Code", "Consolas", monospace;
    font-size: 0.88em;
    background: rgba(250, 245, 232, 0.08);
    border-radius: 0.3rem;
    padding: 0.05em 0.3em;
    color: #d1a05a;
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
      flex: 1 1 10rem;
    }

    .pipeline {
      flex-direction: column;
      align-items: flex-start;
    }

    .step-arrow {
      transform: rotate(90deg);
    }
  }
</style>

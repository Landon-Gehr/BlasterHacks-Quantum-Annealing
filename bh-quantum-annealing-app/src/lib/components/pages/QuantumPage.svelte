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
          This page will explain how the PDE-inspired optimization problem is encoded into a QUBO,
          translated into Ising form, and then handed to a quantum annealing style solver.
        </p>
        <p>
          It should also carry the short proof-of-concept story: the theory works, but simulating
          the relevant quantum processes on classical hardware is still computationally impractical
          to scale.
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

  .pixel-display {
    font-family: "Glastone", "Dotemp", monospace;
    font-size: clamp(2rem, 5vw, 3.8rem);
    line-height: 0.9;
    color: #ffe972;
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

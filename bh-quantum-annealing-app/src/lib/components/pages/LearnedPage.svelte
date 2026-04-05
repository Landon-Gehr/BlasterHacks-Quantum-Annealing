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
    color: #240914;
    text-transform: uppercase;
    font-size: 0.8rem;
    letter-spacing: 0.08em;
    background: linear-gradient(90deg, #ef70b7, #9c3f72);
  }

  .window-controls {
    letter-spacing: 0.2em;
  }

  .window-body {
    padding: 1rem 1.1rem 1.15rem;
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

  h2 {
    margin: 0.25rem 0 0.75rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.05;
  }

  p,
  li {
    color: #efdaf9;
    line-height: 1.65;
  }

  .pixel-card {
    border: 2px solid rgba(255, 255, 255, 0.08);
    padding: 0.9rem;
    background: rgba(92, 190, 255, 0.08);
  }

  @media (max-width: 980px) {
    .page-shell {
      padding: 0.8rem;
    }
  }
</style>

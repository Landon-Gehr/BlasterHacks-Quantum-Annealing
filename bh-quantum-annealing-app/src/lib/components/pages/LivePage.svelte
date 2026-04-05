<script lang="ts">
  import AllenCahnDrawer from "$lib/components/AllenCahnDrawer.svelte";
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
      <span>Allen-Cahn drawer</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body stacked">
      <div class="compact-intro">
        <p class="section-kicker">Initial state editor</p>
        <h3>Draw the mask that seeds the evolution</h3>
        <p>
          This is the browser version of the drawer from the <code>Alen-Cahn</code> folder. Paint
          the initial condition here, then use that mask as the starting state for the Allen-Cahn
          simulation.
        </p>
      </div>
      <AllenCahnDrawer />
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

  @media (max-width: 980px) {
    .page-shell {
      padding: 0.8rem;
    }
  }
</style>

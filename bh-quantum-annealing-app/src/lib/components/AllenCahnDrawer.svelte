<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  const GRID_SIZE = 128;
  const EPSILON = 0.85;
  const TIME_STEP = 0.08;
  const SUBSTEPS_PER_FRAME = 3;

  let drawCanvas: HTMLCanvasElement | null = null;
  let drawContext: CanvasRenderingContext2D | null = null;
  let simCanvas: HTMLCanvasElement | null = null;
  let simContext: CanvasRenderingContext2D | null = null;
  let mask = new Uint8Array(GRID_SIZE * GRID_SIZE);
  let field = new Float32Array(GRID_SIZE * GRID_SIZE);

  let brushRadius = $state(2);
  let drawing = $state(false);
  let filledPixels = $state(0);
  let simulationRunning = $state(false);
  let framesRendered = $state(0);
  let animationFrameId: number | null = null;
  let simStatus = $state("Draw an initial condition, then run the evolution.");

  function cellIndex(row: number, col: number): number {
    return row * GRID_SIZE + col;
  }

  function updateFilledPixels() {
    filledPixels = mask.reduce((total, value) => total + value, 0);
  }

  function renderMask() {
    if (!drawContext) {
      return;
    }

    const image = drawContext.createImageData(GRID_SIZE, GRID_SIZE);

    for (let row = 0; row < GRID_SIZE; row += 1) {
      for (let col = 0; col < GRID_SIZE; col += 1) {
        const offset = 4 * cellIndex(row, col);
        const isFilled = mask[cellIndex(row, col)] === 1;

        if (isFilled) {
          image.data[offset] = 255;
          image.data[offset + 1] = 111;
          image.data[offset + 2] = 180;
          image.data[offset + 3] = 255;
        } else {
          image.data[offset] = 10;
          image.data[offset + 1] = 12;
          image.data[offset + 2] = 18;
          image.data[offset + 3] = 255;
        }
      }
    }

    drawContext.putImageData(image, 0, 0);
  }

  function paintAt(row: number, col: number) {
    let changed = false;

    for (let r = row - brushRadius; r <= row + brushRadius; r += 1) {
      for (let c = col - brushRadius; c <= col + brushRadius; c += 1) {
        const inBounds = r >= 0 && r < GRID_SIZE && c >= 0 && c < GRID_SIZE;
        const inBrush = (r - row) ** 2 + (c - col) ** 2 <= brushRadius ** 2;

        if (inBounds && inBrush) {
          const index = cellIndex(r, c);
          if (mask[index] === 0) {
            mask[index] = 1;
            changed = true;
          }
        }
      }
    }

    if (changed) {
      updateFilledPixels();
      renderMask();
    }
  }

  function pointerToCell(event: PointerEvent): { row: number; col: number } | null {
    if (!drawCanvas) {
      return null;
    }

    const rect = drawCanvas.getBoundingClientRect();
    const normalizedX = (event.clientX - rect.left) / rect.width;
    const normalizedY = (event.clientY - rect.top) / rect.height;

    if (normalizedX < 0 || normalizedX > 1 || normalizedY < 0 || normalizedY > 1) {
      return null;
    }

    const col = Math.max(0, Math.min(GRID_SIZE - 1, Math.floor(normalizedX * GRID_SIZE)));
    const row = Math.max(0, Math.min(GRID_SIZE - 1, Math.floor(normalizedY * GRID_SIZE)));

    return { row, col };
  }

  function handlePointerDown(event: PointerEvent) {
    const cell = pointerToCell(event);
    if (!cell || !drawCanvas || simulationRunning) {
      return;
    }

    drawing = true;
    drawCanvas.setPointerCapture(event.pointerId);
    paintAt(cell.row, cell.col);
  }

  function handlePointerMove(event: PointerEvent) {
    if (!drawing) {
      return;
    }

    const cell = pointerToCell(event);
    if (!cell) {
      return;
    }

    paintAt(cell.row, cell.col);
  }

  function handlePointerUp(event: PointerEvent) {
    drawing = false;
    drawCanvas?.releasePointerCapture(event.pointerId);
  }

  function clearCanvas() {
    stopEvolution();
    mask = new Uint8Array(GRID_SIZE * GRID_SIZE);
    updateFilledPixels();
    renderMask();
    clearSimulationView();
    simStatus = "Canvas cleared.";
  }

  function seedDemo() {
    clearCanvas();

    const center = GRID_SIZE / 2;
    const radius = GRID_SIZE / 5;

    for (let row = 0; row < GRID_SIZE; row += 1) {
      for (let col = 0; col < GRID_SIZE; col += 1) {
        const dx = col - center;
        const dy = row - center;
        if (dx * dx + dy * dy <= radius * radius) {
          mask[cellIndex(row, col)] = 1;
        }
      }
    }

    updateFilledPixels();
    renderMask();
    simStatus = "Loaded a sample mask.";
  }

  function clearSimulationView() {
    if (!simContext) {
      return;
    }

    simContext.fillStyle = "#0a0c12";
    simContext.fillRect(0, 0, GRID_SIZE, GRID_SIZE);
  }

  function clampFieldValue(value: number): number {
    return Math.max(-1.5, Math.min(1.5, value));
  }

  function colorForValue(value: number): [number, number, number] {
    const t = (clampFieldValue(value) + 1.5) / 3.0;

    if (t < 0.33) {
      const local = t / 0.33;
      return [
        Math.round(30 + 70 * local),
        Math.round(12 + 30 * local),
        Math.round(70 + 140 * local)
      ];
    }

    if (t < 0.66) {
      const local = (t - 0.33) / 0.33;
      return [
        Math.round(100 + 155 * local),
        Math.round(42 + 70 * local),
        Math.round(210 - 120 * local)
      ];
    }

    const local = (t - 0.66) / 0.34;
    return [
      255,
      Math.round(112 + 140 * local),
      Math.round(90 - 40 * local)
    ];
  }

  function renderField() {
    if (!simContext) {
      return;
    }

    const image = simContext.createImageData(GRID_SIZE, GRID_SIZE);

    for (let row = 0; row < GRID_SIZE; row += 1) {
      for (let col = 0; col < GRID_SIZE; col += 1) {
        const offset = 4 * cellIndex(row, col);
        const [red, green, blue] = colorForValue(field[cellIndex(row, col)]);
        image.data[offset] = red;
        image.data[offset + 1] = green;
        image.data[offset + 2] = blue;
        image.data[offset + 3] = 255;
      }
    }

    simContext.putImageData(image, 0, 0);
  }

  function initializeFieldFromMask() {
    const seededField = new Float32Array(GRID_SIZE * GRID_SIZE);

    for (let index = 0; index < seededField.length; index += 1) {
      const base = mask[index] === 1 ? 1 : -1;
      const noise = (Math.random() - 0.5) * 0.35;
      seededField[index] = clampFieldValue(base + noise);
    }

    field = seededField;
  }

  function evolveOneStep() {
    const next = new Float32Array(field.length);
    const epsilonSquared = EPSILON * EPSILON;

    for (let row = 0; row < GRID_SIZE; row += 1) {
      for (let col = 0; col < GRID_SIZE; col += 1) {
        const index = cellIndex(row, col);
        const center = field[index];
        const left = field[cellIndex(row, Math.max(col - 1, 0))];
        const right = field[cellIndex(row, Math.min(col + 1, GRID_SIZE - 1))];
        const up = field[cellIndex(Math.max(row - 1, 0), col)];
        const down = field[cellIndex(Math.min(row + 1, GRID_SIZE - 1), col)];
        const laplacian = left + right + up + down - 4 * center;
        const reaction = center ** 3 - 2 * center;

        next[index] = clampFieldValue(center + TIME_STEP * (epsilonSquared * laplacian - reaction));
      }
    }

    field = next;
  }

  function tickEvolution() {
    if (!simulationRunning) {
      return;
    }

    for (let step = 0; step < SUBSTEPS_PER_FRAME; step += 1) {
      evolveOneStep();
    }

    framesRendered += 1;
    simStatus = `Allen-Cahn evolution running. Frame ${framesRendered}.`;
    renderField();
    animationFrameId = window.requestAnimationFrame(tickEvolution);
  }

  function stopEvolution() {
    simulationRunning = false;
    if (animationFrameId !== null) {
      window.cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
  }

  function runEvolution() {
    stopEvolution();
    initializeFieldFromMask();
    framesRendered = 0;
    simulationRunning = true;
    simStatus = "Allen-Cahn evolution running.";
    renderField();
    animationFrameId = window.requestAnimationFrame(tickEvolution);
  }

  onMount(() => {
    if (!drawCanvas || !simCanvas) {
      return;
    }

    drawContext = drawCanvas.getContext("2d");
    simContext = simCanvas.getContext("2d");
    renderMask();
    clearSimulationView();
  });

  onDestroy(() => {
    stopEvolution();
  });
</script>

<section class="drawer-shell">
  <div class="drawer-meta">
    <div>
      <p class="meta-label">Initial Condition</p>
      <p class="meta-value">{filledPixels} active pixels</p>
    </div>
  </div>

  <div class="canvas-row">
    <div class="retro-window mini-window">
      <div class="window-bar pink">
        <span>Initial Condition</span>
        <span class="window-controls">_ □ ×</span>
      </div>
      <div class="window-body">
        <div class="frame-label">
          <span>draw</span>
          <span>{filledPixels} active pixels</span>
        </div>
        <div class="canvas-frame">
          <canvas
            bind:this={drawCanvas}
            class="draw-canvas"
            width={GRID_SIZE}
            height={GRID_SIZE}
            onpointerdown={handlePointerDown}
            onpointermove={handlePointerMove}
            onpointerup={handlePointerUp}
            onpointerleave={handlePointerUp}
            onpointercancel={handlePointerUp}
          ></canvas>
        </div>
      </div>
    </div>

    <div class="retro-window mini-window">
      <div class="window-bar blue">
        <span>Evolution Preview</span>
        <span class="window-controls">_ □ ×</span>
      </div>
      <div class="window-body">
        <div class="frame-label">
          {#if simulationRunning}
            <span>live</span>
          {:else}
            <span>idle</span>
          {/if}
          <span>frame {framesRendered}</span>
        </div>
        <div class="canvas-frame evolution-frame">
          <canvas
            bind:this={simCanvas}
            class="draw-canvas simulation-canvas"
            width={GRID_SIZE}
            height={GRID_SIZE}
          ></canvas>
        </div>
      </div>
    </div>
  </div>

  <div class="drawer-actions">
    <label class="brush-control">
      <span>Brush radius</span>
      <input type="range" min="1" max="8" bind:value={brushRadius} />
      <strong>{brushRadius}</strong>
    </label>
    <button type="button" class="primary" onclick={runEvolution}>Run evolution</button>
    <button type="button" onclick={stopEvolution}>Pause</button>
    <button type="button" class="primary" onclick={seedDemo}>Load sample</button>
    <button type="button" onclick={clearCanvas}>Clear</button>
  </div>

  <p class="drawer-note">{simStatus}</p>
  <!-- <p class="drawer-note">
    Add note here.
  </p> -->
</section>

<style>
  .drawer-shell {
    display: grid;
    gap: 1rem;
  }

  .canvas-row {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
    align-items: start;
  }

  .drawer-meta {
    display: flex;
    justify-content: space-between;
    align-items: end;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .meta-label {
    margin: 0 0 0.15rem;
    color: #84c3ff;
    font-size: 0.72rem;
    letter-spacing: 0.16em;
    text-transform: uppercase;
  }

  .meta-value {
    margin: 0;
    color: #fff6a8;
    font-family: "Glastone", "Dotemp", monospace;
    font-size: 1.15rem;
  }

  .brush-control {
    display: flex;
    align-items: center;
    gap: 0.7rem;
    color: #efdaf9;
  }

  .brush-control input {
    width: 10rem;
    accent-color: #ff5ea9;
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

  .mini-window {
    overflow: hidden;
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
  }

  .window-bar.pink {
    background: linear-gradient(90deg, #ef70b7, #9c3f72);
  }

  .window-bar.blue {
    background: linear-gradient(90deg, #6ab6ff, #2f5cff);
    color: #081327;
  }

  .window-controls {
    letter-spacing: 0.2em;
  }

  .window-body {
    display: grid;
    gap: 0.7rem;
    padding: 0.85rem;
  }

  .canvas-frame {
    width: 100%;
    padding: 0.5rem;
    border: 2px solid rgba(255, 255, 255, 0.08);
    background:
      linear-gradient(180deg, rgba(44, 155, 255, 0.08), rgba(255, 72, 130, 0.08)),
      rgba(255, 255, 255, 0.03);
    box-shadow: inset 0 0 0 2px rgba(255, 255, 255, 0.03);
  }

  .evolution-frame {
    display: grid;
    gap: 0.55rem;
  }

  .frame-label {
    display: flex;
    justify-content: space-between;
    color: #fff6a8;
    font-size: 0.78rem;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  .draw-canvas {
    display: block;
    width: 100%;
    aspect-ratio: 1;
    border: 2px solid #332b4b;
    background:
      linear-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255, 255, 255, 0.05) 1px, transparent 1px),
      #0a0c12;
    background-size: 16px 16px;
    cursor: crosshair;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
  }

  .simulation-canvas {
    cursor: default;
  }

  .drawer-actions {
    display: flex;
    gap: 0.8rem;
    flex-wrap: wrap;
  }

  .drawer-actions button {
    border: 2px solid #38294f;
    background: linear-gradient(180deg, #181420, #100d16);
    color: #fbeeff;
    padding: 0.7rem 1rem;
    cursor: pointer;
    box-shadow: 4px 4px 0 #06050a;
  }

  .drawer-actions .primary {
    background: linear-gradient(180deg, #6ab6ff, #2f5cff);
    color: #081327;
  }

  .drawer-note {
    margin: 0;
    color: #cdb8d9;
    line-height: 1.6;
  }

  @media (max-width: 720px) {
    .brush-control {
      width: 100%;
      justify-content: space-between;
    }

    .brush-control input {
      width: min(100%, 8rem);
    }

    .canvas-row {
      grid-template-columns: minmax(0, 1fr);
    }
  }
</style>

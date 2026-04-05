<script lang="ts">
  import { onDestroy, onMount } from "svelte";

  const GRID_SIZE = 125;
  const TIME_STEP = 0.05;
  const SUBSTEPS_PER_FRAME = 1;
  const DX = 1 / (GRID_SIZE - 1);
  const DY = DX;
  const EPSILON = DX;
  const SOLVER_TOLERANCE = 1e-4;
  const SOLVER_MAX_ITERATIONS = 80;
  const BASE_AMPLITUDE = 0.03;
  const NOISE_AMPLITUDE = 0.01;

  let drawCanvas: HTMLCanvasElement | null = null;
  let drawContext: CanvasRenderingContext2D | null = null;
  let simCanvas: HTMLCanvasElement | null = null;
  let simContext: CanvasRenderingContext2D | null = null;
  let mask = new Uint8Array(GRID_SIZE * GRID_SIZE);
  let field = new Float32Array(GRID_SIZE * GRID_SIZE);
  let initialField = new Float32Array(GRID_SIZE * GRID_SIZE);

  let brushRadius = $state(4);
  let drawing = $state(false);
  let filledPixels = $state(0);
  let simulationRunning = $state(false);
  let framesRendered = $state(0);
  let simulatedTime = $state(0);
  let currentMass = $state(0);
  let animationFrameId: number | null = null;
  let simStatus = $state("Draw an initial condition, then run the evolution.");

  function cellIndex(row: number, col: number): number {
    return row * GRID_SIZE + col;
  }

  function updateFilledPixels() {
    filledPixels = mask.reduce((total, value) => total + value, 0);
  }

  function pseudoRandom(index: number): number {
    const value = Math.sin((index + 1) * 12.9898) * 43758.5453;
    return value - Math.floor(value);
  }

  function buildFieldFromMask(): Float32Array {
    const seededField = new Float32Array(GRID_SIZE * GRID_SIZE);
    let sum = 0;

    for (let index = 0; index < seededField.length; index += 1) {
      const base = mask[index] === 1 ? BASE_AMPLITUDE : -BASE_AMPLITUDE;
      const noise = (pseudoRandom(index) - 0.5) * NOISE_AMPLITUDE;
      seededField[index] = base + noise;
      sum += seededField[index];
    }

    const mean = sum / seededField.length;
    for (let index = 0; index < seededField.length; index += 1) {
      seededField[index] = clampFieldValue(seededField[index] - mean);
    }

    return seededField;
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
    field = new Float32Array(GRID_SIZE * GRID_SIZE);
    initialField = new Float32Array(GRID_SIZE * GRID_SIZE);
    currentMass = 0;
    simulatedTime = 0;
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

  function fieldMass(values: Float32Array): number {
    let total = 0;
    for (let index = 0; index < values.length; index += 1) {
      total += values[index];
    }
    return total * DX * DY;
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
    const seededField = buildFieldFromMask();

    field = seededField.slice();
    initialField = seededField.slice();
    currentMass = fieldMass(field);
    simulatedTime = 0;
  }

  function applyLaplacian(input: Float32Array, output: Float32Array) {
    const inverseDxSquared = 1 / (DX * DX);

    for (let row = 0; row < GRID_SIZE; row += 1) {
      for (let col = 0; col < GRID_SIZE; col += 1) {
        const index = cellIndex(row, col);
        const center = input[index];

        const horizontal = col === 0
          ? 2 * (input[cellIndex(row, 1)] - center)
          : col === GRID_SIZE - 1
            ? 2 * (input[cellIndex(row, GRID_SIZE - 2)] - center)
            : input[cellIndex(row, col - 1)] - 2 * center + input[cellIndex(row, col + 1)];

        const vertical = row === 0
          ? 2 * (input[cellIndex(1, col)] - center)
          : row === GRID_SIZE - 1
            ? 2 * (input[cellIndex(GRID_SIZE - 2, col)] - center)
            : input[cellIndex(row - 1, col)] - 2 * center + input[cellIndex(row + 1, col)];

        output[index] = (horizontal + vertical) * inverseDxSquared;
      }
    }
  }

  function applyBackwardEulerMatrix(input: Float32Array, output: Float32Array) {
    const laplacian = new Float32Array(input.length);
    const diffusionScale = TIME_STEP * EPSILON * EPSILON;

    applyLaplacian(input, laplacian);

    for (let index = 0; index < input.length; index += 1) {
      output[index] = input[index] - diffusionScale * laplacian[index];
    }
  }

  function dot(left: Float32Array, right: Float32Array): number {
    let total = 0;
    for (let index = 0; index < left.length; index += 1) {
      total += left[index] * right[index];
    }
    return total;
  }

  function solveBackwardEuler(rhs: Float32Array, guess: Float32Array): Float32Array {
    const solution = guess.slice();
    const matVec = new Float32Array(rhs.length);
    const residual = new Float32Array(rhs.length);
    const direction = new Float32Array(rhs.length);
    const matrixDirection = new Float32Array(rhs.length);

    applyBackwardEulerMatrix(solution, matVec);
    for (let index = 0; index < rhs.length; index += 1) {
      residual[index] = rhs[index] - matVec[index];
      direction[index] = residual[index];
    }

    let residualNormSq = dot(residual, residual);
    if (Math.sqrt(residualNormSq) < SOLVER_TOLERANCE) {
      return solution;
    }

    for (let iteration = 0; iteration < SOLVER_MAX_ITERATIONS; iteration += 1) {
      applyBackwardEulerMatrix(direction, matrixDirection);
      const denom = dot(direction, matrixDirection);
      if (Math.abs(denom) < 1e-12) {
        break;
      }

      const alpha = residualNormSq / denom;
      for (let index = 0; index < solution.length; index += 1) {
        solution[index] += alpha * direction[index];
        residual[index] -= alpha * matrixDirection[index];
      }

      const nextResidualNormSq = dot(residual, residual);
      if (Math.sqrt(nextResidualNormSq) < SOLVER_TOLERANCE) {
        break;
      }

      const beta = nextResidualNormSq / residualNormSq;
      for (let index = 0; index < direction.length; index += 1) {
        direction[index] = residual[index] + beta * direction[index];
      }
      residualNormSq = nextResidualNormSq;
    }

    return solution;
  }

  function reactionStep(values: Float32Array): Float32Array {
    const next = new Float32Array(values.length);
    const decay = Math.exp(-2 * TIME_STEP);

    for (let index = 0; index < values.length; index += 1) {
      const value = values[index];
      const denom = Math.sqrt(value * value + (1 - value * value) * decay);
      next[index] = denom > 1e-12 ? clampFieldValue(value / denom) : 0;
    }

    return next;
  }

  function conservativeCorrection(values: Float32Array): Float32Array {
    const corrected = values.slice();
    let numerator = 0;
    let denominator = 0;

    for (let index = 0; index < corrected.length; index += 1) {
      numerator += initialField[index] - corrected[index];
      denominator += (corrected[index] * corrected[index] - 1) / 2;
    }

    if (Math.abs(denominator) < 1e-12) {
      return corrected;
    }

    const beta = numerator / denominator;
    for (let index = 0; index < corrected.length; index += 1) {
      corrected[index] = clampFieldValue(
        corrected[index] + beta * (corrected[index] * corrected[index] - 1) / 2
      );
    }

    return corrected;
  }

  function evolveOneStep() {
    const uStar = solveBackwardEuler(field, field);
    const reacted = reactionStep(uStar);
    field = conservativeCorrection(reacted);
    currentMass = fieldMass(field);
    simulatedTime += TIME_STEP;
  }

  function tickEvolution() {
    if (!simulationRunning) {
      return;
    }

    for (let step = 0; step < SUBSTEPS_PER_FRAME; step += 1) {
      evolveOneStep();
    }

    framesRendered += 1;
    simStatus = `Allen-Cahn evolution running. t = ${simulatedTime.toFixed(2)}, frame ${framesRendered}, mass = ${currentMass.toExponential(3)}.`;
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
    simStatus = `Allen-Cahn evolution running. t = 0.00, mass = ${currentMass.toExponential(3)}.`;
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
    <div>
      <p class="meta-label">Simulation Params</p>
      <p class="meta-value">N = {GRID_SIZE}, dt = {TIME_STEP}, eps = {EPSILON.toFixed(4)}</p>
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
          <span>t = {simulatedTime.toFixed(2)}</span>
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
    <button
      type="button"
      class="primary icon-button"
      onclick={runEvolution}
      aria-label="Play solver"
      title="Play solver"
    >
      <svg viewBox="0 0 24 24" aria-hidden="true">
        <path d="M8 6L18 12L8 18Z"></path>
      </svg>
    </button>
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

  .icon-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 3rem;
    padding: 0.7rem;
  }

  .icon-button svg {
    width: 1rem;
    height: 1rem;
    fill: currentColor;
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

<script lang="ts">
  let {
    titleOffset,
    titleScale
  }: {
    titleOffset: number;
    titleScale: number;
  } = $props();
</script>

<section class="landing-shell">
  <section class="hero-stage">
    <div class="hero-grid"></div>
    <div class="hero-orb hero-orb-a"></div>
    <div class="hero-orb hero-orb-b"></div>

    <div
      class="hero-title-wrap"
      style={`transform: translateY(${titleOffset}px) scale(${titleScale});`}
    >
      <p class="hero-kicker">Simulating PDEs with</p>
      <h1>QUANTUM<br />ANNEALING</h1>
      <p class="hero-subtitle">(or not)</p>
    </div>
  </section>

  <section class="story-frame">
    <article class="about-card">
      <p class="window-tag">ABOUT US</p>
      <h2>Who are we?</h2>
      <div class="contributors">
        <section class="contributor">
          <h3>Byron Selvage</h3>
          <p>BS in AMS. MS in AMS in progress...</p>
          <a
            class="profile-link"
            href="https://www.linkedin.com/in/byronselvage/"
            target="_blank"
            rel="noreferrer"
            aria-label="Byron Selvage LinkedIn"
          >
            <img src="/icons/pixel-linkedin.png" alt="" class="profile-icon" />
            <span class="profile-tooltip">LinkedIn</span>
          </a>
        </section>

        <section class="contributor">
          <h3>Landon Gehr</h3>
          <p>BS in CS and MS in Data Science. MS in AMS in progress...</p>
          <a
            class="profile-link"
            href="https://www.linkedin.com/in/landon-gehr/"
            target="_blank"
            rel="noreferrer"
            aria-label="Landon Gehr LinkedIn"
          >
            <img src="/icons/pixel-linkedin.png" alt="" class="profile-icon" />
            <span class="profile-tooltip">LinkedIn</span>
          </a>
        </section>

        <section class="contributor">
          <h3>Kyle Sperber</h3>
          <p>BS in AMS and Physics and MS in AMS. MS in Physics in progress... PhD in Nuclear Engineering at UC Berkley coming soon...</p>
          <a
            class="profile-link"
            href="https://www.linkedin.com/in/kyle-sperber-55572b240/"
            target="_blank"
            rel="noreferrer"
            aria-label="Kyle Sperber LinkedIn"
          >
            <img src="/icons/pixel-linkedin.png" alt="" class="profile-icon" />
            <span class="profile-tooltip">LinkedIn</span>
          </a>
        </section>

        <section class="contributor">
          <h3>Melody Goldanloo</h3>
          <p>BS in CS and MS in QE Software in progress...</p>
          <a
            class="profile-link"
            href="https://www.linkedin.com/in/melody-goldanloo/"
            target="_blank"
            rel="noreferrer"
            aria-label="Melody Goldanloo LinkedIn"
          >
            <img src="/icons/pixel-linkedin.png" alt="" class="profile-icon" />
            <span class="profile-tooltip">LinkedIn</span>
          </a>
        </section>
      </div>

      <a
        class="github-link"
        href="https://github.com/Landon-Gehr/BlasterHacks-Quantum-Annealing"
        target="_blank"
        rel="noreferrer"
        aria-label="Open GitHub repository"
      >
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path
            d="M12 2C6.48 2 2 6.59 2 12.25c0 4.53 2.87 8.37 6.84 9.73.5.1.68-.22.68-.49 0-.24-.01-1.04-.01-1.88-2.78.62-3.37-1.21-3.37-1.21-.46-1.2-1.11-1.52-1.11-1.52-.91-.64.07-.63.07-.63 1 .07 1.53 1.06 1.53 1.06.9 1.57 2.35 1.12 2.92.86.09-.67.35-1.12.63-1.38-2.22-.26-4.55-1.14-4.55-5.09 0-1.13.39-2.05 1.03-2.77-.1-.26-.45-1.3.1-2.71 0 0 .85-.28 2.8 1.06A9.5 9.5 0 0 1 12 7.1c.85 0 1.7.12 2.5.36 1.95-1.34 2.8-1.06 2.8-1.06.56 1.41.21 2.45.1 2.71.64.72 1.03 1.64 1.03 2.77 0 3.96-2.33 4.82-4.56 5.08.36.32.68.94.68 1.9 0 1.37-.01 2.47-.01 2.81 0 .27.18.6.69.49A10.26 10.26 0 0 0 22 12.25C22 6.59 17.52 2 12 2Z"
          />
        </svg>
        <span class="github-tooltip">GitHub repo</span>
      </a>
    </article>
  </section>
</section>

<style>
  .landing-shell {
    padding: 1.25rem;
  }

  .hero-stage {
    position: relative;
    min-height: 110vh;
    display: grid;
    place-items: center;
    overflow: hidden;
  }

  .hero-grid {
    position: absolute;
    inset: 0;
    background-image:
      linear-gradient(rgba(255, 255, 255, 0.05) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255, 255, 255, 0.05) 1px, transparent 1px);
    background-size: 56px 56px;
    opacity: 0.28;
  }

  .hero-orb {
    position: absolute;
    border-radius: 999px;
    filter: blur(20px);
    opacity: 0.7;
  }

  .hero-orb-a {
    width: 14rem;
    height: 14rem;
    background: rgba(255, 84, 162, 0.34);
    top: 15%;
    left: 8%;
  }

  .hero-orb-b {
    width: 16rem;
    height: 16rem;
    background: rgba(49, 143, 255, 0.3);
    right: 10%;
    bottom: 14%;
  }

  .hero-title-wrap {
    position: sticky;
    top: 7rem;
    z-index: 2;
    width: min(72rem, calc(100vw - 3rem));
    padding: 1.8rem;
    transform-origin: top center;
    transition: transform 0.14s linear;
  }

  .hero-kicker,
  .window-tag {
    margin: 0 0 0.6rem;
    color: #ff8fcb;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    font-size: 1.5rem;
  }

  .hero-title-wrap h1 {
    margin: 0;
    font-family: "Quagey", "Dotemp", monospace;
    font-size: clamp(4.8rem, 82vw, 11.5rem);
    line-height: 0.88;
    letter-spacing: 0.06em;
    color: #fff6a8;
    text-shadow:
      0 0 18px rgba(255, 153, 0, 0.18),
      4px 4px 0 #05050a;
  }

  .hero-subtitle {
    width: min(30rem, 100%);
    margin: 1.2rem 0 0;
    color: #f7dfff;
    font-size: 1.1rem;
    line-height: 1.5;
  }

  .story-frame {
    display: grid;
    gap: 1.15rem;
  }

  .about-card {
    border: 2px solid #2b2340;
    padding: 1.3rem;
    background:
      linear-gradient(180deg, rgba(20, 16, 28, 0.96), rgba(10, 10, 16, 0.96));
    box-shadow:
      0 0 0 2px rgba(255, 255, 255, 0.02) inset,
      8px 8px 0 #06050a;
  }

  .about-card h2 {
    margin: 0.25rem 0 0.75rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.05;
  }

  .contributors {
    display: grid;
    gap: 1rem;
    margin-top: 1rem;
  }

  .contributor {
    padding-bottom: 0.9rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .contributor:last-of-type {
    padding-bottom: 0;
    border-bottom: none;
  }

  .contributor h3 {
    margin: 0 0 0.3rem;
    font-family: "Glastone", "Dotemp", monospace;
    color: #fff6a8;
    line-height: 1.05;
    font-size: 1.1rem;
  }

  .about-card p {
    color: #efdaf9;
    line-height: 1.65;
  }

  .profile-link {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    margin-top: 0.2rem;
    color: #84c3ff;
    text-decoration: none;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    font-size: 0.82rem;
  }

  .profile-link:hover,
  .profile-link:focus-visible {
    color: #fff6a8;
  }

  .profile-icon {
    width: 22px;
    height: 22px;
    image-rendering: pixelated;
    image-rendering: crisp-edges;
  }

  .profile-tooltip {
    opacity: 0;
    transform: translateX(-4px);
    transition:
      opacity 0.18s ease,
      transform 0.18s ease;
    color: currentColor;
    font-size: 0.78rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    pointer-events: none;
  }

  .profile-link:hover .profile-tooltip,
  .profile-link:focus-visible .profile-tooltip {
    opacity: 1;
    transform: translateX(0);
  }

  .github-link {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    justify-content: center;
    align-self: center;
    margin: 1.2rem auto 0;
    padding: 0;
    border: none;
    background: transparent;
    color: #fff3ff;
    text-decoration: none;
    transition:
      transform 0.2s ease,
      color 0.2s ease;
  }

  .github-link:hover {
    transform: translateY(-1px);
    color: #fff6a8;
  }

  .github-link svg {
    width: 64px;
    height: 64px;
    fill: currentColor;
  }

  .github-tooltip {
    opacity: 0;
    transform: translateX(-4px);
    transition:
      opacity 0.18s ease,
      transform 0.18s ease;
    color: currentColor;
    font-size: 0.78rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    pointer-events: none;
  }

  .github-link:hover .github-tooltip,
  .github-link:focus-visible .github-tooltip {
    opacity: 1;
    transform: translateX(0);
  }

  @media (max-width: 980px) {
    .landing-shell {
      padding: 0.8rem;
    }

    .hero-title-wrap {
      width: calc(100vw - 2rem);
      top: 6.5rem;
      padding: 1rem;
    }
  }
</style>

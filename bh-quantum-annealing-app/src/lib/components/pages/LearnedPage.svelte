<section class="page-shell">
  <article class="retro-window full">
    <div class="window-bar pink">
      <span>Learned Solver</span>
      <span class="window-controls">_ □ ×</span>
    </div>
    <div class="window-body stacked">
      <p class="section-kicker">Learning with DDPMs</p>
      <h2>Denoising Diffusion Probabilistic Modeling</h2>
      <p>
        A DDPM is a type of generative AI model that learns a target distribution through iterative predictions of gaussian errors.
      </p>
      <div class="pixel-card alt">
        <p> About DDPMs <a href="https://arxiv.org/abs/2006.11239"> Here</a> </p>
          
          <div class="centered-images">
                  <p> DDPMs are define in terms of a Markov Chain of gaussian noise applications. The noising a.k.a. forward process which is defined by the distribution q with is used to slowly apply noise to the sample from the original distribution p according to a schedule beta. In our case the reverse process has an additional conditional probability for the know values of the forcing function and boundary conditions. The existence of these initial and boundary conditions are used in attempting to get the model to iterate back to the solution of the pde as the most likely state rather than simply image generation from pure noise.</p>

  <img src="../markov_chain.png">
          <p> By defining a process this way we can train a model to predict on the small amount of gaussian error applied at each step so that when aided by the conditional of the the current step the model can accurately remove small amounts of noise at a time. After training a model in that fashion it is possible to iterate backwards from pure noise toward a likely state from the target distribution.</p>

  <img src="../ddpm_training.png">
  <p>The ability to learn all possible PDE solution states would be impossible. Instead we looked limiting to combinations of 2D trigometric functions up to the 5th harmonic. Below is the known solution cos(x)cos(y) which is an element X_0 from our target distribution.</p>

  <img class="before-img" src="../before.png">
    <p> Below are the results of the reverse process after 25, 50, 100, and 1000 timesteps.</p> 

</div>
        <div class="diffusion-row">
          <img src="../diffused_25.png">
          <img src="../diffused_50.png">
          <img src="../diffused_100.png">
          <img src="../diffused.png">
        </div>

        <p> As we can see that the model is unable to converge on the solution alone. This is likely a result of the such a noisy target distribution that to accurately predict from many close most likely states becomes a very difficult task. However, it is still clearly shown that the DDPM model outperforms random noise and is able to speed up the annealing process with a better initial guess than random.
      </div>
    </div>
  </article>
</section>

<style>
.diffusion-row {
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: center;
    justify-content: center;
  }

  .diffusion-row img {
    height: 200px;
  }

  .centered-images {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.before-img {
  display: block;        /* required for margin auto */
  margin: 0 auto;        /* centers horizontally */
  max-width: 300px;
  height: auto;
}

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

#![recursion_limit = "256"]
use burn::{
    nn::{Linear, LinearConfig, Relu},
    optim::{AdamConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor, Adam},
    prelude::*,
    tensor::{Tensor, Distribution, backend::AutodiffBackend},
};
use plotters::prelude::*;

type MyBackend = burn::backend::Wgpu;
type MyAutodiff = burn::backend::Autodiff<MyBackend>;

pub struct DiffusionModel<B: AutodiffBackend> {
    mlp: ErrorMlp<B>,
    t: usize,
    device: B::Device,
    optim: OptimizerAdaptor<Adam, ErrorMlp<B>, B>,
    betas: Tensor<B, 1>,
    alphas: Tensor<B, 1>,
    alpha_bars: Tensor<B, 1>,
    res: (usize, usize)
}

impl<B: AutodiffBackend> DiffusionModel<B> {
    pub fn new(device: B::Device, t: usize) -> Self {
        let res: (usize,usize) = (4,4); 
        let mlp = ErrorMlp::<B>::new(&device, res);
        let optim = AdamConfig::new().with_epsilon(1e-5).init();
        let betas_vec: Vec<f32> = (0..t).map(|i| 1e-4 + (2e-2 - 1e-4) * i as f32 / (t - 1) as f32).collect();
        let betas = Tensor::<B, 1>::from_floats(betas_vec.as_slice(), &device);
        let alphas: Tensor<B, 1> = 1.0 - betas.clone();
        let alpha_bars = alphas.clone().cumprod(0);
        Self {
            mlp,
            optim,
            device,
            t,
            betas,
            alphas,
            alpha_bars,
            res
        }
    }

    pub fn noise_data(&mut self, x: Tensor<B, 3>) -> (Tensor<B, 3>, Tensor<B, 1, Int>,Tensor<B, 3>) {
        let batch_size = x.dims()[0];
        let ts = Tensor::<B, 1>::random([batch_size],Distribution::Uniform(0.0, (self.t + 1) as f64),&self.device).floor().int();
        let eps = Tensor::<B, 3>::random(x.dims(), Distribution::Normal(0.0, 1.0), &self.device);    
        let alpha_bars_t = self.alpha_bars.clone().select(0, ts.clone()); 
        let x_0_coef = alpha_bars_t.clone().sqrt().reshape([batch_size, 1, 1]);
        let epsilon_coef = (alpha_bars_t.ones_like() - alpha_bars_t).sqrt().reshape([batch_size, 1, 1]);
        let x_noised = x_0_coef * x + epsilon_coef * eps.clone();
        (x_noised, ts, eps)
    }

    pub fn unnoise_data(&self, x_t: Tensor<B, 3>) -> Tensor<B, 3> {
        let mut x = x_t.clone();
        let batch_size = x_t.dims()[0];

        for t in (0..self.t).rev() {
            let alpha_t: f32      = self.alphas.clone().slice([t..t+1]).into_scalar().elem();
            let beta_t: f32       = self.betas.clone().slice([t..t+1]).into_scalar().elem();
            let alpha_bar_t: f32  = self.alpha_bars.clone().slice([t..t+1]).into_scalar().elem();

            let mean_eps_coef = (1.0 - alpha_t.clone()) / (1.0 - alpha_bar_t.clone()).sqrt(); 
            let var_eps_coef  = 1.0 / alpha_t.clone().sqrt();           
            let dev_eps_coef  = beta_t.sqrt();                            

            let z: Tensor<B, 3> = if t > 1 {
                Tensor::random(x.dims(), Distribution::Normal(0.0, 1.0), &self.device)
            } else {
                Tensor::zeros(x.dims(), &self.device)
            };

            let t_tensor = Tensor::<B, 1,  Int>::full([batch_size], t as u32, &self.device);
            let eps_hat = self.mlp.forward(x.clone(), t_tensor);
            x = var_eps_coef * (x - mean_eps_coef * eps_hat) + dev_eps_coef * z;
        }

        x
    }

    pub fn train(&mut self, train_data: Vec<Tensor<B, 3>>, val_data: Vec<Tensor<B, 3>>, epochs: usize) {
        let mut best_val_loss = f32::INFINITY;
        let mut epoch_train_loss = f32::INFINITY;
        let mut epoch_val_loss = f32::INFINITY;

        for epoch in 0..epochs {
            println!(
                "epoch: {}/{}, (train, val) loss: {:.7}, {:.7}",
                epoch, epochs, epoch_train_loss, epoch_val_loss
            );

            // ============
            //   Training
            // ============
            let mut batch_losses: Vec<f32> = Vec::new();

            for x_0 in train_data.iter() {
                let (x_t, t_tensor, eps) = self.noise_data(x_0.clone());

                let eps_hat = self.mlp.forward(x_t, t_tensor);
                let loss = (eps_hat - eps).powf_scalar(2.0).mean();

                batch_losses.push(loss.clone().into_scalar().elem());

                let grads = loss.backward();
                let grads = GradientsParams::from_grads(grads, &self.mlp);
                self.mlp = self.optim.step(1e-5, self.mlp.clone(), grads);
            }

            epoch_train_loss = batch_losses.iter().sum::<f32>() / batch_losses.len() as f32;

            // ============
            //  Validation
            // ============
            let mut batch_losses: Vec<f32> = Vec::new();

            for x_0 in val_data.iter() {
                let (x_t, t_tensor, eps) = self.noise_data(x_0.clone());

                // no_grad equivalent: use the inner backend directly
                let eps_hat = self.mlp.forward(x_t, t_tensor);
                let loss = (eps_hat - eps).powf_scalar(2.0).mean();

                batch_losses.push(loss.into_scalar().elem());
            }

            epoch_val_loss = batch_losses.iter().sum::<f32>() / batch_losses.len() as f32;

            // ========================
            //  Save best model
            // ========================
            if epoch_val_loss < best_val_loss {
                best_val_loss = epoch_val_loss;
                // save model record to file
                let recorder = burn::record::CompactRecorder::new();
                self.mlp
                    .clone()
                    .save_file("model", &recorder)
                    .expect("Failed to save model");
                println!("saved new best model (val loss: {:.7})", best_val_loss);
            }
        }
    }

    pub fn inference(&self) -> Tensor<B, 2> {
        let x = Tensor::<B, 3>::random([1, self.res.0,self.res.1],Distribution::Normal(0.0, 1.0),&self.device);
        let sample: Tensor::<B, 3> = self.unnoise_data(x);
        sample.squeeze::<2>()
    }
}

#[derive(Module, Debug)]
pub struct ErrorMlp<B: Backend> {
    res: (usize, usize),
    linear1: Linear<B>,
    relu: Relu,
    linear2: Linear<B>,
}

impl<B: Backend> ErrorMlp<B> {
    pub fn new(device: &B::Device, res: (usize, usize)) -> Self {
        Self {
            res,
            linear1: LinearConfig::new(res.0 * res.1, 512).with_bias(true).init(device),
            relu: Relu::new(),
            linear2: LinearConfig::new(512, res.0 * res.1).with_bias(true).init(device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 3>, _t: Tensor<B, 1, Int>) -> Tensor<B, 3> {
        let [batch_size, h, w] = x.dims();
        let x = x.reshape([batch_size, h * w]);
        let x = self.linear1.forward(x);
        let x = self.relu.forward(x);
        let x = self.linear2.forward(x);
        x.reshape([batch_size, h, w])
    }
}


pub fn plot_heatmap(tensor: Tensor<MyBackend, 2>, path: &str) {
    let dims = tensor.dims();
    let h = dims[0];
    let w = dims[1];
    let vals: Vec<f32> = tensor.to_data().to_vec().unwrap();

    let root = BitMapBackend::new(path, (w as u32, h as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let cells = root.split_evenly((h, w));
    for (i, cell) in cells.into_iter().enumerate() {
        let val = vals[i].clamp(0.0, 1.0);
        let r = (val * 255.0) as u8;
        let b = ((1.0 - val) * 255.0) as u8;
        cell.fill(&RGBColor(r, 0, b)).unwrap();
    }

    root.present().unwrap();
}

fn main() {
    let res: (usize, usize) = (4,4);
    let batch_size = 32;
    let n_train = 100;
    let n_test = 20;
    let device = <MyAutodiff as Backend>::Device::default();
    let mut model = DiffusionModel::<MyAutodiff>::new(device.clone(), 100);
    let train_x: Vec<Tensor<MyAutodiff, 3>> = (0..n_train).map(|_| Tensor::<MyAutodiff, 3>::random([batch_size, res.0, res.1],Distribution::Uniform(0.0, 2.0),&device).floor()).collect();
    let test_x: Vec<Tensor<MyAutodiff, 3>> = (0..n_test).map(|_| Tensor::<MyAutodiff, 3>::random([batch_size, res.0, res.1],Distribution::Uniform(0.0, 2.0),&device).floor()).collect();
    model.train(train_x, test_x, 10);
    let out = model.inference();
    println!("{:?}", out);

    let data = out.to_data();
    let vals: Vec<f32> = data.to_vec().unwrap();

    let out_inner = out.inner();  // Tensor<MyBackend, 2>
    plot_heatmap(out_inner, "output.png");
}
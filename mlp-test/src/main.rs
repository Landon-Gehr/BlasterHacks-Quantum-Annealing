#![recursion_limit = "256"]
use burn::{
    nn::{Linear, LinearConfig, Relu, conv::{Conv2dConfig}},
    optim::{AdamConfig, GradientsParams, Optimizer, adaptor::OptimizerAdaptor, Adam},
    prelude::*,
    tensor::{Tensor, Distribution, backend::AutodiffBackend},
    record::{CompactRecorder},
    module::AutodiffModule
};
use plotters::prelude::*;
use std::{io::{BufRead, BufReader},
          fs::File
        };

type MyBackend = burn::backend::Wgpu;
type MyAutodiff = burn::backend::Autodiff<MyBackend>;

pub trait NoisePredictor<B: Backend> {
    fn forward(&self, x: Tensor<B, 3>, t: Tensor<B, 1, Int>) -> Tensor<B, 3>;
}

impl<B: Backend> NoisePredictor<B> for ErrorMlp<B> {
    fn forward(&self, x: Tensor<B, 3>, t: Tensor<B, 1, Int>) -> Tensor<B, 3> {
        self.forward(x, t)
    }
}

impl<B: Backend> NoisePredictor<B> for ErrorCnn<B> {
    fn forward(&self, x: Tensor<B, 3>, t: Tensor<B, 1, Int>) -> Tensor<B, 3> {
        self.forward(x, t)
    }
}

pub struct DiffusionModel<B: AutodiffBackend, M: AutodiffModule<B>> {
    mlp: M,
    t: usize,
    device: B::Device,
    optim: OptimizerAdaptor<Adam, M, B>,
    betas: Tensor<B, 1>,
    alphas: Tensor<B, 1>,
    alpha_bars: Tensor<B, 1>,
    res: (usize, usize)
}

impl<B: AutodiffBackend, M: AutodiffModule<B> + NoisePredictor<B>> DiffusionModel<B, M>
    where M: AutodiffModule<B>, {   
        pub fn new(device: B::Device, t: usize, res: (usize, usize), mlp: M) -> Self {
        let optim = AdamConfig::new().with_epsilon(7.5e-5).init();
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

    pub fn load(device: B::Device, t: usize, path: &str, res: (usize,usize), mlp: M) -> Self {
        let mut s = Self::new(device, t, res, mlp);
        let recorder = CompactRecorder::new();
        s.mlp = s.mlp.load_file(path, &recorder, &s.device).expect("Failed to load model");
        s
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

        // println!("reverse process:");
        for t in (0..self.t).rev() {
            // print!("{} ",t);
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

    pub fn train(&mut self, train_data: Vec<Tensor<B, 3>>, epochs: usize) {
        let mut epoch_train_loss = f32::INFINITY;
        let _t_start = std::time::Instant::now();
        for epoch in 0..epochs {
            let t_eplased = _t_start.elapsed();
            println!(
                "epoch: {}/{}, (train) loss: {:.7} in {:.2?} seconds",
                epoch, epochs, epoch_train_loss, t_eplased
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
                self.mlp = self.optim.step(7.5e-5, self.mlp.clone(), grads);
            }

            epoch_train_loss = batch_losses.iter().sum::<f32>() / batch_losses.len() as f32;

        }
        // ========================
        //  Save model
        // ========================
        self.mlp.clone().save_file("model", &CompactRecorder::new()).expect("Failed to save model");
    }

    pub fn inference(&self) -> Tensor<B, 2> {
        let x = Tensor::<B, 3>::random([1, self.res.0,self.res.1],Distribution::Normal(0.0, 1.0),&self.device);                    
        let sample: Tensor::<B, 3> = self.unnoise_data(x);
        let sample = sample.squeeze::<2>();
        sample
    }
}

#[derive(Module, Debug)]
pub struct ErrorMlp<B: Backend> {
    res: (usize, usize),
    t: usize,
    linear1: Linear<B>,
    relu1: Relu,
    linear2: Linear<B>,
    relu2: Relu,
    linear3: Linear<B>,
    relu3: Relu,
    linear4: Linear<B>,
    relu4: Relu,
    linear5: Linear<B>,
    // sigmoid: Sigmoid
}

impl<B: Backend> ErrorMlp<B> {
    pub fn new(device: &B::Device, res: (usize, usize), t: usize) -> Self {
        Self {
            res,
            t,
            linear1: LinearConfig::new(res.0 * res.1 + 1, 512).with_bias(true).init(device),
            relu1: Relu::new(),
            linear2: LinearConfig::new(512, 1024).with_bias(true).init(device),
            relu2: Relu::new(),
            linear3: LinearConfig::new(1024,1024).with_bias(true).init(device),
            relu3: Relu::new(),
            linear4: LinearConfig::new(1024, 512).with_bias(true).init(device),
            relu4: Relu::new(),
            linear5: LinearConfig::new(512, res.0 * res.1).with_bias(true).init(device),
            // relu6: Relu::new()
            // sigmoid: Sigmoid::new()
        }
    }


    pub fn forward(&self, x: Tensor<B, 3>, t: Tensor<B, 1, Int>) -> Tensor<B, 3> {
        let [batch_size, h, w] = x.dims();
        let t = t.float().reshape([batch_size, 1]) / self.t as f32;
        let x = x.reshape([batch_size, h * w]);
        let x = Tensor::cat(vec![x, t], 1);
        let x = self.linear1.forward(x);
        let x = self.relu1.forward(x);
        let x = self.linear2.forward(x);
        let x = self.relu2.forward(x);
        let x = self.linear3.forward(x);
        let x = self.relu3.forward(x);
        let x = self.linear4.forward(x);
        let x = self.relu4.forward(x);
        let x = self.linear5.forward(x);
        // let x = self.sigmoid.forward(x);
        x.reshape([batch_size, h, w])
    }
}



#[derive(Module, Debug)]
pub struct ErrorCnn<B: Backend> {
    res: (usize, usize),
    t: usize,
    t_embed: Linear<B>,  
    conv1: burn::nn::conv::Conv2d<B>,
    relu1: Relu,
    conv2: burn::nn::conv::Conv2d<B>,
    relu2: Relu,
    conv3: burn::nn::conv::Conv2d<B>,
    relu3: Relu,
    conv4: burn::nn::conv::Conv2d<B>,
}

impl<B: Backend> ErrorCnn<B> {
    pub fn new(device: &B::Device, res: (usize, usize), t: usize) -> Self {
        Self {
            res,
            t,
            t_embed: LinearConfig::new(1, 1).with_bias(true).init(device),
            conv1: Conv2dConfig::new([1, 16], [3, 3]).with_padding(burn::nn::PaddingConfig2d::Same).init(device),
            relu1: Relu::new(),
            conv2: Conv2dConfig::new([16, 16], [3, 3]).with_padding(burn::nn::PaddingConfig2d::Same).init(device),
            relu2: Relu::new(),
            conv3: Conv2dConfig::new([16, 16], [3, 3]).with_padding(burn::nn::PaddingConfig2d::Same).init(device),
            relu3: Relu::new(),
            conv4: Conv2dConfig::new([16, 1], [3, 3]).with_padding(burn::nn::PaddingConfig2d::Same).init(device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 3>, t: Tensor<B, 1, Int>) -> Tensor<B, 3> {
        let [batch_size, h, w] = x.dims();

        let t_norm = t.float().reshape([batch_size, 1]) / self.t as f32;
        let t_emb = self.t_embed.forward(t_norm).reshape([batch_size, 1, 1, 1]);

        let x = x.reshape([batch_size, 1, h, w]);
        let x = x + t_emb;
        let x = self.conv1.forward(x);
        let x = self.relu1.forward(x);
        let x = self.conv2.forward(x);
        let x = self.relu2.forward(x);
        let x = self.conv3.forward(x);
        let x = self.relu3.forward(x);
        let x = self.conv4.forward(x);

        x.reshape([batch_size, h, w])
    }
}



pub fn plot_heatmap(tensor: Tensor<MyBackend, 2>, path: &str) {
    let dims = tensor.dims();
    let h = dims[0];
    let w = dims[1];
    let vals: Vec<f32> = tensor.to_data().to_vec::<f32>().unwrap();

    let min = vals.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let root = BitMapBackend::new(path, (900, 800)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // split canvas: left for heatmap, right for colorbar
    let (main_area, colorbar_area) = root.split_horizontally(800);

    let mut chart = ChartBuilder::on(&main_area)
        .margin(10)
        .caption("Diffusion Model Output", ("sans-serif", 20))
        .build_cartesian_2d(0..w, 0..h)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for j in 0..h {
        for i in 0..w {
            let val = vals[j * w + i];
            let t = if max > min { ((val - min) / (max - min) *255.0) as u8} else { 128 };
            let color = RGBColor(t,t,t);
            chart.draw_series(std::iter::once(Rectangle::new(
                [(i, j), (i + 1, j + 1)],
                color.filled(),
            ))).unwrap();
        }
    }

    // draw colorbar
    let mut colorbar = ChartBuilder::on(&colorbar_area)
        .margin(10)
        .build_cartesian_2d(0..1, 0..100)
        .unwrap();

    for step in 0..100 {
        let t = (step as f64 / 100.0 * 255.0) as u8;
        let color = RGBColor(t,t,t);
        colorbar.draw_series(std::iter::once(Rectangle::new(
            [(0, step), (1, step + 1)],
            color.filled(),
        ))).unwrap();
    }

    // label min and max on colorbar
    colorbar_area.draw(&Text::new(
        format!("{:.2}", max),
        (5, 10),
        ("sans-serif", 12).into_font(),
    )).unwrap();
    colorbar_area.draw(&Text::new(
        format!("{:.2}", min),
        (5, 780),
        ("sans-serif", 12).into_font(),
    )).unwrap();

    root.present().unwrap();
}


fn load_data(filename: &str, batch_size: usize, h: usize, w: usize, device: &<MyAutodiff as Backend>::Device) -> Vec<Tensor<MyAutodiff, 3>> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let vals: Vec<f32> = reader.lines().map(|l: Result<String, _>| l.unwrap().trim().parse::<f32>().unwrap()).collect();

    let sample_size = h * w;                     
    let batch_flat = batch_size * sample_size; 

    vals.chunks(batch_flat)
    .filter(|chunk: &&[f32]| chunk.len() == batch_flat)
    .map(|chunk: &[f32]| {
        Tensor::<MyAutodiff, 3>::from_floats(
            burn::tensor::TensorData::new(chunk.to_vec(), [batch_size, h, w]),
            device,
        )
    })
    .collect()
}


fn main() {
    let res: (usize, usize) = (32,32);
    let batch_size = 256;
    let n_timesteps = 2500;
    let device = <MyAutodiff as Backend>::Device::default();
    let mlp = ErrorCnn::<MyAutodiff>::new(&device, res, n_timesteps);
    // let mlp = ErrorMlp::<MyAutodiff>::new(&device, res, t);
    let mut model = DiffusionModel::new(device.clone(), n_timesteps, res, mlp);
    let train_x = load_data("training.dat", batch_size, res.0, res.1, &device);
    model.train(train_x, 250);


    // let model = DiffusionModel::load(device.clone(), n_timesteps, "model", res, mlp);
    
    let file = File::open("test.dat").expect("Failed to open file");
    let reader = BufReader::new(file);

    let vals: Vec<f32> = reader
        .lines()
        .take(res.0 * res.1)
        .map(|l: Result<String, _>| l.unwrap().trim().parse::<f32>().unwrap())
        .collect();
    let x = Tensor::<MyAutodiff, 3>::from_floats(
        burn::tensor::TensorData::new(vals, [1, res.0, res.1]),
        &device,
    );

    println!("training finished, plotting");
    let before = x.clone().squeeze::<2>().inner();
    plot_heatmap(before, "before.png");

    let noised = x.clone()  * 0.85;
    let noised = noised.squeeze::<2>().inner();
    plot_heatmap(noised, "noised.png");

    let sample: Tensor<MyAutodiff, 3> = model.unnoise_data(x);
    let sample = sample.squeeze::<2>();

    let sample_inner = sample.inner(); 
    plot_heatmap(sample_inner, &format!("diffused_{}.png",n_timesteps));
}
#![recursion_limit = "256"]
use burn::{
    module::{Module,AutodiffModule},
    nn::{Linear, LinearConfig, Relu},
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    tensor::Tensor,
    // backend::wgpu::WgpuDevice,
};
use rand::Rng;

// type MyBackend = burn::backend::NdArray;
type MyBackend = burn::backend::Wgpu;
type MyAutodiff = burn::backend::Autodiff<MyBackend>;

#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    linear1: Linear<B>,
    relu: Relu,
    linear2: Linear<B>,
}

impl<B: Backend> Mlp<B> {
    pub fn new(device: &B::Device) -> Self {
        Self {
            linear1: LinearConfig::new(2, 512).with_bias(true).init(device),
            relu: Relu::new(),
            linear2: LinearConfig::new(512, 1).with_bias(true).init(device),
        }
    }

    pub fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(x);
        let x = self.relu.forward(x);
        self.linear2.forward(x)
    }
}


// #[allow(unreachable_code)]
// fn select_device() -> DispatchDevice {
//     #[cfg(feature = "ndarray")]
//     return NdArrayDevice::Cpu.into();

//     #[cfg(all(feature = "tch-gpu", not(target_os = "macos")))]
//     return LibTorchDevice::Cuda(0).into();

//     #[cfg(all(feature = "tch-gpu", target_os = "macos"))]
//     return LibTorchDevice::Mps.into();

//     #[cfg(feature = "tch-cpu")]
//     return LibTorchDevice::Cpu;

//     #[cfg(any(feature = "wgpu", feature = "metal", feature = "vulkan"))]
//     return WgpuDevice::default().into();

//     #[cfg(feature = "cuda")]
//     return CudaDevice::default().into();

//     #[cfg(feature = "rocm")]
//     return RocmDevice::default().into();

//     unreachable!("At least one backend will be selected.")
// }


fn main() {
    let device = <MyBackend as Backend>::Device::default();

    let mut rng = rand::thread_rng();

    let input_data: Vec<f32> = (0..1024 * 2)
        .map(|_| rng.gen_range(0..2) as f32)
        .collect();

    let label_data: Vec<f32> = input_data
        .chunks(2)
        .map(|pair| if pair[0] != pair[1] { 1.0 } else { 0.0 })
        .collect();

    let x = Tensor::<MyAutodiff, 2>::from_floats(
        TensorData::new(input_data.clone(), [1024, 2]),
        &device,
    );

    let y = Tensor::<MyAutodiff, 2>::from_floats(
        TensorData::new(label_data, [1024, 1]),
        &device,
    );

    let mut model = Mlp::<MyAutodiff>::new(&device);
    let mut optim = AdamConfig::new().with_epsilon(1e-5).init::<MyAutodiff, Mlp<MyAutodiff>>();

    let mut avgt = std::time::Duration::from_secs(0);

    let loops=5;
    for _t in 0..loops {

    let start = std::time::Instant::now();

    for epoch in 0..3000 {
        let pred = model.forward(x.clone());
        let loss = (pred.clone() - y.clone()).powf_scalar(2.0).mean();

        if epoch % 100 == 0 {
            print!("\nEpoch {epoch:4}: loss = {:.6}", loss.clone().into_scalar());
        }

        let grads = loss.backward();
        let grads = GradientsParams::from_grads(grads, &model);
        model = optim.step(0.05, model, grads);
    }

    let te = start.elapsed();
    print!("\nTraining took: {:.2?}", te);
    avgt += te;
    }

    avgt /= loops;
    println!("\nAVG Training Time: {:.2?}",avgt);


    let model_infer = model.valid();
    let test_inputs: [[f32; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    let x_infer = Tensor::<MyBackend, 2>::from_floats(test_inputs, &device);
    let preds = model_infer.forward(x_infer);

    println!("\nPredictions after training:");
    println!("[0,0] → {:.3}", preds.clone().slice([0..1]).into_data().to_vec::<f32>().unwrap()[0]);
    println!("[0,1] → {:.3}", preds.clone().slice([1..2]).into_data().to_vec::<f32>().unwrap()[0]);
    println!("[1,0] → {:.3}", preds.clone().slice([2..3]).into_data().to_vec::<f32>().unwrap()[0]);
    println!("[1,1] → {:.3}", preds.slice([3..4]).into_data().to_vec::<f32>().unwrap()[0]);
}
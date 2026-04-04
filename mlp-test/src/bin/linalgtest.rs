use ndarray::prelude::*;
use ndarray_linalg::*;
// use rand::Rng;
use rand::SeedableRng;

fn main() {
// Use fixed algorithm and seed of PRNG for reproducible test
// let mut rng = rand::thread_rng();
let mut rng = rand::rngs::StdRng::seed_from_u64(1234);


let a: Array2<f64> = random_using((3, 3), &mut rng);
let f = a.factorize_into().unwrap(); // LU factorize A (A is consumed)
for _ in 0..10 {
    let b: Array1<f64> = random_using(3, &mut  rng);
    let x = f.solve_into(b).unwrap(); // Solve A * x = b using factorized L, U
    println!("{}",x);
}
}
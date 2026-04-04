#![allow(non_snake_case)]
use qubo::{
    annealing::solve_ising_model,
    dense::{dense_to_csmat, dense_to_csvec},
    ising::{qubo_to_ising, sparse_ising_to_quantrs},
    qubo::{compute_qubo, decode_bits_to_reals, EncodingParams},
};
use sprs::{CsMat, CsVec};

fn example_system() -> (Vec<Vec<f64>>, Vec<f64>, CsMat<f64>, CsVec<f64>) {
    let dense_A = vec![
        vec![1.0, 3.0, 0.0, 2.0, 0.0],
        vec![0.0, 0.0, 4.0, 6.0, 5.0],
        vec![3.0, 1.0, 3.0, 0.0, 2.0],
        vec![1.0, 0.0, 0.0, 0.0, 3.0],
        vec![2.0, 0.0, 5.0, 1.0, 0.0],
    ];
    let dense_b = vec![15.0, 61.0, 24.0, 16.0, 21.0];
    let A = dense_to_csmat(&dense_A);
    let b = dense_to_csvec(&dense_b);

    (dense_A, dense_b, A, b)
}

fn solve_for_x(A: &CsMat<f64>, b: &CsVec<f64>, encoding: &EncodingParams) -> Vec<f64> {
    let qubo = compute_qubo(A, b, encoding);
    let (interactions, linear, offset) = qubo_to_ising(&qubo);
    let model = sparse_ising_to_quantrs(&interactions, &linear);
    let annealing_result = solve_ising_model(&model, offset);

    decode_bits_to_reals(&annealing_result.decoded_bits, encoding)
}

fn main() {
    let encoding = EncodingParams {
        u_min: 0.0,
        u_max: 10.0,
        k_bits: 12,
    };
    let (dense_A, dense_b, A, b) = example_system();
    let x = solve_for_x(&A, &b, &encoding);

    println!("Dense-style A = {:?}", dense_A);
    println!("Dense-style b = {:?}", dense_b);
    println!("Encoding = [u_min={}, u_max={}, k_bits={}]", encoding.u_min, encoding.u_max, encoding.k_bits);
    println!("Recovered x in R = {:?}", x);
}

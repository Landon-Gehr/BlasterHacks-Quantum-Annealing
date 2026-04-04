use quantrs2_anneal::{
    ising::IsingModel,
    simulator::{
        AnnealingParams, AnnealingSolution, QuantumAnnealingSimulator, TemperatureSchedule,
        TransverseFieldSchedule,
    },
};

/// Struct to contain result of the optimization
pub struct AnnealingResult {
    pub solution: AnnealingSolution,
    pub decoded_bits: Vec<u8>,
    pub offset: f64,
}

/// Default optimization params
/// Literally no idea what these should be
fn default_annealing_params() -> AnnealingParams {
    let mut params = AnnealingParams::new();
    params.initial_transverse_field = 5.0;
    params.transverse_field_schedule = TransverseFieldSchedule::Linear;
    params.initial_temperature = 10.0;
    params.final_temperature = 0.01;
    params.temperature_schedule = TemperatureSchedule::Linear;
    params.num_sweeps = 500;
    params.updates_per_sweep = None;
    params.num_repetitions = 16;
    params.seed = Some(7);
    params.timeout = None;
    params.trotter_slices = 8;
    params
}

/// Decodes a slice of Ising spins (`-1` or `1`) 
/// into binary bits (`0` or `1`).
pub fn decode_spins_to_bits(spins: &[i8]) -> Vec<u8> {
    spins.iter().map(|&spin| u8::from(spin > 0)).collect()
}

/// Solve an Ising model using quantum annealing.
///
/// # Arguments
/// * `model` - The Ising model to be solved.
/// * `offset` - The energy offset to be added to the solution.
///
/// # Returns
/// An `AnnealingResult` containing the best solution found, the decoded bits,
/// and the energy offset.
pub fn solve_ising_model(model: &IsingModel, offset: f64) -> AnnealingResult {
    let simulator =
        QuantumAnnealingSimulator::new(default_annealing_params()).expect("valid annealing parameters");
    let solution = simulator
        .solve(model)
        .expect("annealing should solve the Ising instance");

    AnnealingResult {
        decoded_bits: decode_spins_to_bits(&solution.best_spins),
        solution,
        offset,
    }
}

#[cfg(test)]
mod tests {
    use super::solve_ising_model;
    use crate::{
        ising::{qubo_to_ising, sparse_ising_to_quantrs},
        qubo::{compute_qubo, decode_bits_to_reals, EncodingParams},
    };
    use sprs::{CsMat, CsVec};

    /// Builds a linear system whose solution
    /// should be a vector of all ones.
    fn build_linear_system() -> (CsMat<f64>, CsVec<f64>, EncodingParams) {
        let a = CsMat::new_csc(
            (4, 4),
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3],
            vec![1.0, 2.0, 3.0, 4.0],
        );
        let b = CsVec::new(4, vec![0, 1, 2, 3], vec![1.0, 2.0, 3.0, 4.0]);
        let encoding = EncodingParams {
            u_min: 0.0,
            u_max: 1.0,
            k_bits: 1,
        };

        (a, b, encoding)
    }

    /// Tests that quantum annealing recovers the solution
    /// for a diagonal linear system.
    #[test]
    fn basic_quantum_annealing() {
        let (a, b, encoding) = build_linear_system();
        let qubo = compute_qubo(&a, &b, &encoding);
        let (interactions, fields, offset) = qubo_to_ising(&qubo);
        let model = sparse_ising_to_quantrs(&interactions, &fields);
        let result = solve_ising_model(&model, offset);
        let x = decode_bits_to_reals(&result.decoded_bits, &encoding);
        println!("Recovered bits: {:?}", result.decoded_bits);
        println!("Recovered x in R: {:?}", x);
        println!("Best spins: {:?}", result.solution.best_spins);
        println!("Best energy: {}", result.solution.best_energy);
        println!("Ising offset: {}", result.offset);
        println!("Annealing info: {}", result.solution.info);

        assert_eq!(x, vec![1.0, 1.0, 1.0, 1.0]);
    }
}
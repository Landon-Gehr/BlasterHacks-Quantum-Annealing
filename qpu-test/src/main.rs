use quantrs2_anneal::{
    ising::IsingModel,
    simulator::{ClassicalAnnealingSimulator, AnnealingParams}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and solve a simple Max-Cut problem
    let mut model = IsingModel::new(4);
    model.set_coupling(0, 1, -1.0)?;
    model.set_coupling(1, 2, -1.0)?;
    model.set_coupling(2, 3, -1.0)?;
    model.set_coupling(3, 0, -1.0)?;

    let simulator = ClassicalAnnealingSimulator::new(AnnealingParams::default())?;
    let result = simulator.solve(&model)?;

    println!("Best energy: {}", result.best_energy);
    println!("Solution: {:?}", result.best_spins);

    Ok(())
}
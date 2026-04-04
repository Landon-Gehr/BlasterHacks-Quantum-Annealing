#![allow(non_snake_case)]
use quantrs2_anneal::ising::IsingModel;
use sprs::{CsMat, CsVec, TriMat};

/// Converts a QUBO matrix to a sparse Ising model representation.
/// 
/// Implements the following conversion from QUBO to Ising model.
/// - `h[i] = 1/2 Q_{ii} + 1/2 \sum_{j \ne i} Q_{ij}`
/// - `I[i, j] = 1/2 Q_{ij}`
/// - `offset = 1/2 \sum_i Q_{ii} + 1/2 \sum_{i < j} Q_{ij}`
/// 
///
/// # Arguments
/// * `Q` - Sparse CSR matrix representing the QUBO coefficients.
///
/// # Returns
/// A tuple `(I, h, offset)` where:
/// * `I` - Sparse CSR matrix representing the Ising interaction terms.
/// * `h` - Sparse vector representing the Ising local fields.
/// * `offset` - Constant energy offset.
pub fn qubo_to_ising(Q: &CsMat<f64>) -> (CsMat<f64>, CsVec<f64>, f64) {
    let n = Q.rows();
    let mut I = TriMat::new((n, n));

    // build h in dense form
    let mut h_dense = vec![0.0; n];

    let mut offset = 0.0;

    for (value, (i, j)) in Q.iter() {
        if i == j {
            // Diagonal term
            h_dense[i] += 0.5 * value;
            offset += 0.5 * value;
        } else {
            // Off-diagonal term
            let contrib = 0.5 * value;

            // Interaction term
            I.add_triplet(i, j, contrib);

            // Local fields (both i and j get contribution)
            h_dense[i] += contrib;
            h_dense[j] += contrib;

            // Offset only counts upper triangle (i < j)
            if i < j {
                offset += contrib;
            }
        }
    }

    // Convert dense h to sparse
    let (indices, data): (Vec<_>, Vec<_>) = h_dense
        .iter()
        .enumerate()
            .filter(|&(_, &v)| v != 0.0)
        .map(|(i, &v)| (i, v))
        .unzip();

    let h = CsVec::new(n, indices, data);

    (I.to_csr(), h, offset)
}

/// Converts a sparse Ising model representation 
/// to a `IsingModel` from `quantrs2_anneal`.
/// 
/// # Arguments
/// * `interactions` - Sparse CSR matrix representing the Ising interaction terms.
/// * `linear` - Sparse vector representing the Ising local fields.
///
/// # Returns
/// An `IsingModel` from `quantrs2_anneal`.
pub fn sparse_ising_to_quantrs(interactions: &CsMat<f64>, linear: &CsVec<f64>) -> IsingModel {
    let mut model = IsingModel::new(interactions.rows());

    for (index, value) in linear.iter() {
        model
            .set_bias(index, *value)
            .expect("valid Ising bias index");
    }

    for (value, (row, col)) in interactions.iter() {
        if row < col && *value != 0.0 {
            model
                .set_coupling(row, col, *value)
                .expect("valid Ising coupling indices");
        }
    }

    model
}

#[cfg(test)]
mod tests {
    use super::qubo_to_ising;
    use sprs::CsMat;

    #[test]
    fn check_qubo_to_ising() {
        let qubo = CsMat::new_csc((2, 2), vec![0, 1, 2], vec![0, 1], vec![2.0, 4.0]);

        let (interactions, fields, offset) = qubo_to_ising(&qubo);

        assert_eq!(interactions.nnz(), 0);
        assert_eq!(fields.get(0), Some(&1.0));
        assert_eq!(fields.get(1), Some(&2.0));
        assert_eq!(offset, 3.0);
    }
}

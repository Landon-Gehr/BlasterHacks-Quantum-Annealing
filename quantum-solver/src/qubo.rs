#![allow(non_snake_case)]
use sprs::{CsMat, CsVec, TriMat};

pub struct EncodingParams {
    pub u_min: f64,
    pub u_max: f64,
    pub k_bits: usize,
}

pub fn encoding_weights(encoding: &EncodingParams) -> Vec<f64> {
    assert!(encoding.k_bits > 0, "k_bits must be positive");
    assert!(encoding.u_max >= encoding.u_min, "u_max must be at least u_min");

    let max_encoded_value = (1usize << encoding.k_bits) - 1;
    let scale = if max_encoded_value == 0 {
        0.0
    } else {
        (encoding.u_max - encoding.u_min) / max_encoded_value as f64
    };

    (0..encoding.k_bits)
        .map(|bit| scale * (1usize << bit) as f64)
        .collect()
}

pub fn decode_bits_to_reals(bits: &[u8], encoding: &EncodingParams) -> Vec<f64> {
    assert!(encoding.k_bits > 0, "k_bits must be positive");
    assert_eq!(
        bits.len() % encoding.k_bits,
        0,
        "bit vector length must be divisible by k_bits"
    );

    let weights = encoding_weights(encoding);

    bits.chunks_exact(encoding.k_bits)
        .map(|chunk| {
            encoding.u_min
                + chunk
                    .iter()
                    .zip(weights.iter())
                    .map(|(&bit, &weight)| f64::from(bit) * weight)
                    .sum::<f64>()
        })
        .collect()
}

/// Builds A sparse QUBO matrix for the linear system $Au = b$.
///
/// Only creates the upper-triangular part of the QUBO matrix since it is symmetric.
///
/// # Arguments
/// * `A` - Square matrix representing the linear system.
/// * `b` - Right-hand side vector.
/// * `encoding` - Parameters for encoding continuous variables into binary variables.
///
/// # Returns
/// A sparse CSR matrix representing the QUBO coefficients.
pub fn compute_qubo(
    A: &CsMat<f64>,
    b: &CsVec<f64>,
    encoding: &EncodingParams,
) -> CsMat<f64> {
    // Check dims
    assert_eq!(A.rows(), A.cols(), "A must be square");
    assert_eq!(A.rows(), b.dim(), "b must have length equal to A.rows()");

    let K = encoding.k_bits;
    let N = A.rows();

    // Initialize QUBO matrix as empty sparse matrix
    let mut Q = TriMat::new((N * K, N * K));

    // M = A^T * A
    let at = A.transpose_view().to_csr();
    let M: CsMat<f64> = (&at * A).to_owned();

    // C = A^T * b
    let C = &at * b;

    let weights = encoding_weights(encoding);

    // Precompute row sums: sum_j M_{ij}
    let mut row_sums: Vec<f64> = vec![0.0; M.rows()];
    for (value, (i, _j)) in M.iter() {
        row_sums[i] += value;
    }

    // For each non-zero in M, add corresponding terms to Q
    for (value, (i, j)) in M.iter() {
        for k in 0..K {
            for l in 0..K {
                if i == j && k == l {
                    let q_value = weights[k] * weights[l] * value + weights[k] * (2.0 * encoding.u_min * row_sums[i] - 2.0 * C[i]);
                    Q.add_triplet(i * K + k, j * K + l, q_value);
                } else if (i == j && k < l) || (i < j) {
                    let q_value = weights[k] * weights[l] * value;
                    Q.add_triplet(i * K + k, j * K + l, q_value);
                }
            }
        }
    }

    Q.to_csr()
}

#[cfg(test)]
mod tests {
    use super::{compute_qubo, decode_bits_to_reals, EncodingParams};
    use sprs::{CsMat, CsVec};

    #[test]
    fn validate_qubo_shape() {
        let a = CsMat::new_csc(
            (2, 2),
            vec![0, 2, 4],
            vec![0, 1, 0, 1],
            vec![1.0, 2.0, 2.0, 3.0],
        );
        let b = CsVec::new(2, vec![0, 1], vec![1.0, 2.0]);
        let encoding = EncodingParams {
            u_min: 0.0,
            u_max: 1.0,
            k_bits: 2,
        };

        let qubo = compute_qubo(&a, &b, &encoding);

        assert_eq!(qubo.shape(), (2 * encoding.k_bits, 2 * encoding.k_bits));
    }

    #[test]
    fn decode_bits_maps_back_to_real_values() {
        let encoding = EncodingParams {
            u_min: 0.0,
            u_max: 10.0,
            k_bits: 5,
        };

        let x = decode_bits_to_reals(&[1, 0, 0, 1, 1, 1, 1, 1, 1, 1], &encoding);

        assert!((x[0] - 8.064516129032258).abs() < 1e-9);
        assert!((x[1] - 10.0).abs() < 1e-9);
    }
}

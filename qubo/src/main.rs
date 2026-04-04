use sprs::{CsMat, TriMat, CsVec};

pub struct EncodingParams {
    pub u_min: f64,
    pub u_max: f64,
    pub k_bits: usize,
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

    // Precompute weights: w_k = 2^k
    let mut weights: Vec<f64> = Vec::with_capacity(K);
    for k in 0..K {
        weights.push(2f64.powi(k as i32));
    }

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

#[test]
fn test_compute_qubo() {
    use sprs::{CsMat, CsVec};
    let A = CsMat::new_csc(
        (2, 2),
        vec![0, 2, 4],
        vec![0, 1, 0, 1],
        vec![1.0, 2.0, 2.0, 3.0]
    );
    let b = CsVec::new(2, vec![0, 1], vec![1.0, 2.0]);
    let encoding = EncodingParams { u_min: 0.0, u_max: 1.0, k_bits: 2 };
    let Q = compute_qubo(&A, &b, &encoding);
    assert_eq!(Q.rows(), 4); // 2 rows * 2 bits
}
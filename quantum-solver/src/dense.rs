use sprs::{CsMat, CsVec, TriMat};

pub fn dense_to_csmat(dense: &[Vec<f64>]) -> CsMat<f64> {
    assert!(!dense.is_empty(), "dense matrix must not be empty");

    let row_count = dense.len();
    let col_count = dense[0].len();
    assert!(col_count > 0, "dense matrix must have at least one column");

    let mut matrix = TriMat::new((row_count, col_count));

    for (row_index, row) in dense.iter().enumerate() {
        assert_eq!(
            row.len(),
            col_count,
            "dense matrix rows must all have the same length"
        );

        for (col_index, &value) in row.iter().enumerate() {
            if value != 0.0 {
                matrix.add_triplet(row_index, col_index, value);
            }
        }
    }

    matrix.to_csc()
}

pub fn dense_to_csvec(dense: &[f64]) -> CsVec<f64> {
    let (indices, data): (Vec<_>, Vec<_>) = dense
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value != 0.0)
        .map(|(index, &value)| (index, value))
        .unzip();

    CsVec::new(dense.len(), indices, data)
}

#[cfg(test)]
mod tests {
    use super::{dense_to_csmat, dense_to_csvec};

    #[test]
    fn dense_conversions_skip_zero_entries() {
        let matrix = dense_to_csmat(&[vec![1.0, 0.0], vec![0.0, 2.0]]);
        let vector = dense_to_csvec(&[0.0, 3.0, 0.0, 4.0]);

        assert_eq!(matrix.shape(), (2, 2));
        assert_eq!(matrix.nnz(), 2);
        assert_eq!(vector.dim(), 4);
        assert_eq!(vector.nnz(), 2);
    }
}
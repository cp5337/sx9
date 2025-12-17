#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use sx9_glaf_core::matroid::{calculate_rank, LatentMatroid};

    #[test]
    fn test_rank_calculation() {
        // v1 = (1, 0, 0)
        // v2 = (0, 1, 0)
        // v3 = (1, 1, 0) -> Dependent on v1, v2
        let vectors = vec![
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 1.0, 0.0),
        ];

        let matroid = LatentMatroid::new(vectors);
        // We test indices [0, 1, 2]
        let rank = matroid.calculate_rank(&[0, 1, 2]);
        assert_eq!(rank, 2, "Rank of 3 vectors (1 dependent) should be 2");
    }

    #[test]
    fn test_independence() {
        let vectors = vec![
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ];
        let matroid = LatentMatroid::new(vectors);
        let rank = matroid.calculate_rank(&[0, 1, 2]);
        assert_eq!(rank, 3, "Rank of 3 independent vectors should be 3");
    }
}

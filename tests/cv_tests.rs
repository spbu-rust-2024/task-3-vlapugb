#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use arrayp::modules::calculations::coefficient_variations;
    use rstest::rstest;
    #[rstest]
    #[case(&[5.0], 0.0)]
    #[case(&[3.0, 3.0, 3.0, 3.0], 0.0)]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], 0.471405)]
    #[case(&[-3.0, -2.0, 0.4, 2.0, 100.0], 2.068707)]
    fn test_cv_1(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = coefficient_variations(&test_vec);
        assert_eq!((result * 1e4).round() / 1e4, (expected * 1e4).round() / 1e4)
    }
    #[test]
    #[should_panic]
    fn test_cv_null_arifmetic_mean() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = coefficient_variations(&test_vec);
    }
}

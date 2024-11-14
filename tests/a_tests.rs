#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::calculations::arifmetic_mean;
    use rstest::rstest;
    #[rstest]
    #[case(&[1.0, 2.0, 3.0], 2.0)]
    #[case(&[1.0, 2.0, 3.0, 99.5], 26.375)]
    fn test_arifmetic_mean_all_numbers_positive(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = arifmetic_mean(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        );
    }
    #[test]
    fn test_arifmetic_mean_with_negative_numbers() {
        let test_vec: Vec<f64> = Vec::from([1.0, -2.0, 3.0, -4.0, 5.0]);
        let result = arifmetic_mean(&test_vec);
        assert_eq!(result, 0.6);
    }
    #[test]
    #[should_panic]
    fn test_arifmetic_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = arifmetic_mean(&test_vec);
    }
}

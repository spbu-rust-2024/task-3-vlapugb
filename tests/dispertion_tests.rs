#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::calculations::dispertion;

    use rstest::rstest;
    #[rstest]
    #[case(&[2.0, 2.0, 2.0, 2.0], 0.0)]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], 1.58114)]
    #[case(&[-1.0, 0.0, 1.0], 1.0)]
    fn test_dispertion_all_numbers_positive(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = dispertion(&test_vec);
        assert_eq!((result * 1e5).round() / 1e5, (expected * 1e5).round() / 1e5);
    }
    #[test]
    #[should_panic]
    fn test_dispertion_null_elements() {
        let test_vec: Vec<f64> = Vec::from([1.0]);
        let result = dispertion(&test_vec);
        assert_eq!(result, 0.0);
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::calculations::mode;
    use rstest::rstest;
    #[rstest]
    #[case(&[1.0, 2.0, 2.0, 3.0, 4.0], 2.0)]
    #[case(&[1.0, 1.0, 2.0, 3.0], 1.0)]
    #[case(&[3.0, 3.0, 3.0], 3.0)]
    fn test_mode(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = mode(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        );
    }

    #[rstest]
    #[case(&[5.0, 1.0, 2.0, 3.0, 4.0], &[5.0,1.0,2.0,3.0,4.0])]
    #[case(&[1.0, 2.0, 3.0, 4.0], &[1.0,2.0,3.0,4.0])]
    fn test_mode_all_numbers_different(#[case] test_vec: &[f64], #[case] expected: &[f64]) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let exp: Vec<f64> = Vec::from(expected);
        let result = mode(&test_vec);
        assert!(expected.contains(&result));
    }

    #[test]
    #[should_panic]
    fn test_mode_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = mode(&test_vec);
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use arrayp::modules::linear_deviation::linear_deviation;
    use rstest::rstest;
    #[rstest]
    #[case(&[1.0,2.0,3.0,4.0,5.0], 1.2)]
    #[case(&[-1.0,-2.0,-3.0,-4.0,-5.0], 1.2)]
    #[case(&[1.0, 1.0, 1.0, 1.0, 1.0], 0.0)]
    fn test_linear_deviation_1(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = linear_deviation(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        )
    }
    #[test]
    #[should_panic]
    fn test_linear_deviation_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = linear_deviation(&test_vec);
    }
}

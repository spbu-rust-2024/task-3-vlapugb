#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::calculations::power_mean;
    use rstest::rstest;
    #[rstest]
    #[case(&[2.0, 2.0, 2.0, 2.0, 2.0], 2.0, 2)]
    #[case(&[1.0, 1.0, 1.0, 1.0, 1.0], 1.0, 3)]
    #[case(&[3.0, 3.0, 3.0], 3.0, 1)]
    fn test_power_mean(#[case] test_vec: &[f64], #[case] expected: f64, #[case] power: u16) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = power_mean(&test_vec, &power);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        );
    }

    #[test]
    #[should_panic]
    fn test_power_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = power_mean(&test_vec, &10);
    }
}

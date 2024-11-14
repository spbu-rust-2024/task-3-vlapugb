#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use arrayp::modules::standart_deviation::standart_deviation;
    use rstest::rstest;
    #[rstest]
    #[case(&[1000.0, 1.0, 500.0, 10000.0, 50.0], 3861.797011755)]
    #[case(&[1000000.0, 1000001.0, 1000002.0, 1000003.0, 1000004.0], 1.4142135624)]
    #[case(&[1.0, 1.0, 1.0, 1000.0, 1000.0], 489.40805060808)]
    #[case(&[-0.001, 0.0, 0.001, 0.002, -0.002], 0.0014142135623731)]
    fn test_standart_deviation_1(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = standart_deviation(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        )
    }
}

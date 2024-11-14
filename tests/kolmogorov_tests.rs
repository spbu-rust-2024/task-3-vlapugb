#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use std::result;

    use arrayp::modules::calculations::kolmogorov_smirnov;
    #[test]
    fn kolmogorov_smirnov_test_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let eps = 0.0001;
        let result = kolmogorov_smirnov(&test_vec, eps);
        let expected = 3.7020168;
        assert!((result - expected).abs() < eps);
    }
}

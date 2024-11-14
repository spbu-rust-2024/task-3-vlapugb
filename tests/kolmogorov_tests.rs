#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::calculations::kolmogorov_smirnov;

    #[test]
    fn test_kolmogorov_smirnov_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let eps = 0.0001;
        let result = kolmogorov_smirnov(&test_vec, eps);
        let expected = 3.7020168;
        assert!((result - expected).abs() < eps);
    }
    #[test]
    #[should_panic]
    fn kolmogorov_smirnov_null_elemenents() {
        let eps = 0.0001;
        let test_vec: Vec<f64> = Vec::from([]);
        let result = kolmogorov_smirnov(&test_vec, eps);
    }
    #[test]
    #[should_panic]
    fn kolmogorov_smirnov_invalid_epsilon() {
        let eps = 1.1;
        let test_vec: Vec<f64> = Vec::from([]);
        let result = kolmogorov_smirnov(&test_vec, eps);
    }
}

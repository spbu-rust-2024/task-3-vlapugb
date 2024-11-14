#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use arrayp::modules::calculations::modified_arifmetic_geometric_mean;

    #[test]
    fn test_modified_arifmetic_geometric_mean_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let eps = 0.0001;
        let result = modified_arifmetic_geometric_mean(&test_vec, &eps);
        assert!((result[0] - 2.7991).abs() < eps);
        assert!((result[1] - 2.7991).abs() < eps);
        assert!((result[2] + 8.3904).abs() < eps);
    }
    #[test]
    #[should_panic]
    fn test_modified_arifmetic_geometric_mean_invalid_accuracy_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = modified_arifmetic_geometric_mean(&test_vec, &1.1);
    }
    #[test]
    #[should_panic]
    fn test_modified_arifmetic_geometric_mean_invalid_accuracy_2() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = modified_arifmetic_geometric_mean(&test_vec, &-0.1);
    }
    #[test]
    #[should_panic]
    fn test_modified_arifmetic_geometric_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = modified_arifmetic_geometric_mean(&test_vec, &0.01);
    }
}

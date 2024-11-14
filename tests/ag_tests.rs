#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use arrayp::modules::calculations::arifmetic_geometric_mean;

    #[test]
    fn test_arifmetic_geometric_mean_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = arifmetic_geometric_mean(&test_vec, &0.01);
        assert_eq!(result, [2.802585542348676, 2.7956239471881865])
    }
    #[test]
    #[should_panic]
    fn test_arifmetic_geometric_mean_invalid_args_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = arifmetic_geometric_mean(&test_vec, &1.1);
    }
    #[test]
    #[should_panic]
    fn test_arifmetic_geometric_mean_invalid_args_2() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = arifmetic_geometric_mean(&test_vec, &-0.1);
    }
    #[test]
    #[should_panic]
    fn test_arifmetic_geometric_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = arifmetic_geometric_mean(&test_vec, &0.01);
    }
}

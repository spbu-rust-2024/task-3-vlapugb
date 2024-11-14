#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::trimmed_mean::trimmed_mean;
    #[test]
    fn test_trimmed_mean_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = trimmed_mean(&test_vec, 20);
        assert_eq!(result, 3.0);
    }

    #[test]
    #[should_panic]
    fn test_trimmed_mean_deleted_all_elements() {
        let test_vec: Vec<f64> = Vec::from([3.0]);
        let result = trimmed_mean(&test_vec, 100);
        assert_eq!(result, 0.0);
    }
    #[test]
    #[should_panic]
    fn test_trimmed_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = trimmed_mean(&test_vec, 100);
        assert_eq!(result, 0.0);
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use arrayp::modules::winsorized_mean::winsorized_mean;
    #[test]
    fn test_winsorized_mean_1() {
        let test_vec: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = winsorized_mean(&test_vec, 10);
        assert_eq!(result, 3.5);
    }

    #[test]
    #[should_panic]
    fn test_winsorized_mean_deleted_all_elements() {
        let test_vec: Vec<f64> = Vec::from([3.0]);
        let result = winsorized_mean(&test_vec, 100);
        assert_eq!(result, 0.0);
    }
    #[test]
    #[should_panic]
    fn test_winsorized_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = winsorized_mean(&test_vec, 100);
        assert_eq!(result, 0.0);
    }
}

pub fn median(vec: &Vec<f64>) -> f64 {
    if vec.is_empty() {
        panic!("Can't solve median!");
    }
    let mut c_vec = vec.clone();
    c_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if c_vec.len() % 2 != 0 {
        c_vec[c_vec.len() / 2]
    } else {
        (c_vec[c_vec.len() / 2] + c_vec[c_vec.len() / 2 - 1]) / 2.0
    }
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], 3.0)]
    #[case(&[1.0, 2.0, 3.0, 4.0], 2.5)]
    #[case(&[3.0, 1.0, 2.0], 2.0)]
    #[case(&[5.0, 5.0, 5.0], 5.0)]
    #[case(&[1.0], 1.0)]
    fn test_median_all_numbers_positive(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = median(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        );
    }
    #[test]
    #[should_panic]
    fn test_median_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = median(&test_vec);
    }
}

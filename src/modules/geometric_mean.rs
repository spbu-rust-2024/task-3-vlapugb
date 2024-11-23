pub fn geometric_mean(vec: &[f64]) -> f64 {
    let negative_elements = vec.iter().filter(|&&x| x < 0.0).count();
    if negative_elements % 2 != 0 {
        panic!("Value under sqrt is negative. Can't solve it!")
    }
    let product: f64 = vec.iter().product();
    f64::powf(product, 1.0 / vec.len() as f64)
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;

    use rstest::rstest;
    #[rstest]
    #[case(&[1.0], 1.0)]
    #[case(&[1.0, 1.0, 1.0], 1.0)]
    #[case(&[4.0, 8.0, 16.0], 8.0)]
    #[case(&[0.1, 0.2, 0.3], 0.181712059283214)]
    #[case(&[1.0, 2.0, 3.0, 4.0, 5.0], 2.605171084697352)]
    fn test_geometric_mean_all_numbers_positive(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = geometric_mean(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        );
    }

    #[test]
    fn test_geometric_mean_with_null() {
        let test_vec: Vec<f64> = Vec::from([0.0, 1.0, 41234.2]);
        let result = geometric_mean(&test_vec);
        assert_eq!(result, 0.0);
    }
    #[test]
    #[should_panic]
    fn test_geometric_mean_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = geometric_mean(&test_vec);
        assert_eq!(result, 0.0);
    }
    #[test]
    #[should_panic]
    fn test_geometric_mean_negative_sqrt() {
        let test_vec: Vec<f64> = Vec::from([-1.0, 1.0]);
        let result = geometric_mean(&test_vec);
        assert_eq!(result, 0.0);
    }
}

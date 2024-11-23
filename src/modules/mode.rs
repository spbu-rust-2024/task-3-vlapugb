use ordered_float::OrderedFloat;
use std::collections::HashMap;

pub fn mode(vec: &[f64]) -> f64 {
    if vec.is_empty() {
        panic!("Can't solve mode!");
    }
    let mut map = HashMap::new();

    for &num in vec.iter() {
        *map.entry(OrderedFloat(num)).or_insert(0) += 1;
    }
    let mut mode = 0.0;
    let mut max_count = 0;

    for (&num, &count) in map.iter() {
        if count > max_count {
            max_count = count;
            mode = *num;
        }
    }

    mode
}

#[cfg(test)]
#[allow(unused_variables)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(&[1.0, 2.0, 2.0, 3.0, 4.0], 2.0)]
    #[case(&[1.0, 1.0, 2.0, 3.0], 1.0)]
    #[case(&[3.0, 3.0, 3.0], 3.0)]
    fn test_mode(#[case] test_vec: &[f64], #[case] expected: f64) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let result = mode(&test_vec);
        assert_eq!(
            (result * 1e10).round() / 1e10,
            (expected * 1e10).round() / 1e10
        );
    }

    #[rstest]
    #[case(&[5.0, 1.0, 2.0, 3.0, 4.0], &[5.0,1.0,2.0,3.0,4.0])]
    #[case(&[1.0, 2.0, 3.0, 4.0], &[1.0,2.0,3.0,4.0])]
    fn test_mode_all_numbers_different(#[case] test_vec: &[f64], #[case] expected: &[f64]) {
        let test_vec: Vec<f64> = Vec::from(test_vec);
        let exp: Vec<f64> = Vec::from(expected);
        let result = mode(&test_vec);
        assert!(expected.contains(&result));
    }

    #[test]
    #[should_panic]
    fn test_mode_null_elements() {
        let test_vec: Vec<f64> = Vec::from([]);
        let result = mode(&test_vec);
    }
}

use super::arifmetic_mean::arifmetic_mean;

pub fn linear_deviation(vec: &[f64]) -> f64 {
    let a_mean = arifmetic_mean(vec);
    let linear_deviation = vec.iter().map(|&x| (x - a_mean).abs()).sum::<f64>() / vec.len() as f64;
    linear_deviation
}

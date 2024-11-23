use super::{arifmetic_mean::arifmetic_mean, linear_deviation::linear_deviation};

pub fn linear_coefficient_variations(vec: &[f64]) -> f64 {
    if arifmetic_mean(vec) == 0.0 {
        panic!("Can't solve linear Cv!");
    }
    linear_deviation(vec) / arifmetic_mean(vec)
}

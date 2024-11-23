use super::{arifmetic_mean::arifmetic_mean, standart_deviation::standart_deviation};

pub fn coefficient_variations(vec: &[f64]) -> f64 {
    if arifmetic_mean(vec) == 0.0 {
        panic!("Can't solve Cv!");
    }
    standart_deviation(vec) / arifmetic_mean(vec)
}

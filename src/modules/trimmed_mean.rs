use super::arifmetic_mean::arifmetic_mean;

pub fn trimmed_mean(vec: &Vec<f64>, k: usize) -> f64 {
    let mut trimmed_vec = vec.clone();
    let elements_to_trim = trimmed_vec.len() * k / 100;
    trimmed_vec.drain(0..elements_to_trim);
    trimmed_vec.drain((trimmed_vec.len() - elements_to_trim)..);
    arifmetic_mean(&trimmed_vec)
}

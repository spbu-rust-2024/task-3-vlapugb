use super::arifmetic_mean::arifmetic_mean;

pub fn dispertion(vec: &[f64]) -> f64 {
    if vec.len() < 2 {
        panic!("Can't solve dispertion!");
    }
    let a_mean = arifmetic_mean(vec);
    let dispertion = (vec
        .iter()
        .map(|&x| ((x - a_mean).abs()).powf(2.0))
        .sum::<f64>()
        / (vec.len() as f64 - 1.0))
        .sqrt();

    dispertion
}

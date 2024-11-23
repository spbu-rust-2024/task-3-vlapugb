use super::arifmetic_mean::arifmetic_mean;

pub fn standart_deviation(vec: &Vec<f64>) -> f64 {
    if vec.is_empty() {
        panic!("Can't solve standart deviation!");
    }
    let a_mean = arifmetic_mean(vec);
    let deviation = (vec
        .iter()
        .map(|&x| ((x - a_mean).abs()).powf(2.0))
        .sum::<f64>()
        / vec.len() as f64)
        .sqrt();

    deviation
}

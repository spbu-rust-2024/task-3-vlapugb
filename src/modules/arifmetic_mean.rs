use core::panic;

pub fn arifmetic_mean(vec: &Vec<f64>) -> f64 {
    if vec.len() == 0 {
        panic!("Zero elements vector! Can't solve arifmetic mean!");
    }
    let sum: f64 = vec.iter().sum();
    return sum / vec.len() as f64;
}

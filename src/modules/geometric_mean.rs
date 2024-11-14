pub fn geometric_mean(vec: &Vec<f64>) -> f64 {
    let negative_elements = vec.iter().filter(|&&x| x < 0.0).count();
    if negative_elements % 2 != 0 {
        panic!("Value under sqrt is negative. Can't solve it!")
    }
    let product: f64 = vec.iter().product();
    return f64::powf(product, 1.0 / vec.len() as f64);
}

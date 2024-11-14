pub fn power_mean(vec: &Vec<f64>, n: &u16) -> f64 {
    if vec.len() == 0 {
        panic!("Can't solve powered mean");
    }
    let powered: f64 = vec.iter().map(|&x| x.powf(*n as f64)).sum();
    println!("{}", powered);
    return (powered / vec.len() as f64).powf(1.0 / *n as f64);
}

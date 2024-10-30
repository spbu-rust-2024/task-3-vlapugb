use std::f64::consts::E;

fn f(x: f64) -> f64 {
    println!("{}", E.powf(x) + 2.0 * x * x + x + 5.0);
    E.powf(x) + 2.0 * x * x + x + 5.0
}

fn df(x: f64) -> f64 {
    println!("{}", E.powf(x) + 4.0 * x + 1.0);
    E.powf(x) + 4.0 * x + 1.0
}

fn newton_method(y: f64) -> f64 {
    let eps = 0.0000001;
    let mut it = 0;
    let mut start_x = 1.0;
    let mut next_x = start_x - (f(start_x) - y) / df(start_x);
    while (next_x - y).abs() > eps && it < 10000 {
        let f = f(start_x);
        let df = df(start_x);
        next_x = start_x - (f - y) / df;
        start_x = next_x;
        it += 1;
    }
    next_x
}

pub fn kolmogorov_smirnov(vec: &Vec<f64>) -> f64 {
    let x = vec
        .iter()
        .map(|&x| E.powf(x) + 2.0 * x * x + x + 5.0)
        .sum::<f64>()
        / vec.len() as f64;
    println!("x: {} ", &x);
    newton_method(x)
}

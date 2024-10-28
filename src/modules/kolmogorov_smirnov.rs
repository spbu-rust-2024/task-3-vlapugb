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
    let mut x0 = 1.0;
    let mut x1 = x0 - (f(x0) - y) / df(x0);
    while (x1 - y).abs() > eps && it < 10000 {
        let f = f(x0);
        let df = df(x0);
        x1 = x0 - (f - y) / df;
        x0 = x1;
        it += 1;
    }
    x1
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

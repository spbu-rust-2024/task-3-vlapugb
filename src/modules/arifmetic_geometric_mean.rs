use super::{arifmetic_mean::arifmetic_mean, geometric_mean::geometric_mean};

fn recursive_ag(eps: &f64, a_mean: &f64, g_mean: &f64) -> [f64; 2] {
    if (a_mean - g_mean).abs() < *eps {
        return [*a_mean, *g_mean];
    }
    let new_a_mean = (a_mean + g_mean) / 2.0;
    let new_g_mean = (a_mean + g_mean).sqrt();
    recursive_ag(eps, &new_a_mean, &new_g_mean)
}

pub fn arifmetic_geometric_mean(vec: &Vec<f64>, eps: &f64) -> [f64; 2] {
    let a_mean = arifmetic_mean(&vec);
    let g_mean = geometric_mean(&vec);
    let new_a;
    let new_b;
    [new_a, new_b] = recursive_ag(eps, &a_mean, &g_mean);
    return [new_a, new_b];
}

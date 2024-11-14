pub fn median(vec: &Vec<f64>) -> f64 {
    if vec.len() == 0 {
        panic!("Can't solve median!");
    }
    let mut c_vec = vec.clone();
    c_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if c_vec.len() % 2 != 0 {
        return c_vec[c_vec.len() / 2];
    } else {
        return (c_vec[c_vec.len() / 2] + c_vec[c_vec.len() / 2 - 1]) / 2.0;
    }
}

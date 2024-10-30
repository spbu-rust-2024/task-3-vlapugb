mod cli_style;
use crate::cli_style::print_info;
use std::io;
mod modules;
fn main() {
    let mut buf = String::new();
    println!("Enter your array elements: ");
    io::stdin()
        .read_line(&mut buf)
        .expect("Error reading input!");
    let vec: Vec<f64> = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid numbers"))
        .collect();

    print_info(vec);
}

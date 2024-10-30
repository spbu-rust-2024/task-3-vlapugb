use clap::{Arg, Command};
use std::io;

extern crate clap;

mod modules;
use modules::calculations::*;

fn main() {
    let matches = Command::new("Mean Calculator")
        .version("1.0")
        .author("vlapugb")
        .about("A comprehensive calculator for various types of means and statistical measures")
        .long_about(
            "This program calculates different types of means for a given set of numbers. \
		It supports arithmetic mean, geometric mean, power mean, arithmetic-geometric mean, \
		median, and mode calculations. Numbers should be input as space-separated values.",
        )
        .arg(
            Arg::new("arifmetic")
                .short('a')
                .long("arifmetic")
                .help("Calculate the arithmetic mean")
                .long_help(
                    "Calculates the arithmetic mean (average) of the input numbers. \
				Formula: (x₁ + x₂ + ... + xₙ) / n",
                ),
        )
        .arg(
            Arg::new("geometric")
                .short('g')
                .long("geometric")
                .help("Calculate the geometric mean")
                .long_help(
                    "Calculates the geometric mean of the input numbers. \
				Formula: ⁿ√(x₁ × x₂ × ... × xₙ)",
                ),
        )
        .arg(
            Arg::new("power")
                .short('p')
                .long("power")
                .help("Calculate the power mean")
                .long_help(
                    "Calculates the power mean (with power = 5) of the input numbers. \
				Formula: (1/n × (x₁ᵖ + x₂ᵖ + ... + xₙᵖ))^(1/p)",
                ),
        )
        .arg(
            Arg::new("arif_geom")
                .short('A')
                .long("arif_geom")
                .help("Calculate the arithmetic-geometric mean")
                .long_help(
                    "Calculates the arithmetic-geometric mean using an iterative process. \
				This is the limit of the sequence of arithmetic and geometric means.",
                ),
        )
        .arg(
            Arg::new("median")
                .long("median")
                .help("Calculate the median")
                .long_help(
                    "Calculates the median (middle value) of the sorted input numbers. \
				For odd number of values, returns the middle value. \
				For even number of values, returns the average of two middle values.",
                ),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .help("Calculate the mode")
                .long_help(
                    "Calculates the mode (most frequent value) of the input numbers. \
				If multiple values have the same highest frequency, returns the first one.",
                ),
        )
        .after_help(
            "EXAMPLES:\n\
		Calculate arithmetic mean:  mean-calculator -a\n\
		Calculate geometric mean:   mean-calculator -g\n\
		Calculate multiple means:   mean-calculator -a -g --median\n\
		\n\
		Input format: Enter space-separated numbers when prompted.\n\
		Example input: 1 2 3 4 5",
        )
        .get_matches();

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

    let eps = 0.00005;

    if matches.contains_id("arifmetic") {
        println!("Arifmetic: {:.3}", arifmetic_mean(&vec));
    }
    if matches.contains_id("geometric") {
        println!("Geometric: {:.3}", geometric_mean(&vec));
    }
    if matches.contains_id("power") {
        println!("Powered: {:.3}", power_mean(&vec, &5));
    }
    if matches.contains_id("arif_geom") {
        let ag_mean = arifmetic_geometric_mean(&vec, &eps);
        println!(
            "Arifmetic-Geometric, arifmetic: {:.10} ; geometric: {:.10}",
            ag_mean[0], ag_mean[1]
        );
    }
    if matches.contains_id("median") {
        println!("Median: {:.3}", median(&vec));
    }
    if matches.contains_id("mode") {
        println!("Mode: {:.3}", mode(&vec));
    }

    // Handle case where no valid options were provided.
    if !matches.contains_id("arifmetic")
        && !matches.contains_id("geometric")
        && !matches.contains_id("power")
        && !matches.contains_id("arif_geom")
        && !matches.contains_id("median")
        && !matches.contains_id("mode")
    {
        eprintln!("No valid option provided. Use --help for more information.");
    }
    println!("Cv: {:.3}", coefficient_variations(&vec));
    println!("LCv: {:.3}", linear_coefficient_variations(&vec));
    println!("trimmed: {:.3}", trimmed_mean(&vec, 20));
    println!("DSP: {:.3}", dispertion(&vec));
    println!("LDev: {:.3}", linear_deviation(&vec));
    println!(
        "MAGM: {:.3}\t{:.3}\t{:.3}",
        modified_arifmetic_geometric_mean(&vec, &eps)[0],
        modified_arifmetic_geometric_mean(&vec, &eps)[1],
        modified_arifmetic_geometric_mean(&vec, &eps)[2],
    );
    println!("LDev: {:.3}", winsorized_mean(&vec, 25));
    println!("KLGSMRV: {:.10}", kolmogorov_smirnov(&vec));
}

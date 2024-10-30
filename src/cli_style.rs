use crate::modules::{calculations::*, standart_deviation::standart_deviation};

fn count_decimal_places(epsilon: f64) -> usize {
    let epsilon_str = format!("{:.10}", epsilon);
    if let Some(pos) = epsilon_str.find('.') {
        epsilon_str[pos + 1..]
            .chars()
            .take_while(|&c| c == '0')
            .count()
            + 1
    } else {
        0
    }
}

use clap::Parser;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long = "a", help = "Calculates arifmetic mean\n")]
    pub a: bool,
    #[arg(long = "g", help = "Calculates geometric mean\n")]
    pub g: bool,
    #[arg(long = "p", help = "Calculates power mean\n")]
    pub p: bool,
    #[arg(
        long = "ag",
        help = "Calculates arifmetic-geometric mean. \nRequired parameter: 0.0 < accuracy < 1.0\n"
    )]
    pub ag: Option<f64>,
    #[arg(long = "median", help = "Calculates median of array\n")]
    pub median: bool,
    #[arg(long = "mode", help = "Calculates mode of array\n")]
    pub mode: bool,
    #[arg(long = "cv", help = "Calculates coefficient variation\n")]
    pub cv: bool,
    #[arg(long = "lcv", help = "Calculates linear coefficient variation\n")]
    pub lcv: bool,
    #[arg(long = "dsp", help = "Calculates dispertion\n")]
    pub dsp: bool,
    #[arg(
        long = "klg",
        help = "Calculates Kolmogorov-Smirnov mean.\nRequired parameter: 0.0 < accuracy < 1.0\n"
    )]
    pub klg: Option<f64>,
    #[arg(long = "dv", help = "Calculates deviation\n")]
    pub dv: bool,
    #[arg(long = "ldv", help = "Calculates linear deviation\n")]
    pub ldv: bool,
    #[arg(
        long = "mag",
        help = "Calculates modified arifmetic-geometric mean. \nRequired parameter: 0.0 < accuracy < 1.0\n"
    )]
    pub mag: Option<f64>,
    #[arg(
        long = "wn",
        help = "Calculates winsorized mean.  \nRequired parameter: percent of elements to replace (0-100)\n"
    )]
    pub wn: Option<u8>,
    #[arg(
        long = "tr",
        help = "Calculates trimmed mean. \nRequired parameter: percent of elements to trim (0-100)\n"
    )]
    pub tr: Option<u8>,
}

pub fn print_info(vec: Vec<f64>) {
    let args = Args::parse();
    if args.a {
        println!("Arifmetic mean: {:.3}\n", arifmetic_mean(&vec));
    }
    if args.g {
        println!("Geometric mean: {:.3}\n", geometric_mean(&vec));
    }
    if args.p {
        println!("Powered: {:.3}\n", power_mean(&vec, &5));
    }
    if let Some(ag_val) = args.ag {
        if ag_val > 1.0 || ag_val < 0.0 {
            panic!("Incorrect arifmetic-geometric argument. Check --help!");
        }
        println!(
            "A{}: {:.3}, B{}: {:.3}\n",
            ag_val,
            arifmetic_geometric_mean(&vec, &ag_val)[0],
            ag_val,
            arifmetic_geometric_mean(&vec, &args.ag.unwrap_or(0.0))[1]
        )
    }

    if args.median {
        println!("Median: {:.3}\n", median(&vec));
    }
    if args.mode {
        println!("Mode: {:.3}\n", mode(&vec));
    }
    if args.cv {
        println!("Cv: {:.3}\n", coefficient_variations(&vec));
    }
    if args.lcv {
        println!("Linear Cv: {:.3}\n", linear_coefficient_variations(&vec));
    }
    if args.dsp {
        println!("Dispertion: {:.3}\n", dispertion(&vec));
    }
    if let Some(ag_val) = args.klg {
        if ag_val > 1.0 || ag_val < 0.0 {
            panic!("Incorrect Kalmogorov-Smirnov argument. Check --help!");
        }
        let ks = kolmogorov_smirnov(&vec, ag_val);
        if ks.is_infinite() || ks.is_nan() {
            println!("Kolmogorov mean with eps: Can't calculate\n");
        } else {
            let precition = count_decimal_places(ag_val);
            println!("Kolmogorov mean with eps: {:.1$}\n", ks, precition);
        }
    }
    if args.dv {
        println!("Deviation: {:.3}\n", standart_deviation(&vec));
    }
    if args.ldv {
        println!("Deviation: {:.3}\n", linear_deviation(&vec));
    }
    if let Some(ag_val) = args.mag {
        if ag_val > 1.0 || ag_val < 0.0 {
            panic!("Incorrect modified arifmetic-geometric argument. Check --help!");
        }
        let precision = count_decimal_places(ag_val);
        let tmp_tuple = modified_arifmetic_geometric_mean(&vec, &ag_val);
        println!(
            "Modified arifmetic-geometric mean: A(eps) = {:.pr$}, B(eps) = {:.pr$}, Z(eps) = {:.pr$}\n",
            tmp_tuple[0], tmp_tuple[1], tmp_tuple[2], pr = precision
        );
    }
    if let Some(ag_val) = args.wn {
        if ag_val > 100 {
            panic!("Incorrect winsorized argument. Check --help!");
        }
        println!("Winsorized mean: {:.3}\n", winsorized_mean(&vec, ag_val));
    }
    if let Some(ag_val) = args.tr {
        if ag_val > 100 {
            panic!("Incorrect trimmed argument. Check --help!");
        }
        println!("Trimmed mean: {:.3}\n", trimmed_mean(&vec, ag_val));
    }
}

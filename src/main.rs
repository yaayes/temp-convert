use std::{cmp::Ordering, env::args};

fn main() {
    let args: Vec<String> = args().collect();
    let total_args = args.len();

    let result: f64 = match total_args.cmp(&2) {
        Ordering::Equal => {
            if args[1] == "-h" {
                show_help();
            } else if args[1] == "-c" || args[1] == "--celsius" {
                println!("Please enter the temp in Celsius")
            } else if args[1] == "-f" || args[1] == "--fahrenheit" {
                println!("Please enter the temp in Fahrenheit")
            } else {
                println!("Invalid options. -h to show help");
            }

            0.0
        }
        Ordering::Less => {
            show_help();
            0.0
        }
        Ordering::Greater => {
            let initial_temp = &args[1];
            let value: f64 = args[2].trim().parse().expect("Please type a number");

            println!("{initial_temp} {value}");

            if initial_temp == "-c" || initial_temp == "--celsius" {
                convert_from_celesius_to_fahrenheit(&value)
            } else if initial_temp == "-f" || initial_temp == "--fahrenheit" {
                convert_from_fahrenheit_to_celesius(&value)
            } else {
                println!("Invalid option. use -h for help");
                0.0
            }
        }
    };

    println!("{result}");
}

fn convert_from_celesius_to_fahrenheit(value_in_celesius: &f64) -> f64 {
    (value_in_celesius * 1.8) + 32.0
}

fn convert_from_fahrenheit_to_celesius(value_in_fahrenheit: &f64) -> f64 {
    (value_in_fahrenheit - 32.0) / 1.8
}

fn show_help() {
    println!(
        "
    Temperature convert

    Usage:
      -h                Show this page
      -c --celsius      Convert a Celsius temperature to Fahrenheit
      -f --fahrenheit   Convert a Fahrenheit temperature to Celsius
    "
    );
}

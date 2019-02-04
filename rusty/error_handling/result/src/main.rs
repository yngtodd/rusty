extern crate clap;

use clap::{Arg, App};

fn main() {
    let args = App::new("Division by int.")
        .about("Simple program to test error handling")
        .arg(Arg::with_name("num")
                 .short("n")
                 .long("number")
                 .takes_value(true)
                 .help("Single integer, please."))
        .get_matches();

    let num_str = args.value_of("num");
    match num_str {
        None => println!("Need an integer to divide by."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => {
                    match divide_ten_by(n) {
                        Ok(result) => println!("Result = {}", result),
                        Err(e) => println!("ERROR: {}", e)
                    };
                },
                Err(_) => {
                    println!("That's not a number! {}", s)
                },
            }
        }
    }
}

fn divide_ten_by(i: i32) -> Result<i32, String> {
    match i {
        i if i != 0 => Ok(10 / i),
        _ => Err(format!("Division by zero! i is {}", i))
    }
}
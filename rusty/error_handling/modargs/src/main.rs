#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();
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
                    println!("That's not an integer! {}", s)
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
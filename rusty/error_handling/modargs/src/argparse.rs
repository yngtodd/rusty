extern crate clap;

use clap::{Arg, App};

pub fn get_args() -> App {
    let args = App::new("Division by int.")
        .about("Simple program to test error handling")
        .arg(Arg::with_name("num")
                 .short("n")
                 .long("number")
                 .takes_value(true)
                 .help("Integer to divide 10 by."))
        .get_matches();

    return args;
}
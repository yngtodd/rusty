#[macro_use]
extern crate clap;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches();
    let num = value_t!(args.value_of("num"), i32).unwrap_or_else(|e| e.exit());

    match divide_ten_by(num) {
        Ok(result) => println!("Result = {}", result),
        Err(e) => println!("ERROR: {}", e)
    };
}

fn divide_ten_by(i: i32) -> Result<i32, String> {
    match i {
        i if i != 0 => Ok(10 / i),
        _ => Err(format!("Division by zero! i is {}", i))
    }
}
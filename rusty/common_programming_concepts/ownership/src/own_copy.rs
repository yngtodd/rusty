pub fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is *dropped*.

pub fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens.

fn main() {
    // looking at the heap with String.
    let mut s = String::from("Hello");
    s.push_str(", world!");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and is *dropped*.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope, nothing special happens.

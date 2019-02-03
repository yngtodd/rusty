mod own_copy;

fn main() {
    // looking at the heap with String.
    let mut s = String::from("Hello");
    s.push_str(", world!");
    own_copy::takes_ownership(s);

    let x = 5;
    own_copy::makes_copy(x);
}
fn main() {
    // mathematical keywords
    let sum = 5 + 10;
    let product = 2 * 3;
    println!("Sum: {}, Product: {}", sum, product);

    // boolean
    let t = true;
    if t == true {
        println!("t is {}", t);
    }

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Zeroth index of tuple: {}", tup.0);

    let arry: [i32; 5] = [10,11,12,13,14];
    for (i, elem) in arry.iter().enumerate() {
        println!("arry[{}]: {}", i, elem);
    }

    let idx = 0;
    let element = arry[idx];
    println!("Element at index 0 of arry = {}", element);
}

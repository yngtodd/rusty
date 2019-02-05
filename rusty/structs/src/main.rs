mod counter;
mod rectangle;

fn main() {
    let rect1 = rectangle::Rectangle { width: 30, height: 50 };
    let sq = rectangle::Rectangle::square(3);

    let args = counter::Args { init: 0, by: 1 };
    let mut counter = counter::Counter::new(args);

    println!("\n<< Counter Struct >>\n");

    for i in 0..5 {
        counter.print();
        counter.incr();
        if i == 4 {
            counter.decr();
            counter.print();
        }
    }

    let x = counter.get();
    println!("Counter currently holds {}\n", x);

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!(
        "The perimeter of the rectangle is {} pixels\n",
        rect1.perimeter()
    );

    println!(
        "The area of the square is {} square pixels",
        sq.area()
    );

    println!(
        "The perimeter of the square is {} pixels",
        sq.perimeter()
    );
}
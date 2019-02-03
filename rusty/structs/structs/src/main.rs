fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let sq = Rectangle::square(3);

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

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        return (2 * self.width) + (2 * self.height)
    }

    // associated function (no self) - syntax Rectangle::square(u32);
    fn square(size: u32) -> Rectangle {
        return Rectangle { width: size, height: size }
    }
}
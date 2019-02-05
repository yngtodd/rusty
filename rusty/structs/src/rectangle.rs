pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        return self.width * self.height
    }

    pub fn perimeter(&self) -> u32 {
        return (2 * self.width) + (2 * self.height)
    }

    // associated function (no self) - syntax Rectangle::square(u32);
    pub fn square(size: u32) -> Rectangle {
        return Rectangle { width: size, height: size }
    }
}
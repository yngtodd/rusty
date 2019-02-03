pub struct Args {
    pub init: u32,
    pub by: u32
}

pub struct Counter {
    pub val: u32,
    pub by: u32
}

impl Counter {
    pub fn new(args: Args) -> Counter {
        Counter {
            val: args.init,
            by: args.by
        }
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn print(&self) {
        println!("Current value: {}", self.val)
    }

    pub fn incr(&mut self) {
        self.val += self.by;
    }

    pub fn decr(&mut self) {
        self.val -= self.by;
    }
}
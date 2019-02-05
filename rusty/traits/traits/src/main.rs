mod tweet;

use tweet::Summarizable;

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);

    println!("\n<<Tweeting>>\n");

    let tweet = tweet::Tweet {
        username: String::from("yngtodd"),
        content: String::from("I love Studio Ghibli, obviously.")
    };

    println!("1 new tweet - {}", tweet.summary());
}
// show four-byte signed 42
// show eight-byte float 3.14

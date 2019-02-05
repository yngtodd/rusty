pub struct Tweet {
    pub username: String,
    pub content: String
}

pub trait Summarizable {
    fn summary(&self) -> String;
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
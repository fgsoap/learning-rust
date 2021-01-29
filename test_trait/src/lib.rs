use std::fmt::{self, Display};

pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("(Read more about author...)")
    }
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
#[derive(Debug, Clone)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
#[derive(Debug, Clone)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} + {} + {} + {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

pub fn notify(item: impl Summary + Display) {
    println!("Breaking news! notify: {}", item);
}

pub fn notify1<T: Summary + Display>(item: T) {
    println!("Breaking news! notify1: {}", item);
}

pub fn notify2(item1: impl Summary, item2: impl Summary) {
    println!(
        "Breaking news! notify2: {:?} + {:?}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify3<T: Summary>(item1: T, item2: T) {
    println!(
        "Breaking news! notify3: {:?} + {:?}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify4<T, U>(t: T, u: U)
where
    T: Display + Summary,
    U: Clone + Summary,
{
    println!("Notify4... {} | {}", t.summarize(), u.summarize());
}

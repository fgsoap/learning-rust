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

pub fn notify(item: impl Summary) {
    println!("Breaking news! {:?}", item.summarize());
}

pub fn notify1<T: Summary>(item: T) {
    println!("Breaking news! {:?}", item.summarize());
}

pub fn notify2(item1: impl Summary, item2: impl Summary) {
    println!(
        "Breaking news! {:?} + {:?}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify3<T: Summary>(item1: T, item2: T) {
    println!(
        "Breaking news! {:?} + {:?}",
        item1.summarize(),
        item2.summarize()
    );
}

use test_trait::notify;
use test_trait::notify1;
use test_trait::notify2;
use test_trait::notify3;
use test_trait::NewsArticle;
use test_trait::Summary;
use test_trait::Tweet;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course,  as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let newsarticle = NewsArticle {
        headline: String::from("Test Headline"),
        location: String::from("Test Location"),
        author: String::from("Test Author"),
        content: String::from("Test COntent"),
    };
    println!(
        "1 new tweet: {}, author is: {}",
        tweet.summarize(),
        tweet.summarize_author()
    );
    notify(tweet.clone());
    notify1(tweet.clone());
    notify2(tweet.clone(), newsarticle.clone());
    notify3(newsarticle.clone(), newsarticle.clone())
}

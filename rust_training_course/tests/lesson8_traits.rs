use rust_training_course::lesson8_traits::*;

#[test]
fn test_news_article_summary() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again have won the Stanley Cup.",
        ),
    };

    assert_eq!(
        article.summary(),
        "Penguins win the Stanley Cup!, by Iceburgh (Pittsburgh, PA)"
    );
}

#[test]
fn test_tweet_summary() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    assert_eq!(
        tweet.summary(),
        "horse_ebooks: of course, as you probably already know, people"
    );
}

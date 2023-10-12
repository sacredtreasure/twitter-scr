use chrono::{Duration, Utc};
use twapi::{TwitterAPI, TweetSearchQuery, TweetSort};

#[tokio::main]
async fn main() -> Result<(), twapi::Error> {
    // Initialize the TwitterAPI client using your API keys and access tokens
    let api = TwitterAPI::new_from_env();

    // Replace with your desired hashtag
    let hashtag = "KAVA";

    // Construct a search query for tweets with the hashtag and sort by newest
    let query = TweetSearchQuery::new()
        .query(format!("hashtag:{} -filter:retweets", hashtag))
        .sort(TweetSort::Newest);

    // Fetch tweets matching the query
    let tweets = api.search_tweets(&query).await?;

    // Get the current time
    let current_time = Utc::now();

    // Create a vector to store tweets with calculated ages
    let mut tweets_with_age: Vec<(String, Duration)> = vec![];

    // Calculate the age for each tweet
    for tweet in tweets {
        let created_at = tweet.created_at.with_timezone(&Utc);
        let age = current_time.signed_duration_since(created_at);
        tweets_with_age.push((tweet.full_text, age));
    }

    // Sort the tweets by age (oldest first)
    tweets_with_age.sort_by(|(_, age1), (_, age2)| age1.cmp(age2));

    // Print the sorted tweets with their ages
    for (text, age) in tweets_with_age {
        println!("Age: {:?}, Tweet: {}", age, text);
    }

    Ok(())
}

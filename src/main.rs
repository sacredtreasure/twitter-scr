use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use error_chain::error_chain;
use reqwest;
use select::document::Document;
use select::predicate::Name;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct TwitterPost {
    text: String,
    timestamp: NaiveDateTime,
}

impl Ord for TwitterPost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for TwitterPost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TwitterPost {}

impl PartialEq for TwitterPost {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let hashtag = "$KAVA"; // Replace with the desired hashtag

    let res = reqwest::get(&format!("https://twitter.com/hashtag/{}?src=hash", hashtag))
        .await?
        .text()
        .await?;

    let mut post_heap: BinaryHeap<TwitterPost> = BinaryHeap::new();

    Document::from(res.as_str())
        .find(Name("li"))
        .filter(|node| {
            node.attr("data-item-type") == Some("tweet") && node.attr("data-name") == Some("tweet")
        })
        .for_each(|tweet| {
            if let Some(text) = tweet
                .find(Name("p"))
                .filter_map(|p| Some(p.text().trim().to_string()))
                .next()
            {
                if let Some(timestamp) = tweet
                    .find(Name("span"))
                    .filter_map(|n| {
                        if n.attr("data-time") == None {
                            return None;
                        }
                        Some(n.attr("data-time").unwrap().parse::<i64>().unwrap())
                    })
                    .next()
                {
                    let timestamp_utc = NaiveDateTime::from_timestamp(timestamp, 0);
                    post_heap.push(TwitterPost {
                        text,
                        timestamp: timestamp_utc,
                    });
                }
            }
        });

    let current_datetime = Utc::now();

    while let Some(post) = post_heap.pop() {
        let timestamp_utc: DateTime<Utc> = DateTime::from_utc(post.timestamp, Utc);
        let age = current_datetime.signed_duration_since(timestamp_utc);
        println!("Age: {:?}, Tweet: {}", age, post.text);
    }

    Ok(())
}

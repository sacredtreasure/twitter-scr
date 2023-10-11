use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::collections::BinaryHeap;
use std::time::Duration;
use select::document::Document;
use select::predicate::Name;
use chrono::NaiveDateTime;
use error_chain::error_chain;

#[derive(Debug)]
struct TwitterPost {
    text: String,
    timestamp: NaiveDateTime,
}

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let hashtag = "$KAVA"; // Replace with the desired hashtag

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(30))
        .build()?;

    let res = client
        .get(&format!("https://twitter.com/hashtag/{}?src=hash", hashtag))
        .send()
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
                    post_heap.push(TwitterPost { text, timestamp: timestamp_utc });
                }
            }
        });

    while let Some(post) = post_heap.pop() {
        let age = Utc::now().signed_duration_since(post.timestamp);
        println!("Age: {:?}, Tweet: {}", age, post.text);
    }

    Ok(())
}

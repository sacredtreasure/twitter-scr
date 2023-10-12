use serde::Deserialize;
use reqwest;
use serde_json::Value;

#[derive(Deserialize)]
struct YouTubeVideo {
    items: Vec<YouTubeVideoItem>,
}

#[derive(Deserialize)]
struct YouTubeVideoItem {
    id: Value,
    snippet: Value,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Set your YouTube Data API key
    let api_key = "YOUR_YOUTUBE_API_KEY";

    // Set the hashtag you want to search for
    let hashtag = "YOUR_HASHTAG";

    // Create a search query to search for videos with the hashtag
    let search_query = format!("{} video", hashtag);

    // Construct the YouTube API URL
    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?key={}&q={}&part=snippet&type=video",
        api_key, search_query
    );

    // Send a GET request to the YouTube Data API
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(reqwest::Error::new(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", response.status()),
        ));
    }

    // Parse the JSON response
    let response_text = response.text().await?;
    let parsed_response: Result<Value, serde_json::Error> = serde_json::from_str(&response_text)?;

    // Process and print the video data
    if let Some(items) = parsed_response.get("items") {
        for item in items.as_array().unwrap() {
            if let Some(snippet) = item.get("snippet") {
                if let Some(title) = snippet.get("title") {
                    println!("Video Title: {}", title);
                }
                if let Some(description) = snippet.get("description") {
                    println!("Video Description: {}", description);
                }
            }
            if let Some(id) = item.get("id") {
                println!("Video ID: {:?}", id);
            }
            println!("------------------------");
        }
    }

    Ok(())
}

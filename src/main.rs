use web_scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://scrapeme.live/shop/")
        .await?
        .text()
        .await;

    if let Ok(response) = resp {
        web_scraper::parse_document(response).await;
    }

    Ok(())
}

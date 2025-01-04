use std::error::Error;
mod rss;
use rss::{fetch_rss, get_first_item};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rss_url = "https://rsshub.app/rsshub/routes";

    let channel = fetch_rss(rss_url).await.unwrap();
    println!("{:?}", get_first_item(channel).await?);

    Ok(())
}

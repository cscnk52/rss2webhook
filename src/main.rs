use std::error::Error;
mod rss;
use rss::get_first_item;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rss_url = "https://rsshub.app/rsshub/routes";

    println!("{:?}", get_first_item(rss_url).await?);

    Ok(())
}
